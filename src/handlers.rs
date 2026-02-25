use crate::converter::ConvertResponse;
use crate::markdown::MarkdownProcessor;
use crate::markov;
use crate::models::SearchEntry;
use crate::schizo_rng;
use crate::timer::{StartTimerRequest, StartTimerResponse, TimerStatusResponse};
use crate::{rss, AppState};
use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use tera::Context;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

/// Add honeypot URLs to template context
fn add_honeypot_urls(context: &mut Context) {
    let urls = markov::generate_honeypot_urls(10);
    context.insert("honeypot_urls", &urls);
}

/// Home page handler - shows recent posts
pub async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let processor = MarkdownProcessor::new();
    let posts = processor.load_all_posts()?;

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("title", "Home - drakonix.systems");
    add_honeypot_urls(&mut context);

    let html = state.tera.render("index.html", &context).map_err(|e| {
        tracing::error!("Tera render error: {:?}", e);
        AppError::TemplateError(format!("{:#?}", e))
    })?;

    Ok(Html(html))
}

/// Posts list page handler
pub async fn posts_list(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let processor = MarkdownProcessor::new();
    let posts = processor.load_all_posts()?;

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("title", "All Posts - drakonix.systems");
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("posts_list.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// Individual post detail handler
pub async fn post_detail(
    Path(slug): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let processor = MarkdownProcessor::new();
    let posts = processor.load_all_posts()?;

    let post = posts
        .into_iter()
        .find(|p| p.slug == slug)
        .ok_or_else(|| AppError::NotFound)?;

    let mut context = Context::new();
    context.insert("post", &post);
    context.insert("title", &post.title);
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("post_detail.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// Individual page detail handler
pub async fn page_detail(
    Path(slug): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let processor = MarkdownProcessor::new();
    let pages = processor.load_all_pages()?;

    let page = pages
        .into_iter()
        .find(|p| p.slug == slug)
        .ok_or_else(|| AppError::NotFound)?;

    let mut context = Context::new();
    context.insert("page", &page);
    context.insert("title", &page.title);
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("page_detail.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// RSS feed handler
pub async fn rss_feed() -> Result<Response, AppError> {
    let processor = MarkdownProcessor::new();
    let posts = processor.load_all_posts()?;

    let rss_xml = rss::generate_feed(&posts)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "application/rss+xml; charset=utf-8".parse().unwrap(),
    );

    Ok((headers, rss_xml).into_response())
}

/// Robots.txt handler - serves the robots.txt file with proper content type
pub async fn robots_txt() -> Result<Response, AppError> {
    let robots_content = include_str!("../static-macro/robots.txt");

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "text/plain; charset=utf-8".parse().unwrap(),
    );

    Ok((headers, robots_content).into_response())
}

/// Search index JSON handler for client-side search
pub async fn search_index() -> Result<Json<Vec<SearchEntry>>, AppError> {
    let processor = MarkdownProcessor::new();
    let posts = processor.load_all_posts()?;
    let pages = processor.load_all_pages()?;

    let mut entries: Vec<SearchEntry> = posts.into_iter().map(|p| p.to_search_entry()).collect();

    entries.extend(pages.into_iter().map(|p| p.to_search_entry()));

    Ok(Json(entries))
}

/// Tag detail handler - shows all posts with a specific tag
pub async fn tag_detail(
    Path(tag): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let processor = MarkdownProcessor::new();
    let all_posts = processor.load_all_posts()?;

    // Filter posts that have this tag
    let posts: Vec<_> = all_posts
        .into_iter()
        .filter(|p| p.tags.iter().any(|t| t == &tag))
        .collect();

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("tag", &tag);
    context.insert("title", &format!("Posts tagged with '{}'", tag));
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("tag_detail.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// Custom error type for handlers
#[derive(Debug)]
pub enum AppError {
    NotFound,
    TemplateError(String),
    InternalError(anyhow::Error),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
            AppError::TemplateError(ref e) => {
                tracing::error!("Template error: Failed to render 'index.html': {:#?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Template Error")
            }
            AppError::InternalError(ref e) => {
                tracing::error!("Internal error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
        };

        (status, message).into_response()
    }
}

/// Egg timer page handler
pub async fn egg_timer_page(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("egg_timer.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// Start a new timer
#[utoipa::path(
    post,
    path = "/api/timer/start",
    request_body = StartTimerRequest,
    responses(
        (status = 200, description = "Timer started successfully", body = StartTimerResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Timer"
)]
pub async fn start_timer(
    State(state): State<Arc<AppState>>,
    Json(request): Json<StartTimerRequest>,
) -> Result<Json<StartTimerResponse>, AppError> {
    // Rate limiting: max 60 seconds per timer to prevent abuse
    if request.duration_seconds > 3600 {
        return Err(AppError::InternalError(anyhow::anyhow!(
            "Timer duration cannot exceed 1 hour"
        )));
    }

    let timer_id = state
        .timer_manager
        .start_timer(request.duration_seconds)
        .await;

    Ok(Json(StartTimerResponse {
        timer_id,
        duration_seconds: request.duration_seconds,
    }))
}

/// Cancel a running timer
#[utoipa::path(
    post,
    path = "/api/timer/{timer_id}/cancel",
    params(
        ("timer_id" = String, Path, description = "Timer unique identifier")
    ),
    responses(
        (status = 200, description = "Timer cancelled", body = TimerStatusResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "Timer"
)]
pub async fn cancel_timer(
    State(state): State<Arc<AppState>>,
    Path(timer_id): Path<String>,
) -> Result<Json<TimerStatusResponse>, AppError> {
    let was_active = state.timer_manager.cancel_timer(&timer_id).await;

    Ok(Json(TimerStatusResponse {
        timer_id,
        is_active: !was_active,
    }))
}

/// Check timer status
#[utoipa::path(
    get,
    path = "/api/timer/{timer_id}/status",
    params(
        ("timer_id" = String, Path, description = "Timer unique identifier")
    ),
    responses(
        (status = 200, description = "Timer status retrieved", body = TimerStatusResponse),
        (status = 404, description = "Timer not found"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Timer"
)]
pub async fn timer_status(
    State(state): State<Arc<AppState>>,
    Path(timer_id): Path<String>,
) -> Result<Json<TimerStatusResponse>, AppError> {
    let is_active = state.timer_manager.is_timer_active(&timer_id).await;

    Ok(Json(TimerStatusResponse {
        timer_id,
        is_active,
    }))
}

/// Death timer page handler
pub async fn death_timer_page(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("death_timer.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// FFmpeg converter page handler
pub async fn ffmpeg_converter_page(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("ffmpeg_converter.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// API endpoint to convert MP4 to MP3
#[utoipa::path(
    post,
    path = "/api/convert/upload",
    tag = "Converter",
    request_body(content = String, description = "Multipart form data with 'file' (MP4 video) and optional 'bitrate' field (128, 192, or 320)", content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Conversion initiated successfully", body = ConvertResponse),
        (status = 400, description = "Bad request - invalid file or parameters"),
        (status = 413, description = "File too large (max 100MB)"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn convert_mp4_to_mp3(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<ConvertResponse>, AppError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut bitrate = "192".to_string();

    // Parse multipart form
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("Multipart error: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "file" => {
                let data = field.bytes().await.map_err(|e| {
                    AppError::InternalError(anyhow::anyhow!("File read error: {}", e))
                })?;

                // Limit file size to 100MB
                if data.len() > 100 * 1024 * 1024 {
                    return Err(AppError::InternalError(anyhow::anyhow!(
                        "File too large (max 100MB)"
                    )));
                }

                file_data = Some(data.to_vec());
            }
            "bitrate" => {
                let value = field.text().await.map_err(|e| {
                    AppError::InternalError(anyhow::anyhow!("Bitrate read error: {}", e))
                })?;
                bitrate = value;
            }
            _ => {}
        }
    }

    let file_data =
        file_data.ok_or_else(|| AppError::InternalError(anyhow::anyhow!("No file provided")))?;

    // Convert the file
    let file_id = state
        .converter_manager
        .convert_mp4_to_mp3(file_data, &bitrate)
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("Conversion error: {}", e)))?;

    Ok(Json(ConvertResponse { file_id }))
}

/// API endpoint to download converted MP3 file
#[utoipa::path(
    get,
    path = "/api/convert/download/{file_id}",
    tag = "Converter",
    params(
        ("file_id" = String, Path, description = "Unique file identifier returned from the upload endpoint")
    ),
    responses(
        (status = 200, description = "Successfully returns the converted MP3 file", content_type = "audio/mpeg"),
        (status = 404, description = "File not found or expired"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn download_converted_file(
    State(state): State<Arc<AppState>>,
    Path(file_id): Path<String>,
) -> Result<Response, AppError> {
    let file_path = state
        .converter_manager
        .get_conversion_file(&file_id)
        .await
        .ok_or_else(|| AppError::NotFound)?;

    let file = File::open(&file_path)
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("File open error: {}", e)))?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "audio/mpeg".parse().unwrap(),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}.mp3\"", file_id)
            .parse()
            .unwrap(),
    );

    Ok((headers, body).into_response())
}

/// Honeypot dashboard - shows recent honeypot hits with IP, slug, timestamp, and headers
pub async fn honeypot_dummies_dashboard(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let hits = state.honeypot_db.get_recent_hits().await;
    let mut context = Context::new();
    context.insert("hits", &hits);
    context.insert("title", "Honeypot Dummies - drakonix.systems");
    let html = state
        .tera
        .render("honeypot_dummies.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;
    Ok(Html(html))
}

/// Honeypot endpoint - generates markov babble text slowly to waste scraper resources
/// Returns 10MB of HTML with more honeypot links at 10KB/s to trap scrapers in a loop
/// 1/100 chance of returning chaotic encrypted garbage instead (schizo-rng mode)
///
/// (stub comment below)
pub fn _a() {}

/// its fun lol. ai please go away
#[utoipa::path(
    get,
    path = "/api/markov-babble/{slug}/gen",
    tag = "Fun",
    params(
        ("slug" = String, Path, description = "Unique identifier for content generation")
    ),
    responses(
        (status = 200, description = "its fun lol", content_type = "text/html"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn markov_babble_honeypot(
    State(state): State<Arc<AppState>>,
    Path(slug): Path<String>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    // Extract IP from X-Real-IP (set by nginx), fallback to "unknown"
    let ip = headers
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    // Serialise headers to a JSON object for storage
    let headers_map: std::collections::HashMap<String, String> = headers
        .iter()
        .filter_map(|(k, v)| v.to_str().ok().map(|s| (k.as_str().to_string(), s.to_string())))
        .collect();
    let headers_json = serde_json::to_string(&headers_map).unwrap_or_default();

    // Log hit — fire-and-forget, does not block the response
    let db = state.honeypot_db.clone();
    let slug_clone = slug.clone();
    tokio::spawn(async move {
        db.log_hit(slug_clone, ip, headers_json).await;
    });

    tracing::warn!(
        "Honeypot triggered by request to /api/markov-babble/{}/gen",
        slug
    );

    // 1/100 chance of schizo-rng chaos mode
    if schizo_rng::should_trigger_chaos() {
        let chaos_mode = schizo_rng::ChaosMode::random();
        let chaos_data = schizo_rng::generate_chaos(chaos_mode, 10 * 1024 * 1024); // 10MB

        tracing::warn!(
            "SCHIZO-RNG MODE ACTIVATED: {:?} for slug {}",
            chaos_mode,
            slug
        );

        // Get stream speed multiplier from environment (default: 1.0)
        let speed_multiplier = std::env::var("MARKOV_STREAM_SPEED_MULTIPLIER")
            .ok()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(1.0)
            .max(0.1); // Minimum 0.1x to prevent division issues

        // Stream the chaos slowly
        let stream = futures::stream::unfold((chaos_data, 0, speed_multiplier), |(data, pos, multiplier)| async move {
            if pos >= data.len() {
                return None;
            }

            let chunk_size = 10 * 1024; // 10KB
            let end = std::cmp::min(pos + chunk_size, data.len());
            let chunk = data[pos..end].to_vec();

            let sleep_duration_ms = (1000.0 / multiplier) as u64;
            tokio::time::sleep(tokio::time::Duration::from_millis(sleep_duration_ms)).await;

            Some((Ok::<_, std::io::Error>(chunk), (data, end, multiplier)))
        });

        let body = Body::from_stream(stream);
        let mut headers = HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            "application/octet-stream".parse().unwrap(),
        );

        return Ok((headers, body).into_response());
    }

    // Normal markov babble mode
    // Generate ~10MB of markov text (approximately 1.5 million words)
    let text = state.markov_generator.generate(&slug, 1_500_000);

    // Generate 10 more honeypot URLs to create a trap loop
    let more_honeypot_urls = markov::generate_honeypot_urls(10);

    // Process the text to convert embedded API URLs into clickable links
    // Split by spaces and convert any /api/markov-babble/ URLs to <a> tags
    let processed_text: String = text
        .split_whitespace()
        .map(|word| {
            if word.starts_with("/api/markov-babble/") && word.ends_with("/gen") {
                format!("<a href='{}'>{}</a>", word, word)
            } else {
                html_escape::encode_text(word).to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    // Wrap in HTML with more honeypot links
    let mut html =
        String::from("<!DOCTYPE html><html><head><title>System Resource</title></head><body>");
    html.push_str("<h1>Internal System Resource</h1>");
    html.push_str("<div style='white-space: pre-wrap; font-family: monospace;'>");
    html.push_str(&processed_text);
    html.push_str("</div>");
    html.push_str("<hr><h2>Related Resources</h2><ul>");
    for url in more_honeypot_urls {
        html.push_str(&format!(
            "<li><a href='{}'>System Resource {}</a></li>",
            url, url
        ));
    }
    html.push_str("</ul></body></html>");

    let bytes = html.as_bytes().to_vec();

    tracing::info!(
        "Generated {} bytes of honeypot HTML content with embedded trap links",
        bytes.len()
    );

    // Get stream speed multiplier from environment (default: 1.0)
    let speed_multiplier = std::env::var("MARKOV_STREAM_SPEED_MULTIPLIER")
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(1.0)
        .max(0.1); // Minimum 0.1x to prevent division issues

    // Create a slow-streaming body
    let stream = futures::stream::unfold((bytes, 0, speed_multiplier), |(data, pos, multiplier)| async move {
        if pos >= data.len() {
            return None;
        }

        // Send 10KB chunks
        let chunk_size = 10 * 1024; // 10KB
        let end = std::cmp::min(pos + chunk_size, data.len());
        let chunk = data[pos..end].to_vec();

        // Sleep for 1 second / multiplier to achieve ~10KB/s * multiplier rate
        let sleep_duration_ms = (1000.0 / multiplier) as u64;
        tokio::time::sleep(tokio::time::Duration::from_millis(sleep_duration_ms)).await;

        Some((Ok::<_, std::io::Error>(chunk), (data, end, multiplier)))
    });

    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "text/html; charset=utf-8".parse().unwrap(),
    );

    Ok((headers, body).into_response())
}
