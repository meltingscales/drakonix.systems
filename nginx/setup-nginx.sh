#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOMAIN="drakonix.systems"
NGINX_CONF="${DOMAIN}.conf"

echo "=== drakonix.systems infrastructure setup ==="

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
apt-get update -qq
apt-get install -y -qq nginx certbot python3-certbot-nginx

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
        --non-interactive --agree-tos --register-unsafely-without-email || {
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
cp "${SCRIPT_DIR}/${NGINX_CONF}" "/etc/nginx/sites-available/${DOMAIN}.conf"
ln -sf "/etc/nginx/sites-available/${DOMAIN}.conf" "/etc/nginx/sites-enabled/${DOMAIN}.conf"

# Remove default site if it exists
rm -f /etc/nginx/sites-enabled/default

echo "Testing nginx config..."
nginx -t

# --- Enable and start nginx ---
echo ""
echo "[4/5] Starting nginx..."
systemctl enable nginx
systemctl restart nginx

# --- Firewall ---
echo ""
echo "[5/5] Configuring firewall (ufw)..."
if command -v ufw &>/dev/null; then
    ufw allow 22/tcp   comment 'SSH'    2>/dev/null || true
    ufw allow 80/tcp   comment 'HTTP'   2>/dev/null || true
    ufw allow 443/tcp  comment 'HTTPS'  2>/dev/null || true
    ufw --force enable
    echo "UFW rules applied."
else
    echo "ufw not found, skipping firewall setup."
    echo "Make sure ports 22, 80, and 443 are open."
fi

echo ""
echo "=== Setup complete ==="
echo ""
echo "Next steps:"
echo "  1. Make sure your router forwards ports 80 and 443 to this machine."
echo "  2. Point DNS for ${DOMAIN} to your public IP (or configure DDNS)."
echo "  3. Deploy your Rust services:"
echo "       :3000  main site (drakonix.systems/)"
echo "       :3001  meowderall (/dragonrouter/meowderall)"
echo "       :3002  carethermometer (/dragonrouter/carethermometer)"
echo "       :3003  donationaggregator (/dragonrouter/donationaggregator)"
echo "  4. To update nginx config later, just git pull and run:"
echo "       sudo cp nginx/${DOMAIN}.conf /etc/nginx/sites-available/"
echo "       sudo nginx -t && sudo systemctl reload nginx"
