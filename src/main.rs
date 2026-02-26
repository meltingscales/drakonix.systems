mod constants;
mod converter;
mod favicon;
mod handlers;
mod honeypot_db;
mod markdown;
mod markov;
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
        handlers::honeypot_hits_api,
        handlers::honeypot_config_api,
    ),
    components(
        schemas(
            timer::StartTimerRequest,
            timer::StartTimerResponse,
            timer::TimerStatusResponse,
            converter::ConvertResponse,
            honeypot_db::HoneypotHit,
        )
    ),
    tags(
        (name = "Timer", description = "Kitchen timer API endpoints"),
        (name = "Converter", description = "Media conversion API endpoints"),
        (name = "Fun", description = "its fun lol")
    ),
    info(
        title = "drakonix.systems Services API",
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
            tracing::info!(
                "Successfully loaded {} templates",
                t.get_template_names().count()
            );
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

    let db_path = std::env::var("HONEYPOT_DB_PATH").unwrap_or_else(|_| "honeypot.db".to_string());
    let honeypot_db = honeypot_db::HoneypotDb::new(&db_path)
        .map_err(|e| anyhow::anyhow!("Failed to open honeypot DB: {}", e))?;

    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(3))
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build HTTP client: {}", e))?;

    let country_cache = std::sync::Arc::new(
        tokio::sync::RwLock::new(std::collections::HashMap::<String, String>::new()),
    );

    let state = std::sync::Arc::new(AppState {
        tera,
        timer_manager,
        converter_manager,
        markov_generator,
        honeypot_db,
        http_client,
        country_cache,
    });

    // Build the application router
    let app = Router::new()
        .route("/favicon.ico", get(favicon::favicon_ico))
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
        // Death timer service
        .route("/services/death-timer", get(handlers::death_timer_page))
        // FFmpeg converter service
        .route(
            "/services/ffmpeg-mp4-to-mp3",
            get(handlers::ffmpeg_converter_page),
        )
        .route(
            "/api/convert/mp4-to-mp3",
            post(handlers::convert_mp4_to_mp3).layer(DefaultBodyLimit::max(100 * 1024 * 1024)), // 100MB limit
        )
        .route(
            "/api/convert/download/:file_id",
            get(handlers::download_converted_file),
        )
        // Honeypot endpoint - slow markov babble to trap scrapers
        .route(
            "/api/markov-babble/:slug/gen",
            get(handlers::markov_babble_honeypot),
        )
        // Honeypot dashboard + JSON API
        .route(
            "/services/honeypot-dummies",
            get(handlers::honeypot_dummies_dashboard),
        )
        .route("/api/honeypot/hits",   get(handlers::honeypot_hits_api))
        .route("/api/honeypot/config", get(handlers::honeypot_config_api))
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

pub type CountryCache = std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, String>>>;

#[derive(Clone)]
pub struct AppState {
    pub tera: tera::Tera,
    pub timer_manager: timer::TimerManager,
    pub converter_manager: converter::ConverterManager,
    pub markov_generator: markov::MarkovGenerator,
    pub honeypot_db: honeypot_db::HoneypotDb,
    pub http_client: reqwest::Client,
    pub country_cache: CountryCache,
}
