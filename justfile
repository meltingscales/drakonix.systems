# List available recipes
help:
    @just --list

# Build the project in release mode
build:
    cargo build --release

# Run all tests
test:
    cargo test

# Run the web server locally (dev mode with hot reload)
# Streams markov babble 10x faster for development
run:
    MARKOV_STREAM_SPEED_MULTIPLIER=10.0 cargo run --bin rust-blog

# Run the web server in release mode (faster)
run-release:
    cargo run --release --bin rust-blog

# Format Rust code
fmt:
    cargo fmt

# Run clippy linter
lint:
    cargo clippy -- -D warnings

# Show version info that will be embedded in the build
version-info:
    @echo "Git Commit:  $(git rev-parse --short HEAD)"
    @echo "Git Branch:  $(git rev-parse --abbrev-ref HEAD)"
    @echo "Build Date:  $(date -u +%Y-%m-%dT%H:%M:%SZ)"

# Clean up generated files and build artifacts
clean:
    cargo clean

# Clear cargo cache (frees disk space, keeps target/ intact)
clean-cache:
    #!/usr/bin/env bash
    echo "Clearing cargo cache..."
    rm -rf ~/.cargo/registry/src ~/.cargo/registry/index ~/.cargo/git/db
    echo "Cache cleared. Previous builds in target/ preserved."

# Security Scanning
# =================

# Run Trivy security scan on the Docker image
trivy-scan:
    #!/usr/bin/env bash
    echo "Building Docker image for scanning..."
    docker build --network=host -t rust-blog:scan .
    echo ""
    echo "Running Trivy vulnerability scan..."
    trivy image --severity HIGH,CRITICAL rust-blog:scan

# Run Trivy scan with all severity levels
trivy-scan-all:
    #!/usr/bin/env bash
    echo "Building Docker image for scanning..."
    docker build --network=host -t rust-blog:scan .
    echo ""
    echo "Running Trivy vulnerability scan (all severities)..."
    trivy image rust-blog:scan

# Run Trivy scan and save report to file
trivy-scan-report:
    #!/usr/bin/env bash
    echo "Building Docker image for scanning..."
    docker build --network=host -t rust-blog:scan .
    echo ""
    echo "Running Trivy vulnerability scan and saving report..."
    trivy image --severity HIGH,CRITICAL --format json --output trivy-report.json rust-blog:scan
    trivy image --severity HIGH,CRITICAL --format table --output trivy-report.txt rust-blog:scan
    echo "Reports saved to trivy-report.json and trivy-report.txt"

# Docker operations
# ================

# Build Docker image
docker-build:
    docker build --network=host -t rust-blog:latest .

# Build Docker image with a specific tag
docker-build-tag tag:
    docker build --network=host -t rust-blog:{{tag}} .

# Run Docker container locally
docker-run port="8080":
    docker run -p {{port}}:8080 -v $(pwd)/content:/app/content rust-blog:latest

# Stop all running containers for this project
docker-stop:
    docker ps -q --filter ancestor=rust-blog:latest | xargs -r docker stop

# Remove Docker image
docker-clean:
    docker rmi rust-blog:latest

# GCP Deployment
# ==============

# Set these variables for your GCP project
GCP_PROJECT := env_var_or_default("GCP_PROJECT", "personal-site-meltingscales")
GCP_REGION := env_var_or_default("GCP_REGION", "us-central1")
SERVICE_NAME := "rust-blog"
DOMAIN_NAME := env_var_or_default("DOMAIN_NAME", "blog.example.com")

# Build and push Docker image to Google Container Registry
gcp-push:
    docker build --network=host -t gcr.io/{{GCP_PROJECT}}/{{SERVICE_NAME}}:latest .
    docker push gcr.io/{{GCP_PROJECT}}/{{SERVICE_NAME}}:latest

# Build and push Docker image with a specific tag
gcp-push-tag tag:
    docker build --network=host -t gcr.io/{{GCP_PROJECT}}/{{SERVICE_NAME}}:{{tag}} .
    docker push gcr.io/{{GCP_PROJECT}}/{{SERVICE_NAME}}:{{tag}}

