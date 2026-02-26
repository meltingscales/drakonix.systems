use crate::models::{Page, Post};
use anyhow::{Context, Result};
use pulldown_cmark::{html, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

#[derive(Debug, Clone)]
struct TocEntry {
    level: u8,
    text: String,
    id: String,
}

enum FrontmatterFormat {
    Yaml,
    Toml,
}

pub struct MarkdownProcessor {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl Default for MarkdownProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl MarkdownProcessor {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    /// Parse markdown with syntax highlighting
    pub fn parse(&self, markdown: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(markdown, options);
        let mut in_code_block = false;
        let mut code_block_lang = String::new();
        let mut code_block_content = String::new();

        let events: Vec<Event> = parser
            .into_iter()
            .flat_map(|event| match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                    in_code_block = true;
                    code_block_lang = lang.to_string();
                    code_block_content.clear();
                    vec![]
                }
                Event::End(TagEnd::CodeBlock) if in_code_block => {
                    in_code_block = false;
                    let highlighted = self.highlight_code(&code_block_content, &code_block_lang);
                    vec![Event::Html(highlighted.into())]
                }
                Event::Text(text) if in_code_block => {
                    code_block_content.push_str(&text);
                    vec![]
                }
                _ => vec![event],
            })
            .collect();

        let mut html_output = String::new();
        html::push_html(&mut html_output, events.into_iter());
        html_output
    }

    /// Highlight code using syntect
    fn highlight_code(&self, code: &str, lang: &str) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_token(lang)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"];

        highlighted_html_for_string(code, &self.syntax_set, syntax, theme).unwrap_or_else(|_| {
            format!("<pre><code>{}</code></pre>", html_escape::encode_text(code))
        })
    }

    /// Generate a slug from text for use as an ID
    fn generate_id(text: &str) -> String {
        text.to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join("-")
    }

    /// Extract table of contents from markdown
    fn extract_toc(&self, markdown: &str) -> Vec<TocEntry> {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(markdown, options);
        let mut toc_entries = Vec::new();
        let mut current_heading_level: Option<HeadingLevel> = None;
        let mut current_heading_text = String::new();

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    current_heading_level = Some(level);
                    current_heading_text.clear();
                }
                Event::End(TagEnd::Heading(_)) => {
                    if let Some(level) = current_heading_level {
                        let level_num = match level {
                            HeadingLevel::H1 => 1,
                            HeadingLevel::H2 => 2,
                            HeadingLevel::H3 => 3,
                            HeadingLevel::H4 => 4,
                            HeadingLevel::H5 => 5,
                            HeadingLevel::H6 => 6,
                        };

                        let id = Self::generate_id(&current_heading_text);
                        toc_entries.push(TocEntry {
                            level: level_num,
                            text: current_heading_text.clone(),
                            id,
                        });
                    }
                    current_heading_level = None;
                }
                Event::Text(text) if current_heading_level.is_some() => {
                    current_heading_text.push_str(&text);
                }
                Event::Code(code) if current_heading_level.is_some() => {
                    current_heading_text.push_str(&code);
                }
                _ => {}
            }
        }

        toc_entries
    }

    /// Generate HTML for table of contents
    fn generate_toc_html(&self, toc_entries: &[TocEntry]) -> String {
        if toc_entries.is_empty() {
            return String::new();
        }

        let mut html = String::from("<details class=\"toc\" open>\n<summary>Table of Contents</summary>\n<nav>\n<ul>\n");
        let mut current_level = 1u8;

        for entry in toc_entries {
            // Close nested lists as needed
            while current_level > entry.level {
                html.push_str("</li>\n</ul>\n");
                current_level -= 1;
            }

            // Close previous <li> at same level
            if current_level == entry.level && current_level > 0 {
                html.push_str("</li>\n");
            }

            // Open nested lists as needed
            while current_level < entry.level {
                if current_level > 0 {
                    html.push_str("\n<ul>\n");
                }
                current_level += 1;
            }

            html.push_str(&format!(
                "<li><a href=\"#{}\">{}</a>",
                html_escape::encode_text(&entry.id),
                html_escape::encode_text(&entry.text)
            ));
        }

        // Close all remaining open lists
        while current_level > 0 {
            html.push_str("</li>\n");
            if current_level > 1 {
                html.push_str("</ul>\n");
            }
            current_level -= 1;
        }

        html.push_str("</ul>\n</nav>\n</details>\n");
        html
    }

    /// Parse markdown with syntax highlighting and add IDs to headings for TOC
    pub fn parse_with_heading_ids(&self, markdown: &str) -> String {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(markdown, options);
        let mut in_code_block = false;
        let mut code_block_lang = String::new();
        let mut code_block_content = String::new();
        let mut current_heading_level: Option<HeadingLevel> = None;
        let mut current_heading_text = String::new();

        let events: Vec<Event> = parser
            .into_iter()
            .flat_map(|event| match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                    in_code_block = true;
                    code_block_lang = lang.to_string();
                    code_block_content.clear();
                    vec![]
                }
                Event::End(TagEnd::CodeBlock) if in_code_block => {
                    in_code_block = false;
                    let highlighted = self.highlight_code(&code_block_content, &code_block_lang);
                    vec![Event::Html(highlighted.into())]
                }
                Event::Text(text) if in_code_block => {
                    code_block_content.push_str(&text);
                    vec![]
                }
                Event::Start(Tag::Heading { level, .. }) => {
                    current_heading_level = Some(level);
                    current_heading_text.clear();
                    vec![]
                }
                Event::End(TagEnd::Heading(level)) => {
                    let id = Self::generate_id(&current_heading_text);
                    let level_tag = match level {
                        HeadingLevel::H1 => "h1",
                        HeadingLevel::H2 => "h2",
                        HeadingLevel::H3 => "h3",
                        HeadingLevel::H4 => "h4",
                        HeadingLevel::H5 => "h5",
                        HeadingLevel::H6 => "h6",
                    };

                    current_heading_level = None;
                    vec![Event::Html(
                        format!(
                            "<{} id=\"{}\">{}</{}>",
                            level_tag,
                            html_escape::encode_text(&id),
                            html_escape::encode_text(&current_heading_text),
                            level_tag
                        )
                        .into(),
                    )]
                }
                Event::Text(text) if current_heading_level.is_some() => {
                    current_heading_text.push_str(&text);
                    vec![]
                }
                Event::Code(code) if current_heading_level.is_some() => {
                    current_heading_text.push_str(&code);
                    vec![]
                }
                _ => vec![event],
            })
            .collect();

        let mut html_output = String::new();
        html::push_html(&mut html_output, events.into_iter());
        html_output
    }

    /// Load and parse a blog post from a markdown file
    pub fn load_post(&self, path: &Path) -> Result<Post> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        let (frontmatter, body, format) = self.split_frontmatter(&content)?;
        let mut post: Post = match format {
            FrontmatterFormat::Yaml => serde_yaml::from_str(&frontmatter).with_context(|| {
                format!(
                    "Failed to parse YAML frontmatter. Content:\n{}",
                    frontmatter
                )
            })?,
            FrontmatterFormat::Toml => toml::from_str(&frontmatter).with_context(|| {
                format!(
                    "Failed to parse TOML frontmatter. Content:\n{}",
                    frontmatter
                )
            })?,
        };

        post.content = body.to_string();

        // Generate TOC if requested
        if post.toc {
            let toc_entries = self.extract_toc(&body);
            post.toc_html = self.generate_toc_html(&toc_entries);
            post.html = self.parse_with_heading_ids(&body);
        } else {
            post.html = self.parse(&body);
        }

        post.slug = self.extract_slug(path)?;
        post.url = format!("/posts/{}", post.slug);
        post.file_path = path.to_path_buf();

        Ok(post)
    }

    /// Load and parse a page from a markdown file
    pub fn load_page(&self, path: &Path) -> Result<Page> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        let (frontmatter, body, format) = self.split_frontmatter(&content)?;
        let mut page: Page = match format {
            FrontmatterFormat::Yaml => serde_yaml::from_str(&frontmatter)
                .with_context(|| "Failed to parse YAML frontmatter")?,
            FrontmatterFormat::Toml => {
                toml::from_str(&frontmatter).with_context(|| "Failed to parse TOML frontmatter")?
            }
        };

        page.content = body.to_string();

        // Generate TOC if requested
        if page.toc {
            let toc_entries = self.extract_toc(&body);
            page.toc_html = self.generate_toc_html(&toc_entries);
            page.html = self.parse_with_heading_ids(&body);
        } else {
            page.html = self.parse(&body);
        }

        page.slug = self.extract_slug(path)?;
        page.url = format!("/pages/{}", page.slug);
        page.file_path = path.to_path_buf();

        Ok(page)
    }

    /// Split frontmatter from markdown content (supports both YAML and TOML)
    fn split_frontmatter<'a>(
        &self,
        content: &'a str,
    ) -> Result<(String, String, FrontmatterFormat)> {
        // Try YAML format first (---)
        let yaml_re = Regex::new(r"(?s)^---\s*\n(.*?)\n---\s*\n(.*)$").unwrap();
        if let Some(caps) = yaml_re.captures(content) {
            let frontmatter = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let body = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            return Ok((
                frontmatter.to_string(),
                body.to_string(),
                FrontmatterFormat::Yaml,
            ));
        }

        // Try TOML format (+++)
        let toml_re = Regex::new(r"(?s)^\+\+\+\s*\n(.*?)\n\+\+\+\s*\n(.*)$").unwrap();
        if let Some(caps) = toml_re.captures(content) {
            let frontmatter = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let body = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            return Ok((
                frontmatter.to_string(),
                body.to_string(),
                FrontmatterFormat::Toml,
            ));
        }

        anyhow::bail!("No frontmatter found in markdown file (tried YAML and TOML formats)");
    }

    /// Extract slug from file path
    fn extract_slug(&self, path: &Path) -> Result<String> {
        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("Invalid file name")?;

        // Remove date prefix if present (e.g., "2024-03-15-my-post" -> "my-post")
        let re = Regex::new(r"^\d{4}-\d{2}-\d{2}-(.+)$").unwrap();
        let slug = if let Some(caps) = re.captures(file_stem) {
            caps.get(1).map(|m| m.as_str()).unwrap_or(file_stem)
        } else {
            file_stem
        };

        Ok(slug.to_string())
    }

    /// Load all posts from the posts directory
    pub fn load_all_posts(&self) -> Result<Vec<Post>> {
        let posts_dir = PathBuf::from("content/posts");
        let mut posts = Vec::new();

        if !posts_dir.exists() {
            return Ok(posts);
        }

        for entry in fs::read_dir(&posts_dir)? {
            let entry = entry?;
            let path = entry.path();

            if matches!(path.extension().and_then(|s| s.to_str()), Some("md") | Some("markdown")) {
                match self.load_post(&path) {
                    Ok(post) if !post.draft => posts.push(post),
                    Ok(_) => {} // Skip drafts
                    Err(e) => tracing::warn!("Failed to load post {:?}: {}", path, e),
                }
            }
        }

        // Sort by date, newest first
        posts.sort_by(|a, b| b.date.cmp(&a.date));

        Ok(posts)
    }

    /// Load all pages from the pages directory
    pub fn load_all_pages(&self) -> Result<Vec<Page>> {
        let pages_dir = PathBuf::from("content/pages");
        let mut pages = Vec::new();

        if !pages_dir.exists() {
            return Ok(pages);
        }

        for entry in fs::read_dir(&pages_dir)? {
            let entry = entry?;
            let path = entry.path();

            if matches!(path.extension().and_then(|s| s.to_str()), Some("md") | Some("markdown")) {
                match self.load_page(&path) {
                    Ok(page) => pages.push(page),
                    Err(e) => tracing::warn!("Failed to load page {:?}: {}", path, e),
                }
            }
        }

        Ok(pages)
    }
}
