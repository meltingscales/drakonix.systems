use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::path::PathBuf;

// Custom deserializer for flexible date parsing
fn deserialize_flexible_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    // Try parsing as full ISO-8601 with timezone
    if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Try parsing as date-only (YYYY-MM-DD)
    if let Ok(date) = NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
        return Ok(Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap()));
    }

    // Try parsing other common formats
    if let Ok(dt) = DateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%z") {
        return Ok(dt.with_timezone(&Utc));
    }

    Err(serde::de::Error::custom(format!(
        "Could not parse date from '{}'. Expected ISO-8601 or YYYY-MM-DD format.",
        s
    )))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    #[serde(deserialize_with = "deserialize_flexible_date")]
    pub date: DateTime<Utc>,
    #[serde(skip_deserializing, default)]
    pub slug: String,
    #[serde(skip_deserializing, default)]
    pub url: String,
    #[serde(skip_deserializing, default)]
    pub content: String,
    #[serde(skip_deserializing, default)]
    pub html: String,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub toc: bool,
    #[serde(skip_deserializing, default)]
    pub toc_html: String,
    #[serde(skip_deserializing)]
    pub file_path: PathBuf,
    // Optional Hugo fields
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(
        rename = "authorTwitter",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub author_twitter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        rename = "showFullContent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub show_full_content: Option<bool>,
    #[serde(
        rename = "readingTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reading_time: Option<bool>,
    #[serde(
        rename = "hideComments",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_comments: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub title: String,
    #[serde(deserialize_with = "deserialize_flexible_date")]
    pub date: DateTime<Utc>,
    #[serde(skip_deserializing, default)]
    pub slug: String,
    #[serde(skip_deserializing, default)]
    pub url: String,
    #[serde(skip_deserializing, default)]
    pub content: String,
    #[serde(skip_deserializing, default)]
    pub html: String,
    #[serde(default)]
    pub toc: bool,
    #[serde(skip_deserializing, default)]
    pub toc_html: String,
    #[serde(skip_deserializing)]
    pub file_path: PathBuf,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchEntry {
    pub title: String,
    pub url: String,
    pub content: String,
    pub tags: Vec<String>,
}

impl Post {
    pub fn url(&self) -> String {
        format!("/posts/{}", self.slug)
    }

    pub fn to_search_entry(&self) -> SearchEntry {
        SearchEntry {
            title: self.title.clone(),
            url: self.url(),
            content: self.content.clone(),
            tags: self.tags.clone(),
        }
    }
}

impl Page {
    pub fn url(&self) -> String {
        format!("/pages/{}", self.slug)
    }

    pub fn to_search_entry(&self) -> SearchEntry {
        SearchEntry {
            title: self.title.clone(),
            url: self.url(),
            content: self.content.clone(),
            tags: vec![],
        }
    }
}
