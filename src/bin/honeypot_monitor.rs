use crossterm::event::{self, Event, KeyCode};
use futures::stream::{FuturesUnordered, StreamExt};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio::time::timeout;

const HOST: &str = "localhost";
const PORT: u16 = 8080;
const NUM_STREAMS: usize = 200;
const SAMPLE_SIZE: usize = 8192; // Sample 8KB to detect chaos mode
const LIVE_UPDATE_INTERVAL_MS: u64 = 1000; // How often to update live samples (1 second)

#[derive(Clone, Debug)]
struct StreamResult {
    id: usize,
    slug: String,
    is_chaos: bool,
    bytes_received: usize,
    duration: Duration,
    chaos_type: Option<String>,
    #[allow(dead_code)]
    sample: Vec<u8>,
}

#[derive(Clone, Debug)]
struct LiveStreamSample {
    sample: Vec<u8>,
    is_chaos: bool,
    chaos_type: Option<String>,
    bytes_so_far: usize,
}

#[derive(Clone, Debug)]
struct MonitorState {
    total: usize,
    completed: usize,
    html_count: usize,
    chaos_count: usize,
    chaos_streams: Vec<StreamResult>,
    active_streams: Vec<String>,
    start_time: Instant,
    latest_html: Option<StreamResult>,
    latest_chaos: Option<StreamResult>,
    total_bytes: usize,
    avg_bytes_per_stream: usize,
    live_html_sample: Option<LiveStreamSample>,
    live_chaos_sample: Option<LiveStreamSample>,
}

impl MonitorState {
    fn new(total: usize) -> Self {
        Self {
            total,
            completed: 0,
            html_count: 0,
            chaos_count: 0,
            chaos_streams: Vec::new(),
            active_streams: Vec::new(),
            start_time: Instant::now(),
            latest_html: None,
            latest_chaos: None,
            total_bytes: 0,
            avg_bytes_per_stream: 0,
            live_html_sample: None,
            live_chaos_sample: None,
        }
    }
}

