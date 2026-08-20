// Full-text search over the AA Big Book and 12&12, ported from
// github.com/HenryFBP/AAFullTextSearch. Pages are pre-extracted Markdown
// files (one per real printed page) checked into aa-pages/<book>/<n>.md;
// the tantivy index is built in-memory at startup.
use serde::Serialize;
use std::sync::Arc;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Field, Schema, TantivyDocument, Value, FAST, STORED, STRING, TEXT};
use tantivy::snippet::SnippetGenerator;
use tantivy::{doc, Index, IndexReader};

struct AaPage {
    book: String,
    page_num: u32,
    text: String,
}

#[derive(Serialize)]
pub struct AaHit {
    pub book: String,
    pub page_num: u32,
    pub snippet: String,
}

#[derive(Clone)]
pub struct AaSearchManager {
    pages: Arc<Vec<AaPage>>,
    index: Index,
    reader: IndexReader,
    book_field: Field,
    page_num_field: Field,
    text_field: Field,
}

impl AaSearchManager {
    pub fn new(pages_dir: &str) -> anyhow::Result<Self> {
        let pages = load_pages(pages_dir);

        let mut schema_builder = Schema::builder();
        let book_field = schema_builder.add_text_field("book", STRING | STORED);
        let page_num_field = schema_builder.add_u64_field("page_num", STORED | FAST);
        let text_field = schema_builder.add_text_field("text", TEXT | STORED);
        let schema = schema_builder.build();

        let index = Index::create_in_ram(schema);
        let mut writer: tantivy::IndexWriter = index.writer(50_000_000)?;
        for page in &pages {
            writer.add_document(doc!(
                book_field => page.book.clone(),
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
            book_field,
            page_num_field,
            text_field,
        })
    }

    pub fn search(&self, q: &str) -> Vec<AaHit> {
        if q.trim().is_empty() {
            return Vec::new();
        }
        let searcher = self.reader.searcher();
        let query_parser = QueryParser::for_index(&self.index, vec![self.text_field]);
        // Fall back to a literal phrase if it doesn't parse as query syntax
        // (e.g. stray quotes or operators typed as plain words).
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
            let book = doc
                .get_first(self.book_field)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
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
            hits.push(AaHit { book, page_num, snippet });
        }
        hits
    }

    pub fn get_page(&self, book: &str, page_num: u32) -> Option<AaHit> {
        self.pages
            .iter()
            .find(|p| p.book == book && p.page_num == page_num)
            .map(|p| AaHit {
                book: p.book.clone(),
                page_num: p.page_num,
                snippet: p.text.clone(),
            })
    }
}

// Each page is one Markdown file: <pages_dir>/<book>/<page_num>.md
fn load_pages(pages_dir: &str) -> Vec<AaPage> {
    let mut pages = Vec::new();
    for book_dir in std::fs::read_dir(pages_dir).into_iter().flatten().flatten() {
        let book = book_dir.file_name().to_string_lossy().into_owned();
        for entry in std::fs::read_dir(book_dir.path()).into_iter().flatten().flatten() {
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
            pages.push(AaPage { book: book.clone(), page_num, text });
        }
    }
    pages
}
