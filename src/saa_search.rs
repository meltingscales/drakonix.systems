// Full-text search over the SAA Basic Text, ported from
// github.com/HenryFBP/SAAFullTextSearch. Same design as aa_search.rs, minus
// the book dimension — this is one book, so pages aren't namespaced.
use serde::Serialize;
use std::sync::Arc;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, TantivyDocument, Value, FAST, STORED, TEXT};
use tantivy::snippet::SnippetGenerator;
use tantivy::{doc, Index, IndexReader};

struct SaaPage {
    page_num: u32,
    text: String,
}

#[derive(Serialize)]
pub struct SaaHit {
    pub page_num: u32,
    pub snippet: String,
}

#[derive(Clone)]
pub struct SaaSearchManager {
    pages: Arc<Vec<SaaPage>>,
    index: Index,
    reader: IndexReader,
    page_num_field: Field,
    text_field: Field,
}

impl SaaSearchManager {
    pub fn new(pages_dir: &str) -> anyhow::Result<Self> {
        let pages = load_pages(pages_dir);

        let mut schema_builder = Schema::builder();
        let page_num_field = schema_builder.add_u64_field("page_num", STORED | FAST);
        let text_field = schema_builder.add_text_field("text", TEXT | STORED);
        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema);
        let mut writer: tantivy::IndexWriter = index.writer(50_000_000)?;
        for page in &pages {
            writer.add_document(doc!(
                page_num_field => page.page_num as u64,
                text_field => page.text.clone(),
            ))?;
        }
        writer.commit()?;
        let reader = index.reader()?;

        Ok(Self {
            pages: Arc::new(pages),
            index,
            reader,
            page_num_field,
            text_field,
        })
    }

    pub fn search(&self, q: &str) -> Vec<SaaHit> {
        if q.trim().is_empty() {
            return Vec::new();
        }
        let searcher = self.reader.searcher();
        let query_parser = QueryParser::for_index(&self.index, vec![self.text_field]);
        let query = query_parser
            .parse_query(q)
            .or_else(|_| query_parser.parse_query(&format!("\"{}\"", q.replace('"', "'"))));
        let Ok(query) = query else {
            return Vec::new();
        };

        let Ok(top_docs) = searcher.search(&query, &TopDocs::with_limit(50).order_by_score()) else {
            return Vec::new();
        };
        let snippet_generator = SnippetGenerator::create(&searcher, &*query, self.text_field).ok();

        let mut hits = Vec::new();
        for (_score, doc_address) in top_docs {
            let Ok(doc) = searcher.doc::<TantivyDocument>(doc_address) else {
                continue;
            };
            let page_num = doc
                .get_first(self.page_num_field)
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as u32;
            let snippet = snippet_generator
                .as_ref()
                .map(|g| g.snippet_from_doc(&doc).fragment().to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| {
                    doc.get_first(self.text_field)
                        .and_then(|v| v.as_str())
                        .map(|t| t.chars().take(400).collect())
                        .unwrap_or_default()
                });
            hits.push(SaaHit { page_num, snippet });
        }
        hits
    }

    pub fn get_page(&self, page_num: u32) -> Option<SaaHit> {
        self.pages
            .iter()
            .find(|p| p.page_num == page_num)
            .map(|p| SaaHit { page_num: p.page_num, snippet: p.text.clone() })
    }
}

// Each page is one Markdown file: <pages_dir>/<page_num>.md
fn load_pages(pages_dir: &str) -> Vec<SaaPage> {
    let mut pages = Vec::new();
    for entry in std::fs::read_dir(pages_dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let Some(page_num) = path
            .file_stem()
            .and_then(|s| s.to_str())
            .and_then(|s| s.parse().ok())
        else {
            continue;
        };
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        pages.push(SaaPage { page_num, text });
    }
    pages
}
