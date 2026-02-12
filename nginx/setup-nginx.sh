#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOMAIN="drakonix.systems"
NGINX_CONF="${DOMAIN}.conf"

echo "=== drakonix.systems infrastructure setup (Debian 12) ==="

# --- Preflight checks ---
if [[ $EUID -ne 0 ]]; then
    echo "Error: This script must be run as root (use sudo)."
    exit 1
fi

if [[ ! -f "${SCRIPT_DIR}/${NGINX_CONF}" ]]; then
    echo "Error: ${NGINX_CONF} not found. Run this from the repo root."
    exit 1
fi

# --- Install dependencies ---
echo ""
echo "[1/5] Installing nginx and certbot..."
apt-get update
apt-get install -y nginx certbot

# --- TLS certs (before nginx config, since config references certs) ---
echo ""
echo "[2/5] Setting up TLS certificates..."
if [[ -d "/etc/letsencrypt/live/${DOMAIN}" ]]; then
    echo "Certs already exist for ${DOMAIN}, skipping."
else
    echo "Obtaining certs via certbot standalone..."
    echo "NOTE: Port 80 must be reachable from the internet for this to work."

    # Stop nginx if running so certbot can bind to port 80
    systemctl stop nginx 2>/dev/null || true

    certbot certonly --standalone -d "${DOMAIN}" \
        --non-interactive --agree-tos || {
        echo ""
        echo "WARNING: Certbot failed. You can retry manually with:"
        echo "  sudo certbot certonly --standalone -d ${DOMAIN}"
        echo ""
        echo "Continuing setup without TLS for now..."
        echo "Nginx will fail to start until certs are in place."
    }
fi

# --- Deploy nginx config ---
echo ""
echo "[3/5] Deploying nginx config..."
mkdir -p /etc/nginx/conf.d
cp "${SCRIPT_DIR}/${NGINX_CONF}" "/etc/nginx/conf.d/${DOMAIN}.conf"

# Remove default site configs (Debian default)
rm -f /etc/nginx/conf.d/default.conf
rm -f /etc/nginx/sites-enabled/default 2>/dev/null || true

echo "Testing nginx config..."
nginx -t

# --- Enable and start nginx ---
echo ""
echo "[4/5] Starting nginx..."
systemctl enable nginx
systemctl restart nginx

# --- Firewall ---
echo ""
echo "[5/5] Configuring firewall..."
if command -v firewall-cmd &>/dev/null; then
    # firewalld
    firewall-cmd --permanent --add-service=ssh 2>/dev/null || true
    firewall-cmd --permanent --add-service=http 2>/dev/null || true
    firewall-cmd --permanent --add-service=https 2>/dev/null || true
    firewall-cmd --reload 2>/dev/null || true
    echo "firewalld rules applied."
elif command -v ufw &>/dev/null; then
    # ufw (sometimes used on Arch)
    ufw allow 22/tcp   comment 'SSH'    2>/dev/null || true
    ufw allow 80/tcp   comment 'HTTP'   2>/dev/null || true
    ufw allow 443/tcp  comment 'HTTPS'  2>/dev/null || true
    ufw --force enable
    echo "UFW rules applied."
else
    echo "No firewall detected (firewalld/ufw not found)."
    echo "Make sure ports 22, 80, and 443 are open."
fi

echo ""
echo "=== Setup complete ==="
echo ""
echo "Next steps:"
echo "  1. Configure GCP firewall to allow HTTP (80) and HTTPS (443) traffic:"
echo "       gcloud compute firewall-rules create allow-http --allow tcp:80"
echo "       gcloud compute firewall-rules create allow-https --allow tcp:443"
echo "  2. Deploy your Rust services:"
echo "       :3000  main site (drakonix.systems/)"
echo "       :3001  meowderall (/dragonrouter/meowderall)"
echo "       :3002  carethermometer (/dragonrouter/carethermometer)"
echo "       :3003  donationaggregator (/dragonrouter/donationaggregator)"
echo "  3. To update nginx config later, just git pull and run:"
echo "       sudo cp nginx/${DOMAIN}.conf /etc/nginx/conf.d/"
echo "       sudo nginx -t && sudo systemctl reload nginx"
