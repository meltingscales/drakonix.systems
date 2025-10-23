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
run:
    cargo run

# Run the web server in release mode (faster)
run-release:
    cargo run --release

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

# Security Scanning
# =================

# Run Trivy security scan on the Docker image
trivy-scan:
    #!/usr/bin/env bash
    echo "Building Docker image for scanning..."
    docker build -t rust-blog:scan .
    echo ""
    echo "Running Trivy vulnerability scan..."
    trivy image --severity HIGH,CRITICAL rust-blog:scan

# Run Trivy scan with all severity levels
trivy-scan-all:
    #!/usr/bin/env bash
    echo "Building Docker image for scanning..."
    docker build -t rust-blog:scan .
    echo ""
    echo "Running Trivy vulnerability scan (all severities)..."
    trivy image rust-blog:scan

# Run Trivy scan and save report to file
trivy-scan-report:
    #!/usr/bin/env bash
    echo "Building Docker image for scanning..."
    docker build -t rust-blog:scan .
    echo ""
    echo "Running Trivy vulnerability scan and saving report..."
    trivy image --severity HIGH,CRITICAL --format json --output trivy-report.json rust-blog:scan
    trivy image --severity HIGH,CRITICAL --format table --output trivy-report.txt rust-blog:scan
    echo "Reports saved to trivy-report.json and trivy-report.txt"

# Docker operations
# ================

# Build Docker image
docker-build:
    docker build -t rust-blog:latest .

# Build Docker image with a specific tag
docker-build-tag tag:
    docker build -t rust-blog:{{tag}} .

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
    docker build -t gcr.io/{{GCP_PROJECT}}/{{SERVICE_NAME}}:latest .
    docker push gcr.io/{{GCP_PROJECT}}/{{SERVICE_NAME}}:latest

# Build and push Docker image with a specific tag
gcp-push-tag tag:
    docker build -t gcr.io/{{GCP_PROJECT}}/{{SERVICE_NAME}}:{{tag}} .
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
    gcloud run domain-mappings list --region {{GCP_REGION}} --project {{GCP_PROJECT}}

# Get DNS records needed for domain verification
gcp-domain-records domain=DOMAIN_NAME:
    gcloud run domain-mappings describe {{domain}} --region {{GCP_REGION}} --project {{GCP_PROJECT}}

# Delete a domain mapping
gcp-domain-delete domain=DOMAIN_NAME:
    gcloud run domain-mappings delete {{domain}} --region {{GCP_REGION}} --project {{GCP_PROJECT}}

# Development helpers
# ===================

# Watch for changes and rebuild (requires cargo-watch)
watch:
    cargo watch -x run

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
