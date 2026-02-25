# drakonix.systems

My personal website

## Subdomain list

- [drakonix.systems](//drakonix.systems): {[github](https://github.com/meltingscales/drakonix.systems)} blog and micro utils
- [donationaggregator.drakonix.systems](//donationaggregator.drakonix.systems) {[github](https://github.com/meltingscales/CAREShelterDonationDataAggregation)} animal shelter data aggregator
- [meowderall.drakonix.systems](//meowderall.drakonix.systems) {[github](https://github.com/meltingscales/Meowderall)} cat med tracker
- [carethermometer.drakonix.systems](//carethermometer.drakonix.systems) {[github](https://github.com/meltingscales/animal-shelter-donation-thermometer)} donation thermometer

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
- Ideally 60GB or more of disk space (Rust/Cargo dependencies). 20GB is minimum.

### Development

```bash
# Run server locally
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

#### Docker Build Optimization

The Dockerfile uses a multi-stage build with dependency caching to speed up builds:

1. **Dependency Cache Layer**: Builds dependencies first with dummy source files
2. **Source Build Layer**: Rebuilds only when source code changes

**Cache Management**:

To clear Docker build cache when needed:
```bash
# Clear all build cache
docker builder prune -af

# Clear cache for specific image
docker build --no-cache -t your-image-name .

# Remove all unused images and containers
docker system prune -af
```

**When to clear cache**:
- After updating Cargo.toml dependencies
- When builds are failing with stale artifacts
- To free up disk space

### Google Cloud Run

```bash
# Set your GCP project
export GCP_PROJECT="your-project-id"

# Build, push, and deploy
just gcp-deploy-all

# Get the deployed URL
just gcp-url
```

### GCP VM Deployment (Subdomains)

Deploy all 4 services to a GCP e2-micro VM with nginx. Each service gets its own subdomain.

**DNS Setup** - Add A records for each subdomain pointing to the same VM IP:

```bash
# Point all subdomains to your VM
drakonix.systems              → 34.132.91.229
donationaggregator.drakonix.systems → 34.132.91.229
carethermometer.drakonix.systems   → 34.132.91.229
meowderall.drakonix.systems        → 34.132.91.229
```

**On VM** - Build and install each service:

```bash
# drakonix.systems (main site, port 3000)
cd ~/Git/drakonix.systems
cargo build --release
sudo just systemd-install

# Meowderall (Elm SPA, port 3001)
cd ~/Git/Meowderall
just build-release
sudo just systemd-install

# CAREShelter Thermometer (port 3002)
cd ~/Git/animal-shelter-donation-thermometer
cargo build --release
sudo just systemd-install

# CAREShelter Donation Aggregation (port 3003)
cd ~/Git/CAREShelterDonationDataAggregation
cargo build --release
sudo just systemd-install
```

Verify all 4 services are running:

```bash
sudo systemctl status drakonix-systems meowderall care-shelter-donation animal-shelter-thermometer
```

**nginx Config** - Copy and reload nginx config:

```bash
cd ~/Git/drakonix.systems
sudo cp nginx/drakonix.systems.conf /etc/nginx/conf.d/
sudo nginx -t && sudo systemctl reload nginx
```

**Manage services**:

```bash
# Restart a service
sudo just systemd-restart

# View logs
sudo just systemd-logs

# Uninstall a service
sudo just systemd-uninstall
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

Edit `static/css/style.css` to customize look. The default is a minimal reset style - add your own theme!

### Templates

Templates are in `templates/` and use Tera syntax (similar to Jinja2):
- `base.html` - Base template
- `index.html` - Home page
- `posts_list.html` - All posts listing
- `post_detail.html` - Individual post
- `page_detail.html` - Individual page

### Search

The search JavaScript is in `static/js/search.js` and uses `/search.json` endpoint for search index.

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
