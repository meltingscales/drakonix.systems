use crate::converter::ConvertResponse;
use crate::markdown::MarkdownProcessor;
use crate::models::SearchEntry;
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

/// Home page handler - shows recent posts
pub async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let processor = MarkdownProcessor::new();
    let posts = processor.load_all_posts()?;

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("title", "Home");

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
    context.insert("title", "All Posts");

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
    let context = Context::new();

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

    let timer_id = state.timer_manager.start_timer(request.duration_seconds).await;

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

/// FFmpeg converter page handler
pub async fn ffmpeg_converter_page(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let context = Context::new();

    let html = state
        .tera
        .render("ffmpeg_converter.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// API endpoint to convert MP4 to MP3
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
                let data = field
                    .bytes()
                    .await
                    .map_err(|e| AppError::InternalError(anyhow::anyhow!("File read error: {}", e)))?;

                // Limit file size to 100MB
                if data.len() > 100 * 1024 * 1024 {
                    return Err(AppError::InternalError(anyhow::anyhow!(
                        "File too large (max 100MB)"
                    )));
                }

                file_data = Some(data.to_vec());
            }
            "bitrate" => {
                let value = field
                    .text()
                    .await
                    .map_err(|e| AppError::InternalError(anyhow::anyhow!("Bitrate read error: {}", e)))?;
                bitrate = value;
            }
            _ => {}
        }
    }

    let file_data = file_data.ok_or_else(|| AppError::InternalError(anyhow::anyhow!("No file provided")))?;

    // Convert the file
    let file_id = state
        .converter_manager
        .convert_mp4_to_mp3(file_data, &bitrate)
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("Conversion error: {}", e)))?;

    Ok(Json(ConvertResponse { file_id }))
}

/// API endpoint to download converted MP3 file
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
