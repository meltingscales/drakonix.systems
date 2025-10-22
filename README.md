# Rust Blog

A blazingly fast blog engine powered by Rust, featuring server-side rendering, syntax highlighting, RSS feeds, and client-side search.

## Features

- **Server-Side Rendering**: Dynamic pages rendered with Tera templates
- **Markdown Support**: Write posts in Markdown with YAML frontmatter
- **Syntax Highlighting**: Beautiful code highlighting with syntect
- **RSS Feed**: Auto-generated RSS feed at `/rss.xml`
- **Client-Side Search**: Fast, instant search across all content
- **Docker Ready**: Deploy to Google Cloud Run or any container platform
- **Hot Reload**: Development mode with automatic rebuilds (with cargo-watch)

## Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- just ([Install just](https://github.com/casey/just#installation))

### Development

```bash
# Run the server locally
just run

# Or with hot reload (requires cargo-watch)
just watch
```

Visit `http://localhost:8080`

### Creating Content

```bash
# Create a new blog post
just new-post "My Awesome Post"

# Create a new page
just new-page "About Me"
```

Posts go in `content/posts/` and pages in `content/pages/`.

### Content Format

Blog posts use YAML frontmatter:

```markdown
---
title: "My Post Title"
date: 2025-01-15T12:00:00Z
draft: false
tags: ["rust", "web"]
---

Your content here...
```

## Deployment

### Docker

```bash
# Build Docker image
just docker-build

# Run locally with Docker
just docker-run

# Or run on custom port
just docker-run 3000
```

### Google Cloud Run

```bash
# Set your GCP project
export GCP_PROJECT="your-project-id"

# Build, push, and deploy
just gcp-deploy-all

# Get the deployed URL
just gcp-url
```

### Environment Variables

- `PORT`: Server port (default: 8080)
- `BASE_URL`: Your site URL for RSS feed (default: https://example.com)
- `RUST_LOG`: Log level (default: info)

## Project Structure

```
.
├── content/
│   ├── posts/          # Blog posts (.md files)
│   └── pages/          # Static pages (.md files)
├── templates/          # Tera HTML templates
├── static/
│   ├── css/           # Stylesheets
│   └── js/            # JavaScript
├── src/
│   ├── main.rs        # Server entry point
│   ├── handlers.rs    # Route handlers
│   ├── markdown.rs    # Markdown processing
│   ├── models.rs      # Data models
│   └── rss.rs         # RSS feed generation
├── Dockerfile         # Docker container config
├── justfile           # Task runner commands
└── Cargo.toml         # Rust dependencies
```

## Customization

### Styling

Edit `static/css/style.css` to customize the look. The default is a minimal reset style - add your own theme!

### Templates

Templates are in `templates/` and use Tera syntax (similar to Jinja2):

- `base.html` - Base template
- `index.html` - Home page
- `posts_list.html` - All posts listing
- `post_detail.html` - Individual post
- `page_detail.html` - Individual page

### Search

The search JavaScript is in `static/js/search.js` and uses the `/search.json` endpoint for the search index.

## Available Commands

See all commands:
```bash
just help
```

Key commands:
- `just run` - Run server locally
- `just build` - Build release binary
- `just test` - Run tests
- `just fmt` - Format code
- `just lint` - Run clippy
- `just docker-build` - Build Docker image
- `just gcp-deploy-all` - Deploy to Google Cloud Run

## License

MIT

## Contributing

PRs welcome!
