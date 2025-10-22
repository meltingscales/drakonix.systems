use crate::models::Post;
use anyhow::Result;
use rss::{ChannelBuilder, ItemBuilder};

pub fn generate_feed(posts: &[Post]) -> Result<String> {
    let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| "https://example.com".to_string());

    let items: Vec<rss::Item> = posts
        .iter()
        .take(20) // Limit to 20 most recent posts
        .map(|post| {
            ItemBuilder::default()
                .title(Some(post.title.clone()))
                .link(Some(format!("{}{}", base_url, post.url())))
                .description(Some(post.html.clone()))
                .pub_date(Some(post.date.to_rfc2822()))
                .build()
        })
        .collect();

    let channel = ChannelBuilder::default()
        .title("Rust Blog")
        .link(base_url.clone())
        .description("A blog powered by Rust")
        .language(Some("en-us".to_string()))
        .items(items)
        .build();

    Ok(channel.to_string())
}
