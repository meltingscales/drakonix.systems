mod converter;
mod handlers;
mod markov;
mod markdown;
mod models;
mod rss;
mod schizo_rng;
mod timer;

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::{compression::CompressionLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::start_timer,
        handlers::cancel_timer,
        handlers::timer_status,
        handlers::convert_mp4_to_mp3,
        handlers::download_converted_file,
        handlers::markov_babble_honeypot,
    ),
    components(
        schemas(
            timer::StartTimerRequest,
            timer::StartTimerResponse,
            timer::TimerStatusResponse,
            converter::ConvertResponse,
        )
    ),
    tags(
        (name = "Timer", description = "Kitchen timer API endpoints"),
        (name = "Converter", description = "Media conversion API endpoints"),
        (name = "Fun", description = "its fun lol")
    ),
    info(
        title = "Rust Blog Services API",
        version = "0.1.0",
        description = "API documentation for blog services including timers and media conversion",
    )
)]
struct ApiDoc;

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

    let timer_manager = timer::TimerManager::new();
    let converter_manager = converter::ConverterManager::new()
        .map_err(|e| anyhow::anyhow!("Failed to initialize converter: {}", e))?;

    let markov_generator = markov::MarkovGenerator::new();

    let state = std::sync::Arc::new(AppState {
        tera,
        timer_manager,
        converter_manager,
        markov_generator,
    });

    // Build the application router
    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/posts", get(handlers::posts_list))
        .route("/posts/:slug", get(handlers::post_detail))
        .route("/pages/:slug", get(handlers::page_detail))
        .route("/tags/:tag", get(handlers::tag_detail))
        .route("/rss.xml", get(handlers::rss_feed))
        .route("/robots.txt", get(handlers::robots_txt))
        .route("/search.json", get(handlers::search_index))
        // Egg timer service
        .route("/services/egg-timer", get(handlers::egg_timer_page))
        .route("/api/timer/start", post(handlers::start_timer))
        .route("/api/timer/:timer_id/cancel", post(handlers::cancel_timer))
        .route("/api/timer/:timer_id/status", get(handlers::timer_status))
        // FFmpeg converter service
        .route("/services/ffmpeg-mp4-to-mp3", get(handlers::ffmpeg_converter_page))
        .route(
            "/api/convert/mp4-to-mp3",
            post(handlers::convert_mp4_to_mp3)
                .layer(DefaultBodyLimit::max(100 * 1024 * 1024)) // 100MB limit
        )
        .route("/api/convert/download/:file_id", get(handlers::download_converted_file))
        // Honeypot endpoint - slow markov babble to trap scrapers
        .route("/api/markov-babble/:slug/gen", get(handlers::markov_babble_honeypot))
        // Swagger UI for API documentation
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
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
    pub timer_manager: timer::TimerManager,
    pub converter_manager: converter::ConverterManager,
    pub markov_generator: markov::MarkovGenerator,
}
