use crate::markdown::MarkdownProcessor;
use crate::models::SearchEntry;
use crate::timer::{StartTimerRequest, StartTimerResponse, TimerStatusResponse};
use crate::{rss, AppState};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use tera::Context;

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

/// API endpoint to start a timer
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

/// API endpoint to cancel a timer
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

/// API endpoint to check timer status
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
