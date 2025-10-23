mod handlers;
mod markdown;
mod models;
mod rss;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_blog=debug,tower_http=debug,axum=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting application initialization...");

    // Initialize template engine
    tracing::info!("Loading templates from templates/**/*.html");
    let mut tera = match tera::Tera::new("templates/**/*.html") {
        Ok(t) => {
            tracing::info!("Successfully loaded {} templates", t.get_template_names().count());
            t
        }
        Err(e) => {
            tracing::error!("Failed to load templates: {}", e);
            return Err(e.into());
        }
    };

    // Register custom date filter
    tera.register_filter(
        "date",
        |value: &tera::Value, args: &std::collections::HashMap<String, tera::Value>| {
            use chrono::{DateTime, Utc};

            // Parse the date string
            let date_str = value
                .as_str()
                .ok_or_else(|| tera::Error::msg("Date value must be a string"))?;
            let dt = DateTime::parse_from_rfc3339(date_str)
                .map_err(|e| tera::Error::msg(format!("Invalid date format: {}", e)))?
                .with_timezone(&Utc);

            // Get format argument
            let format = args
                .get("format")
                .and_then(|v| v.as_str())
                .unwrap_or("%Y-%m-%d");

            Ok(tera::to_value(dt.format(format).to_string())?)
        },
    );

    let state = std::sync::Arc::new(AppState { tera });

    // Build the application router
    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/posts", get(handlers::posts_list))
        .route("/posts/:slug", get(handlers::post_detail))
        .route("/pages/:slug", get(handlers::page_detail))
        .route("/tags/:tag", get(handlers::tag_detail))
        .route("/rss.xml", get(handlers::rss_feed))
        .route("/search.json", get(handlers::search_index))
        // Serve static files (CSS, JS, images)
        .nest_service("/static", ServeDir::new("static"))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Determine port from environment or default to 8080
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    tracing::info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    pub tera: tera::Tera,
}
