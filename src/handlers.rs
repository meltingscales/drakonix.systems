use crate::aa_search::AaHit;
use crate::converter::ConvertResponse;
use crate::saa_search::SaaHit;
use crate::doggypastebin::{CreatePasteRequest, CreatePasteResponse};
use crate::markdown::MarkdownProcessor;
use crate::markov;
use crate::models::SearchEntry;
use crate::schizo_rng;
use crate::timer::{StartTimerRequest, StartTimerResponse, TimerStatusResponse};
use crate::twitch_icon_gen::IconGenResponse;
use crate::twitch_emote_gen::EmoteGenResponse;
use crate::{constants, honeypot_db, rss, AppState, CountryCache, OrgCache};
use axum::{
    body::Body,
    extract::{Multipart, Path, Query, Request, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use tera::Context;
use tokio::fs::File;
use tokio_util::io::ReaderStream;

/// Add honeypot URLs to template context
fn add_honeypot_urls(context: &mut Context) {
    let urls = markov::generate_honeypot_urls(10);
    context.insert("honeypot_urls", &urls);
}

/// Home page handler - shows about-me page
pub async fn index(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let processor = MarkdownProcessor::new();
    let pages = processor.load_all_pages()?;

    let page = pages
        .into_iter()
        .find(|p| p.slug == "about-me")
        .ok_or_else(|| AppError::NotFound)?;

    let mut context = Context::new();
    context.insert("page", &page);
    context.insert("title", &page.title);
    add_honeypot_urls(&mut context);

    let html = state.tera.render("page_detail.html", &context).map_err(|e| {
        tracing::error!("Tera render error: {:?}", e);
        AppError::TemplateError(format!("{:#?}", e))
    })?;

    Ok(Html(html))
}

/// Blog page handler - shows recent posts
pub async fn blog(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let processor = MarkdownProcessor::new();
    let posts = processor.load_all_posts()?;

    let mut context = Context::new();
    context.insert("posts", &posts);
    context.insert("title", "Blog - drakonix.systems");
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
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::TemplateError(ref e) => {
                tracing::error!("Template error: {:#?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e))
            }
            AppError::InternalError(ref e) => {
                tracing::error!("Internal error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        };

        (
            status,
            axum::Json(serde_json::json!({ "error": message })),
        )
            .into_response()
    }
}

/// Services index page handler
pub async fn services_index(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("title", "~/services — drakonix.systems");
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("services.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
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

/// Loan amortization calculator page handler
pub async fn loan_calculator_page(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("loan_calculator.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// Solar panel payoff calculator page handler
pub async fn solar_payoff_page(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("solar_payoff.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// Linked list learning REPL page handler
pub async fn linked_list_learning_page(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("title", "Linked List Learning - drakonix.systems");
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("linked_list_learning.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// Bifurcation diagram viewer page handler
pub async fn bifurcation_diagram_viewer_page(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("bifurcation_diagram_viewer.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// Riemann zeta zeros critical line visualizer page handler
pub async fn riemann_zeta_zeros_page(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("riemann_zeta_zeros.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

/// AA full-text search page handler
pub async fn aa_search_page(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let html = state
        .tera
        .render("aa_search.html", &Context::new())
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct AaSearchParams {
    q: String,
}

pub async fn aa_search_query(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AaSearchParams>,
) -> Json<Vec<AaHit>> {
    Json(state.aa_search_manager.search(&params.q))
}

#[derive(Deserialize)]
pub struct AaPageParams {
    book: String,
    page_num: u32,
}

pub async fn aa_get_page(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AaPageParams>,
) -> Result<Json<AaHit>, StatusCode> {
    state
        .aa_search_manager
        .get_page(&params.book, params.page_num)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Deserialize)]
pub struct AaSimilarParams {
    word: String,
}

#[derive(Deserialize)]
struct DatamuseWord {
    word: String,
}

// "Similar words" via Datamuse (free, no API key). The original 164andmore
// site uses Merriam-Webster's Thesaurus API, which requires a registered
// key we don't have.
pub async fn aa_similar_words(
    State(state): State<Arc<AppState>>,
    Query(params): Query<AaSimilarParams>,
) -> Json<Vec<String>> {
    let resp = state
        .http_client
        .get("https://api.datamuse.com/words")
        .query(&[("ml", params.word.as_str()), ("max", "10")])
        .send()
        .await
        .ok()
        .and_then(|r| r.error_for_status().ok());
    let Some(resp) = resp else {
        return Json(Vec::new());
    };
    let parsed: Vec<DatamuseWord> = resp.json().await.unwrap_or_default();
    Json(parsed.into_iter().map(|w| w.word).collect())
}

/// SAA full-text search page handler
pub async fn saa_search_page(State(state): State<Arc<AppState>>) -> Result<Html<String>, AppError> {
    let html = state
        .tera
        .render("saa_search.html", &Context::new())
        .map_err(|e| AppError::TemplateError(e.to_string()))?;

    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct SaaSearchParams {
    q: String,
}

pub async fn saa_search_query(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SaaSearchParams>,
) -> Json<Vec<SaaHit>> {
    Json(state.saa_search_manager.search(&params.q))
}

#[derive(Deserialize)]
pub struct SaaPageParams {
    page_num: u32,
}

pub async fn saa_get_page(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SaaPageParams>,
) -> Result<Json<SaaHit>, StatusCode> {
    state
        .saa_search_manager
        .get_page(params.page_num)
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

#[derive(Deserialize)]
pub struct SaaSimilarParams {
    word: String,
}

pub async fn saa_similar_words(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SaaSimilarParams>,
) -> Json<Vec<String>> {
    let resp = state
        .http_client
        .get("https://api.datamuse.com/words")
        .query(&[("ml", params.word.as_str()), ("max", "10")])
        .send()
        .await
        .ok()
        .and_then(|r| r.error_for_status().ok());
    let Some(resp) = resp else {
        return Json(Vec::new());
    };
    let parsed: Vec<DatamuseWord> = resp.json().await.unwrap_or_default();
    Json(parsed.into_iter().map(|w| w.word).collect())
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

/// Twitch sub badge icon generator page
pub async fn twitch_icon_gen_page(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    add_honeypot_urls(&mut context);
    let html = state
        .tera
        .render("twitch_icon_gen_generic.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;
    Ok(Html(html))
}

/// API endpoint to generate Twitch sub badge icon pack from uploaded images
#[utoipa::path(
    post,
    path = "/api/twitch-icons/generate",
    tag = "Twitch",
    request_body(
        content = String,
        description = "Multipart form with one or more 'images[]' fields (JPG/PNG, max 500MB total). Each image is processed at 18×18, 36×36, and 72×72 px with transparent background.",
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "Icons generated; download ZIP via job_id", body = IconGenResponse),
        (status = 400, description = "No images provided"),
        (status = 413, description = "Upload too large (max 500MB)"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn generate_twitch_icons(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<IconGenResponse>, AppError> {
    let mut images: Vec<(String, Vec<u8>)> = Vec::new();
    let mut sizes: Vec<u32> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("Multipart error: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "images[]" | "images" => {
                let filename = field.file_name().unwrap_or("image.png").to_string();
                let data = field.bytes().await.map_err(|e| {
                    AppError::InternalError(anyhow::anyhow!("File read error: {}", e))
                })?;
                if !data.is_empty() {
                    images.push((filename, data.to_vec()));
                }
            }
            "sizes[]" | "sizes" => {
                let val = field.text().await.map_err(|e| {
                    AppError::InternalError(anyhow::anyhow!("Sizes read error: {}", e))
                })?;
                if let Ok(n) = val.trim().parse::<u32>() {
                    if matches!(n, 18 | 36 | 72) {
                        sizes.push(n);
                    }
                }
            }
            _ => {}
        }
    }

    if images.is_empty() {
        return Err(AppError::InternalError(anyhow::anyhow!(
            "No images provided"
        )));
    }

    // Default to all three sizes if none specified
    if sizes.is_empty() {
        sizes = vec![18, 36, 72];
    }
    sizes.sort_unstable();
    sizes.dedup();

    let (job_id, results) = state
        .twitch_icon_gen_manager
        .generate_icon_pack(images, sizes)
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("Icon generation error: {}", e)))?;

    Ok(Json(IconGenResponse { job_id, results }))
}

/// API endpoint to download a generated Twitch icon pack ZIP
#[utoipa::path(
    get,
    path = "/api/twitch-icons/download/{job_id}",
    tag = "Twitch",
    params(
        ("job_id" = String, Path, description = "Job identifier returned from the generate endpoint")
    ),
    responses(
        (status = 200, description = "Returns the icon pack as a ZIP archive", content_type = "application/zip"),
        (status = 404, description = "Job not found or expired (10-minute TTL)"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn download_twitch_icon_pack(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<String>,
) -> Result<Response, AppError> {
    let zip_path = state
        .twitch_icon_gen_manager
        .get_zip_file(&job_id)
        .await
        .ok_or(AppError::NotFound)?;

    let file = File::open(&zip_path)
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("File open error: {}", e)))?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "application/zip".parse().unwrap(),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"twitch-badges-{}.zip\"", &job_id[..8])
            .parse()
            .unwrap(),
    );

    Ok((headers, body).into_response())
}

/// Twitch emote generator page
pub async fn twitch_emote_gen_page(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    add_honeypot_urls(&mut context);
    let html = state
        .tera
        .render("twitch_emote_gen.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;
    Ok(Html(html))
}

/// API endpoint to generate Twitch emote pack from uploaded images
#[utoipa::path(
    post,
    path = "/api/twitch-emotes/generate",
    tag = "Twitch",
    request_body(
        content = String,
        description = "Multipart form with one or more 'images[]' fields (PNG/GIF, max 50MB total). Each image is processed at 28×28, 56×56, and 112×112 px.",
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "Emotes generated; download ZIP via job_id", body = EmoteGenResponse),
        (status = 400, description = "No images provided"),
        (status = 413, description = "Upload too large (max 50MB)"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn generate_twitch_emotes(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<Json<EmoteGenResponse>, AppError> {
    let mut images: Vec<(String, Vec<u8>)> = Vec::new();
    let mut sizes: Vec<u32> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("Multipart error: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "images[]" | "images" => {
                let filename = field.file_name().unwrap_or("emote.png").to_string();
                let data = field.bytes().await.map_err(|e| {
                    AppError::InternalError(anyhow::anyhow!("File read error: {}", e))
                })?;
                if !data.is_empty() {
                    images.push((filename, data.to_vec()));
                }
            }
            "sizes[]" | "sizes" => {
                let val = field.text().await.map_err(|e| {
                    AppError::InternalError(anyhow::anyhow!("Sizes read error: {}", e))
                })?;
                if let Ok(n) = val.trim().parse::<u32>() {
                    if matches!(n, 28 | 56 | 112) {
                        sizes.push(n);
                    }
                }
            }
            _ => {}
        }
    }

    if images.is_empty() {
        return Err(AppError::InternalError(anyhow::anyhow!(
            "No images provided"
        )));
    }

    if sizes.is_empty() {
        sizes = vec![28, 56, 112];
    }
    sizes.sort_unstable();
    sizes.dedup();

    let (job_id, results) = state
        .twitch_emote_gen_manager
        .generate_emote_pack(images, sizes)
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("Emote generation error: {}", e)))?;

    Ok(Json(EmoteGenResponse { job_id, results }))
}

/// API endpoint to download a generated Twitch emote pack ZIP
#[utoipa::path(
    get,
    path = "/api/twitch-emotes/download/{job_id}",
    tag = "Twitch",
    params(
        ("job_id" = String, Path, description = "Job identifier returned from the generate endpoint")
    ),
    responses(
        (status = 200, description = "Returns the emote pack as a ZIP archive", content_type = "application/zip"),
        (status = 404, description = "Job not found or expired (10-minute TTL)"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn download_twitch_emote_pack(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<String>,
) -> Result<Response, AppError> {
    let zip_path = state
        .twitch_emote_gen_manager
        .get_zip_file(&job_id)
        .await
        .ok_or(AppError::NotFound)?;

    let file = File::open(&zip_path)
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("File open error: {}", e)))?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "application/zip".parse().unwrap(),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"twitch-emotes-{}.zip\"", &job_id[..8])
            .parse()
            .unwrap(),
    );

    Ok((headers, body).into_response())
}

/// Honeypot map timeline - animated world map of honeypot hits
pub async fn honeypot_map_timeline_dashboard(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("title", "Honeypot Map Timeline - drakonix.systems");
    let html = state
        .tera
        .render("honeypot_map_timeline.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;
    Ok(Html(html))
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

/// JSON API – returns the most recent honeypot hits (up to HONEYPOT_MAX_ENTRIES)
#[utoipa::path(
    get,
    path = "/api/honeypot/hits",
    tag = "Fun",
    responses(
        (status = 200, description = "List of recent honeypot hits (newest first)", body = Vec<honeypot_db::HoneypotHit>),
    )
)]
pub async fn honeypot_hits_api(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<honeypot_db::HoneypotHit>>, AppError> {
    let hits = state.honeypot_db.get_recent_hits().await;
    Ok(Json(hits))
}

/// JSON API – returns the current honeypot configuration constants
#[utoipa::path(
    get,
    path = "/api/honeypot/config",
    tag = "Fun",
    responses(
        (status = 200, description = "Honeypot configuration constants", body = serde_json::Value),
    )
)]
pub async fn honeypot_config_api() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "max_entries": constants::HONEYPOT_MAX_ENTRIES,
    }))
}

/// Look up an IP's country code via ipinfo.io, using an in-process cache so
/// each unique IP is only ever fetched once per server lifetime.
async fn lookup_country(
    client: &reqwest::Client,
    cache: &CountryCache,
    ip: &str,
) -> String {
    if ip == "unknown" {
        return String::new();
    }

    // Fast path: cache hit
    {
        let guard = cache.read().await;
        if let Some(country) = guard.get(ip) {
            return country.clone();
        }
    }

    // Cache miss: fetch from ipinfo.io (/country returns "US\n" style plain text)
    let country = match client
        .get(format!("https://ipinfo.io/{}/country", ip))
        .send()
        .await
    {
        Ok(resp) => resp.text().await.unwrap_or_default().trim().to_string(),
        Err(_) => String::new(),
    };

    cache.write().await.insert(ip.to_string(), country.clone());
    country
}

/// Look up an IP's ASN + org via ipinfo.io with the same cache pattern as lookup_country.
async fn lookup_org(
    client: &reqwest::Client,
    cache: &OrgCache,
    ip: &str,
) -> String {
    if ip == "unknown" {
        return String::new();
    }

    {
        let guard = cache.read().await;
        if let Some(org) = guard.get(ip) {
            return org.clone();
        }
    }

    // /org returns e.g. "AS14061 DigitalOcean, LLC\n"
    let org = match client
        .get(format!("https://ipinfo.io/{}/org", ip))
        .send()
        .await
    {
        Ok(resp) => resp.text().await.unwrap_or_default().trim().to_string(),
        Err(_) => String::new(),
    };

    cache.write().await.insert(ip.to_string(), org.clone());
    org
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
    let db            = state.honeypot_db.clone();
    let http_client   = state.http_client.clone();
    let country_cache = state.country_cache.clone();
    let org_cache     = state.org_cache.clone();
    let slug_clone    = slug.clone();
    let ip_clone      = ip.clone();
    tokio::spawn(async move {
        let (country, org) = tokio::join!(
            lookup_country(&http_client, &country_cache, &ip_clone),
            lookup_org(&http_client, &org_cache, &ip_clone),
        );
        db.log_hit(slug_clone, ip_clone, headers_json, String::new(), country, org).await;
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

/// Catch-all fallback — logs any unmatched path (+ query string + body) as a honeypot hit and returns 404.
pub async fn catch_all_honeypot(
    State(state): State<Arc<AppState>>,
    request: Request,
) -> impl IntoResponse {
    let (parts, body) = request.into_parts();

    let slug = parts.uri.path_and_query()
        .map(|pq| pq.as_str().to_string())
        .unwrap_or_else(|| parts.uri.path().to_string());

    let ip = parts.headers
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    let headers_map: std::collections::HashMap<String, String> = parts.headers
        .iter()
        .filter_map(|(k, v)| v.to_str().ok().map(|s| (k.as_str().to_string(), s.to_string())))
        .collect();
    let headers_json = serde_json::to_string(&headers_map).unwrap_or_default();

    // Read up to 64 KB of body; silently truncate anything larger.
    let body_bytes = axum::body::to_bytes(body, 64 * 1024).await.unwrap_or_default();
    let body_str = String::from_utf8_lossy(&body_bytes).into_owned();

    tracing::warn!("Catch-all honeypot hit: {} from {}", slug, ip);

    let db            = state.honeypot_db.clone();
    let http_client   = state.http_client.clone();
    let country_cache = state.country_cache.clone();
    let org_cache     = state.org_cache.clone();
    let ip_clone      = ip.clone();
    tokio::spawn(async move {
        let (country, org) = tokio::join!(
            lookup_country(&http_client, &country_cache, &ip_clone),
            lookup_org(&http_client, &org_cache, &ip_clone),
        );
        db.log_hit(slug, ip_clone, headers_json, body_str, country, org).await;
    });

    StatusCode::NOT_FOUND
}

// ---------------------------------------------------------------------------
// Dogbox Lite — temporary file sharing (5 GB max, 1 hour TTL)
// ---------------------------------------------------------------------------

/// Dogbox Lite page handler
pub async fn dogbox_lite_page(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    add_honeypot_urls(&mut context);
    let html = state
        .tera
        .render("dogbox_lite.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;
    Ok(Html(html))
}

/// Upload a file; returns a JSON object with the GUID
pub async fn dogbox_lite_upload(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<axum::Json<serde_json::Value>, AppError> {
    let mut file_data: Option<Vec<u8>> = None;
    let mut original_filename = "upload".to_string();
    let mut content_type = "application/octet-stream".to_string();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("Multipart error: {}", e)))?
    {
        if field.name() == Some("file") {
            if let Some(fname) = field.file_name() {
                original_filename = fname.to_string();
            }
            if let Some(ct) = field.content_type() {
                content_type = ct.to_string();
            }
            let data = field
                .bytes()
                .await
                .map_err(|e| AppError::InternalError(anyhow::anyhow!("Read error: {}", e)))?;
            file_data = Some(data.to_vec());
        }
    }

    let data = file_data
        .ok_or_else(|| AppError::InternalError(anyhow::anyhow!("No file provided")))?;

    let guid = state
        .dogbox_lite_manager
        .store_file(data, original_filename, content_type)
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("Storage error: {}", e)))?;

    Ok(axum::Json(serde_json::json!({ "guid": guid.to_string() })))
}

/// Download a file by GUID
pub async fn dogbox_lite_download(
    State(state): State<Arc<AppState>>,
    Path(guid): Path<String>,
) -> Result<Response, AppError> {
    let id = guid
        .parse::<uuid::Uuid>()
        .map_err(|_| AppError::NotFound)?;

    let info = state
        .dogbox_lite_manager
        .get_file(&id)
        .await
        .ok_or(AppError::NotFound)?;

    let file = File::open(&info.file_path)
        .await
        .map_err(|_| AppError::NotFound)?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let disposition = format!(
        "attachment; filename=\"{}\"",
        info.original_filename.replace('"', "")
    );

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        info.content_type.parse().unwrap_or_else(|_| {
            "application/octet-stream".parse().unwrap()
        }),
    );
    headers.insert(
        axum::http::header::CONTENT_DISPOSITION,
        disposition.parse().unwrap(),
    );

    Ok((headers, body).into_response())
}

// ---------------------------------------------------------------------------
// Doggypastebin — simple pastebin clone (30 day TTL)
// ---------------------------------------------------------------------------

/// Doggypastebin create-paste page handler
pub async fn doggypastebin_page(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    add_honeypot_urls(&mut context);
    let html = state
        .tera
        .render("doggypastebin.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;
    Ok(Html(html))
}

/// List all currently stored pastes
pub async fn doggypastebin_browse(
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, AppError> {
    let pastes = state.doggypastebin_manager.list_pastes().await;

    let mut context = Context::new();
    context.insert("pastes", &pastes);
    add_honeypot_urls(&mut context);
    let html = state
        .tera
        .render("doggypastebin_browse.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;
    Ok(Html(html))
}

/// Create a paste; returns a JSON object with the paste ID
pub async fn doggypastebin_create(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreatePasteRequest>,
) -> Result<Json<CreatePasteResponse>, AppError> {
    if req.content.trim().is_empty() {
        return Err(AppError::InternalError(anyhow::anyhow!(
            "Paste content cannot be empty"
        )));
    }

    let id = state
        .doggypastebin_manager
        .create_paste(req.content, req.language)
        .await
        .map_err(|e| AppError::InternalError(anyhow::anyhow!("Storage error: {}", e)))?;

    Ok(Json(CreatePasteResponse { id }))
}

/// View a paste, syntax-highlighted
pub async fn doggypastebin_view(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Html<String>, AppError> {
    let paste = state
        .doggypastebin_manager
        .get_paste(&id)
        .await
        .ok_or(AppError::NotFound)?;

    let highlighted = state
        .doggypastebin_manager
        .highlight(&paste.content, &paste.language);

    let mut context = Context::new();
    context.insert("id", &id);
    context.insert("language", &paste.language);
    context.insert("content_html", &highlighted);
    add_honeypot_urls(&mut context);

    let html = state
        .tera
        .render("doggypastebin_view.html", &context)
        .map_err(|e| AppError::TemplateError(e.to_string()))?;
    Ok(Html(html))
}

/// Fetch a paste's raw, unhighlighted content
pub async fn doggypastebin_raw(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Response, AppError> {
    let paste = state
        .doggypastebin_manager
        .get_paste(&id)
        .await
        .ok_or(AppError::NotFound)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        "text/plain; charset=utf-8".parse().unwrap(),
    );

    Ok((headers, paste.content).into_response())
}