# Deploy to Google Cloud Run
gcp-deploy:
    gcloud run deploy {{SERVICE_NAME}} \
        --image gcr.io/{{GCP_PROJECT}}/{{SERVICE_NAME}}:latest \
        --platform managed \
        --region {{GCP_REGION}} \
        --allow-unauthenticated \
        --port 8080 \
        --memory 512Mi \
        --cpu 1 \
        --project {{GCP_PROJECT}}

# Deploy a specific tagged version to Cloud Run
gcp-deploy-tag tag:
    gcloud run deploy {{SERVICE_NAME}} \
        --image gcr.io/{{GCP_PROJECT}}/{{SERVICE_NAME}}:{{tag}} \
        --platform managed \
        --region {{GCP_REGION}} \
        --allow-unauthenticated \
        --port 8080 \
        --memory 512Mi \
        --cpu 1 \
        --project {{GCP_PROJECT}}

# Build, push, and deploy to GCP in one command
gcp-deploy-all:
    just gcp-push
    just gcp-deploy

# View Cloud Run service logs
gcp-logs:
    gcloud run services logs read {{SERVICE_NAME}} --region {{GCP_REGION}} --project {{GCP_PROJECT}}

# Get Cloud Run service URL
gcp-url:
    gcloud run services describe {{SERVICE_NAME}} --region {{GCP_REGION}} --project {{GCP_PROJECT}} --format 'value(status.url)'

# Domain Management
# =================

# Map a custom domain to the Cloud Run service
gcp-domain-map domain=DOMAIN_NAME:
    gcloud run domain-mappings create \
        --service {{SERVICE_NAME}} \
        --domain {{domain}} \
        --region {{GCP_REGION}} \
        --project {{GCP_PROJECT}}

# List all domain mappings
gcp-domain-list:
    gcloud beta run domain-mappings list --region {{GCP_REGION}} --project {{GCP_PROJECT}}

# Get DNS records needed for domain verification
gcp-domain-records domain=DOMAIN_NAME:
    gcloud run domain-mappings describe {{domain}} --region {{GCP_REGION}} --project {{GCP_PROJECT}}

# Delete a domain mapping
gcp-domain-delete domain=DOMAIN_NAME:
    gcloud run domain-mappings delete {{domain}} --region {{GCP_REGION}} --project {{GCP_PROJECT}}

# Set up drakonix.systems domain
gcp-setup-drakonix:
    gcloud beta run domain-mappings create \
        --service {{SERVICE_NAME}} \
        --domain drakonix.systems \
        --region {{GCP_REGION}} \
        --platform managed \
        --project {{GCP_PROJECT}}

# Set up www.drakonix.systems subdomain
gcp-setup-drakonix-www:
    gcloud beta run domain-mappings create \
        --service {{SERVICE_NAME}} \
        --domain www.drakonix.systems \
        --region {{GCP_REGION}} \
        --platform managed \
        --project {{GCP_PROJECT}}

# Systemd Service Setup (for GCP VM)
# ====================================

# Install as systemd service running on port 3000
# Run with sudo
systemd-install:
    #!/usr/bin/env bash
    set -euo pipefail

    if [[ $EUID -ne 0 ]]; then
        echo "Error: This recipe must be run as root (use sudo)."
        exit 1
    fi

    # Use current directory as REPO_DIR (user should run from repo root)
    REPO_DIR="$(pwd)"
    SERVICE_NAME="drakonix-systems"
    PORT="${PORT:-3000}"
    USER="${SUDO_USER:-root}"

    echo "Installing systemd service: ${SERVICE_NAME}"

    # Build release binary first (skip if already built)
    if [[ ! -f "${REPO_DIR}/target/release/rust-blog" ]]; then
        echo "Building release binary..."
        (cd "${REPO_DIR}" && cargo build --release)
    else
        echo "Binary already exists, skipping build."
    fi

    # Copy and template service file
    sed -e "s|USER_PLACEHOLDER|${USER}|g" \
        -e "s|REPO_DIR_PLACEHOLDER|${REPO_DIR}|g" \
        "${REPO_DIR}/systemd/${SERVICE_NAME}.service" \
        > /etc/systemd/system/${SERVICE_NAME}.service

    # Reload systemd and enable service
    systemctl daemon-reload
    systemctl enable ${SERVICE_NAME}
    systemctl restart ${SERVICE_NAME}

    echo "Service installed and started!"
    echo ""
    echo "Commands:"
    echo "  sudo systemctl status ${SERVICE_NAME}"
    echo "  sudo systemctl restart ${SERVICE_NAME}"
    echo "  sudo journalctl -u ${SERVICE_NAME} -f"