fn format_hexdump(data: &[u8], max_lines: usize) -> Vec<String> {
    let bytes_per_line = 16;
    let max_bytes = max_lines * bytes_per_line;
    let data_slice = &data[..data.len().min(max_bytes)];

    let mut lines = Vec::new();
    for (i, chunk) in data_slice.chunks(bytes_per_line).enumerate() {
        let offset = i * bytes_per_line;

        // Hex part
        let hex: String = chunk
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|pair| pair.join(" "))
            .collect::<Vec<_>>()
            .join("  ");

        // ASCII part
        let ascii: String = chunk
            .iter()
            .map(|&b| {
                if b >= 32 && b <= 126 {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();

        // Pad hex part if needed
        let hex_padded = format!("{:48}", hex);

        lines.push(format!("{:08x}  {}  |{}|", offset, hex_padded, ascii));
    }

    lines
}

// Tail-following hexdump: shows the LAST max_lines of data (like tail -f)
fn format_hexdump_tail(data: &[u8], max_lines: usize) -> Vec<String> {
    let bytes_per_line = 16;
    let total_bytes = data.len();

    if total_bytes == 0 {
        return vec![];
    }

    // Calculate where to start to get the last max_lines
    let max_bytes = max_lines * bytes_per_line;
    let start_offset = if total_bytes > max_bytes {
        total_bytes - max_bytes
    } else {
        0
    };

    // Align to line boundary for cleaner output
    let aligned_offset = (start_offset / bytes_per_line) * bytes_per_line;
    let data_slice = &data[aligned_offset..];

    let mut lines = Vec::new();
    for (i, chunk) in data_slice.chunks(bytes_per_line).enumerate() {
        let offset = aligned_offset + (i * bytes_per_line);

        // Hex part
        let hex: String = chunk
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|pair| pair.join(" "))
            .collect::<Vec<_>>()
            .join("  ");

        // ASCII part
        let ascii: String = chunk
            .iter()
            .map(|&b| {
                if b >= 32 && b <= 126 {
                    b as char
                } else {
                    '.'
                }
            })
            .collect();

        // Pad hex part if needed
        let hex_padded = format!("{:48}", hex);

        lines.push(format!("{:08x}  {}  |{}|", offset, hex_padded, ascii));
    }

    lines
}

fn generate_honeypot_slug() -> String {
    let mut rng = rand::thread_rng();
    let prefixes = [
        "admin", "api", "internal", "private", "secret", "staging",
        "dev", "test", "backup", "config", "control", "dashboard",
    ];
    let suffixes = [
        "panel", "console", "manager", "portal", "system", "db",
        "redis", "mysql", "postgres", "jenkins", "gitlab", "aws",
    ];

    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let suffix = suffixes[rng.gen_range(0..suffixes.len())];
    let num = rng.gen_range(100..999);

    format!("{}-{}-{}", prefix, suffix, num)
}

fn detect_chaos_mode(data: &[u8]) -> (bool, Option<String>) {
    // Check if data looks like HTML
    if data.starts_with(b"<!DOCTYPE") || data.starts_with(b"<html") {
        return (false, None);
    }

    // Calculate entropy and non-printable ratio
    let non_printable = data.iter().filter(|&&b| b < 32 || b > 126).count();
    let non_printable_ratio = non_printable as f64 / data.len() as f64;

    // High non-printable ratio = likely binary/chaos
    if non_printable_ratio > 0.3 {
        // Try to detect chaos type
        let chaos_type = if data.iter().all(|&b| b.is_ascii_alphabetic() || b.is_ascii_whitespace()) {
            Some("Caesar Cipher".to_string())
        } else if data.windows(2).all(|w| w[0] ^ w[1] < 128) {
            Some("XOR Cipher".to_string())
        } else if non_printable_ratio > 0.7 {
            Some("/dev/urandom".to_string())
        } else {
            Some("Flawed AES-CBC".to_string())
        };

        return (true, chaos_type);
    }

    (false, None)
}

async fn fetch_stream(
    id: usize,
    state: Arc<RwLock<MonitorState>>,
    live_bytes: Arc<AtomicUsize>,
) -> StreamResult {
    let slug = generate_honeypot_slug();
    let path = format!("/api/markov-babble/{}/gen", slug);

    // Add to active streams
    {
        let mut s = state.write().await;
        s.active_streams.push(slug.clone());
    }

    let start = Instant::now();

    // Connect via raw TCP
    let result = match timeout(
        Duration::from_secs(5),
        TcpStream::connect(format!("{}:{}", HOST, PORT)),
    )
    .await
    {
        Ok(Ok(mut stream)) => {
            // Send HTTP GET request
            let request = format!(
                "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
                path, HOST
            );

            if let Err(_) = stream.write_all(request.as_bytes()).await {
                return StreamResult {
                    id,
                    slug: slug.clone(),
                    is_chaos: false,
                    bytes_received: 0,
                    duration: start.elapsed(),
                    chaos_type: None,
                    sample: Vec::new(),
                };
            }

            // Read response until EOF
            let mut total_bytes = 0;
            let mut sample = Vec::with_capacity(SAMPLE_SIZE);
            let mut buffer = vec![0u8; 4096];
            let mut header_done = false;
            let mut last_update = Instant::now();

            loop {
                match stream.read(&mut buffer).await {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        total_bytes += n;
                        // Update live byte counter atomically
                        live_bytes.fetch_add(n, Ordering::Relaxed);

                        if !header_done {
                            // Look for end of HTTP headers
                            if let Some(pos) = buffer[..n]
                                .windows(4)
                                .position(|w| w == b"\r\n\r\n")
                            {
                                header_done = true;
                                let body_start = pos + 4;
                                if sample.len() < SAMPLE_SIZE && body_start < n {
                                    let to_copy = (n - body_start).min(SAMPLE_SIZE - sample.len());
                                    sample.extend_from_slice(&buffer[body_start..body_start + to_copy]);
                                }
                            }
                        } else {
                            // Already past headers, collect sample
                            if sample.len() < SAMPLE_SIZE {
                                let to_copy = n.min(SAMPLE_SIZE - sample.len());
                                sample.extend_from_slice(&buffer[..to_copy]);
                            }
                        }

                        // Update live samples periodically (once per second)
                        if last_update.elapsed().as_millis() > LIVE_UPDATE_INTERVAL_MS as u128 && sample.len() >= 256 {
                            let (is_chaos, chaos_type) = detect_chaos_mode(&sample);
                            let live_sample = LiveStreamSample {
                                sample: sample.clone(),
                                is_chaos,
                                chaos_type: chaos_type.clone(),
                                bytes_so_far: total_bytes,
                            };

                            let mut s = state.write().await;
                            if is_chaos {
                                s.live_chaos_sample = Some(live_sample);
                            } else {
                                s.live_html_sample = Some(live_sample);
                            }
                            last_update = Instant::now();
                        }
                    }
                    Err(_) => break,
                }
            }

            let (is_chaos, chaos_type) = detect_chaos_mode(&sample);
            StreamResult {
                id,
                slug: slug.clone(),
                is_chaos,
                bytes_received: total_bytes,
                duration: start.elapsed(),
                chaos_type,
                sample,
            }
        }
        _ => StreamResult {
            id,
            slug: slug.clone(),
            is_chaos: false,
            bytes_received: 0,
            duration: start.elapsed(),
            chaos_type: None,
            sample: Vec::new(),
        },
    };

    // Update state
    {
        let mut s = state.write().await;
        s.completed += 1;
        s.active_streams.retain(|x| x != &slug);
        s.total_bytes += result.bytes_received;

        // Update running average
        if s.completed > 0 {
            s.avg_bytes_per_stream = s.total_bytes / s.completed;
        }

        if result.is_chaos {
            s.chaos_count += 1;
            s.chaos_streams.push(result.clone());
            s.latest_chaos = Some(result.clone());
        } else {
            s.html_count += 1;
            s.latest_html = Some(result.clone());
        }
    }

    result
}

fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    state: &MonitorState,
    live_bytes: &AtomicUsize,
) -> io::Result<()> {
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),   // Title
                Constraint::Length(9),   // Stats (added bandwidth line)
                Constraint::Min(14),     // Chaos list
                Constraint::Length(12),  // Hexdumps
            ])
            .split(f.area());

        // Title
        let title = Paragraph::new("🍯 Honeypot Stream Monitor (press 'q' to quit)")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Stats
        let elapsed = state.start_time.elapsed().as_secs_f64();
        let progress = (state.completed as f64 / state.total as f64) * 100.0;
        let rate = if elapsed > 0.0 {
            state.completed as f64 / elapsed
        } else {
            0.0
        };

        // Use live atomic byte counter for real-time updates
        let total_bytes_live = live_bytes.load(Ordering::Relaxed);
        let bandwidth_kbps = if elapsed > 0.0 {
            (total_bytes_live as f64 / 1024.0) / elapsed
        } else {
            0.0
        };
        let total_mb = total_bytes_live as f64 / (1024.0 * 1024.0);

        let stats_text = vec![
            Line::from(vec![
                Span::styled("Progress: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{}/{} ({:.1}%)", state.completed, state.total, progress),
                    Style::default().fg(Color::Green),
                ),
            ]),
            Line::from(vec![
                Span::styled("HTML Responses: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{}", state.html_count),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![
                Span::styled("🎲 CHAOS MODE: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::styled(
                    format!("{}", state.chaos_count),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Data consumed: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{:.2} MB @ {:.1} KB/s", total_mb, bandwidth_kbps),
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Rate: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{:.1} streams/sec", rate),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(vec![
                Span::styled("Elapsed: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{:.1}s", elapsed),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![
                Span::styled("Active: ", Style::default().fg(Color::Cyan)),
                Span::styled(
                    format!("{}", state.active_streams.len()),
                    Style::default().fg(Color::Magenta),
                ),
            ]),
        ];

        let stats = Paragraph::new(stats_text)
            .block(Block::default().borders(Borders::ALL).title("Statistics"));
        f.render_widget(stats, chunks[1]);

        // Chaos mode streams
        let chaos_items: Vec<ListItem> = state
            .chaos_streams
            .iter()
            .map(|stream| {
                let chaos_label = stream.chaos_type.as_ref().map(|s| s.as_str()).unwrap_or("Unknown");
                let content = format!(
                    "Stream #{} [{}] - {} - {}KB in {:.1}s",
                    stream.id,
                    stream.slug,
                    chaos_label,
                    stream.bytes_received / 1024,
                    stream.duration.as_secs_f64(),
                );
                ListItem::new(content).style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            })
            .collect();

        let chaos_list = List::new(chaos_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("🎲 Chaos Mode Streams (Binary/Encrypted)")
                    .title_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            );
        f.render_widget(chaos_list, chunks[2]);

        // Hexdump panels - split horizontally
        let hexdump_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[3]);

        // Latest HTML hexdump - prefer live streaming data
        let html_hex_lines: Vec<Line> = if let Some(ref live) = state.live_html_sample {
            let mut lines = vec![Line::from(Span::styled(
                format!("🔴 LIVE - {:.1} KB streaming... (tail view)", live.bytes_so_far as f64 / 1024.0),
                Style::default()
                    .fg(Color::LightGreen)
                    .add_modifier(Modifier::BOLD),
            ))];
            // Use tail hexdump to show the LAST bytes (like tail -f)
            lines.extend(
                format_hexdump_tail(&live.sample, 7)
                    .into_iter()
                    .map(|s| Line::from(Span::styled(s, Style::default().fg(Color::Green)))),
            );
            lines
        } else if let Some(ref html_stream) = state.latest_html {
            let mut lines = vec![Line::from(Span::styled(
                format!("✓ Completed - {} bytes", html_stream.bytes_received),
                Style::default().fg(Color::DarkGray),
            ))];
            lines.extend(
                format_hexdump(&html_stream.sample, 7)
                    .into_iter()
                    .map(|s| Line::from(Span::styled(s, Style::default().fg(Color::Green)))),
            );
            lines
        } else {
            vec![Line::from(Span::styled(
                "Waiting for HTML streams...",
                Style::default().fg(Color::DarkGray),
            ))]
        };

        let html_hexdump = Paragraph::new(html_hex_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("📄 HTML Stream (hexdump)")
                .title_style(Style::default().fg(Color::Green)),
        );
        f.render_widget(html_hexdump, hexdump_chunks[0]);

        // Latest Chaos hexdump - prefer live streaming data
        let chaos_hex_lines: Vec<Line> = if let Some(ref live) = state.live_chaos_sample {
            let chaos_label = live.chaos_type.as_ref().map(|s| s.as_str()).unwrap_or("Unknown");
            let mut lines = vec![Line::from(Span::styled(
                format!("🔴 LIVE - {} - {:.1} KB streaming... (tail view)", chaos_label, live.bytes_so_far as f64 / 1024.0),
                Style::default()
                    .fg(Color::LightRed)
                    .add_modifier(Modifier::BOLD),
            ))];
            // Use tail hexdump to show the LAST bytes (like tail -f)
            lines.extend(
                format_hexdump_tail(&live.sample, 7)
                    .into_iter()
                    .map(|s| Line::from(Span::styled(s, Style::default().fg(Color::Red)))),
            );
            lines
        } else if let Some(ref chaos_stream) = state.latest_chaos {
            let chaos_label = chaos_stream
                .chaos_type
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("Unknown");
            let mut lines = vec![Line::from(Span::styled(
                format!("✓ {} - {} bytes", chaos_label, chaos_stream.bytes_received),
                Style::default().fg(Color::DarkGray),
            ))];
            lines.extend(
                format_hexdump(&chaos_stream.sample, 7)
                    .into_iter()
                    .map(|s| Line::from(Span::styled(s, Style::default().fg(Color::Red)))),
            );
            lines
        } else {
            vec![Line::from(Span::styled(
                "Waiting for chaos mode...",
                Style::default().fg(Color::DarkGray),
            ))]
        };

        let chaos_hexdump = Paragraph::new(chaos_hex_lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title("🎲 Chaos Stream (hexdump)")
                .title_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        );
        f.render_widget(chaos_hexdump, hexdump_chunks[1]);
    })?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let state = Arc::new(RwLock::new(MonitorState::new(NUM_STREAMS)));
    let live_bytes = Arc::new(AtomicUsize::new(0));

    // Spawn all stream fetchers
    let mut futures = FuturesUnordered::new();
    for i in 0..NUM_STREAMS {
        let state_clone = Arc::clone(&state);
        let bytes_clone = Arc::clone(&live_bytes);
        futures.push(tokio::spawn(fetch_stream(i, state_clone, bytes_clone)));
    }

    // UI update loop with keyboard handling
    let ui_state = Arc::clone(&state);
    let ui_bytes = Arc::clone(&live_bytes);
    let mut ui_interval = tokio::time::interval(Duration::from_millis(100));

    loop {
        tokio::select! {
            _ = ui_interval.tick() => {
                // Check for keyboard input (non-blocking)
                if event::poll(Duration::from_millis(0)).unwrap_or(false) {
                    if let Ok(Event::Key(key)) = event::read() {
                        if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                            // User pressed 'q' - quit
                            break;
                        }
                    }
                }

                let state_snapshot = ui_state.read().await.clone();
                if let Err(e) = draw_ui(&mut terminal, &state_snapshot, &ui_bytes) {
                    eprintln!("UI error: {}", e);
                    break;
                }

                if state_snapshot.completed >= state_snapshot.total {
                    // One final draw and break
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    let _ = draw_ui(&mut terminal, &state_snapshot, &ui_bytes);
                    break;
                }
            }
            Some(_) = futures.next() => {
                // Stream completed
            }
            else => {
                // All streams completed
                break;
            }
        }
    }

    // Cleanup terminal
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    // Print final summary
    let final_state = state.read().await;
    println!("\n🎉 Monitoring Complete!");
    println!("═══════════════════════════════════════");
    println!("Total streams: {}", final_state.total);
    println!("HTML responses: {}", final_state.html_count);
    println!("🎲 Chaos mode: {}", final_state.chaos_count);
    println!("Chaos rate: {:.2}%", (final_state.chaos_count as f64 / final_state.total as f64) * 100.0);
    println!("Duration: {:.1}s", final_state.start_time.elapsed().as_secs_f64());

    if !final_state.chaos_streams.is_empty() {
        println!("\n🎲 Chaos Mode Streams:");
        for stream in &final_state.chaos_streams {
            println!(
                "  - Stream #{}: {} [{}]",
                stream.id,
                stream.slug,
                stream.chaos_type.as_ref().unwrap_or(&"Unknown".to_string())
            );
        }
    }

    Ok(())
}