# Uninstall systemd service
# Run with sudo
systemd-uninstall:
    #!/usr/bin/env bash
    SERVICE_NAME="drakonix-systems"

    if [[ $EUID -ne 0 ]]; then
        echo "Error: This recipe must be run as root (use sudo)."
        exit 1
    fi

    echo "Stopping and disabling ${SERVICE_NAME}..."
    systemctl stop ${SERVICE_NAME} 2>/dev/null || true
    systemctl disable ${SERVICE_NAME} 2>/dev/null || true
    rm -f /etc/systemd/system/${SERVICE_NAME}.service
    systemctl daemon-reload
    echo "Service uninstalled."

# Show service status
systemd-status:
    #!/usr/bin/env bash
    SERVICE_NAME="${SERVICE_NAME:-drakonix-systems}"
    systemctl status ${SERVICE_NAME}

# View service logs
systemd-logs:
    #!/usr/bin/env bash
    SERVICE_NAME="${SERVICE_NAME:-drakonix-systems}"
    journalctl -u ${SERVICE_NAME} -f

# Reload templates + content without recompiling (just restarts the service)
# Tera loads templates from disk at startup, so a restart is all that's needed.
reload:
    #!/usr/bin/env bash
    SERVICE_NAME="${SERVICE_NAME:-drakonix-systems}"
    systemctl restart ${SERVICE_NAME}
    systemctl status ${SERVICE_NAME}

# Restart the service
systemd-restart:
    #!/usr/bin/env bash
    SERVICE_NAME="${SERVICE_NAME:-drakonix-systems}"
    systemctl restart ${SERVICE_NAME}
    systemctl status ${SERVICE_NAME}

# Nginx Access Logs
# =================

# Tail all vhost access logs live
nginx-logs:
    sudo tail -f /var/log/nginx/*drakonix.systems.access.log

# Tail access logs for a specific vhost (e.g. just nginx-logs-vhost meowderall.drakonix.systems)
nginx-logs-vhost vhost:
    sudo tail -f /var/log/nginx/{{vhost}}.access.log

# Development helpers
# ===================

# Watch for changes and rebuild (requires cargo-watch)
watch:
    cargo watch -x run

# Monitor honeypot streams in real-time with TUI (requires local server running)
monitor-honeypot:
    cargo run --bin honeypot_monitor

# Create a new blog post
new-post title:
    #!/usr/bin/env bash
    slug=$(echo "{{title}}" | tr '[:upper:]' '[:lower:]' | tr ' ' '-')
    date=$(date +%Y-%m-%d)
    filename="content/posts/${date}-${slug}.md"
    cat > "$filename" << EOF
    ---
    title: "{{title}}"
    date: $(date -u +%Y-%m-%dT%H:%M:%SZ)
    draft: false
    tags: []
    ---

    Your content here...
    EOF
    echo "Created: $filename"

# Create a new page
new-page title:
    #!/usr/bin/env bash
    slug=$(echo "{{title}}" | tr '[:upper:]' '[:lower:]' | tr ' ' '-')
    filename="content/pages/${slug}.md"
    cat > "$filename" << EOF
    ---
    title: "{{title}}"
    date: $(date -u +%Y-%m-%dT%H:%M:%SZ)
    ---

    Your content here...
    EOF
    echo "Created: $filename"
