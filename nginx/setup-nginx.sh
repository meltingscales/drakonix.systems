#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DOMAIN="drakonix.systems"
NGINX_CONF="nginx/${DOMAIN}.conf"

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

# --- Deploy nginx config ---
echo ""
echo "[2/5] Deploying nginx config..."
cp "${SCRIPT_DIR}/${NGINX_CONF}" "/etc/nginx/sites-available/${DOMAIN}.conf"
ln -sf "/etc/nginx/sites-available/${DOMAIN}.conf" "/etc/nginx/sites-enabled/${DOMAIN}.conf"

# Remove default site if it exists
rm -f /etc/nginx/sites-enabled/default

echo "Testing nginx config..."
nginx -t

# --- TLS certs ---
echo ""
echo "[3/5] Setting up TLS certificates..."
if [[ -d "/etc/letsencrypt/live/${DOMAIN}" ]]; then
    echo "Certs already exist for ${DOMAIN}, skipping."
else
    echo "Obtaining certs via certbot..."
    echo "NOTE: Port 80 must be reachable from the internet for this to work."
    # Temporarily start nginx without SSL for the ACME challenge
    # Comment out the SSL server block won't work, so use standalone mode
    certbot certonly --nginx -d "${DOMAIN}" --non-interactive --agree-tos --register-unsafely-without-email || {
        echo ""
        echo "Certbot failed. You can retry manually with:"
        echo "  sudo certbot --nginx -d ${DOMAIN}"
        echo ""
        echo "Continuing setup without TLS for now..."
    }
fi

# --- Enable and start nginx ---
echo ""
echo "[4/5] Starting nginx..."
systemctl enable nginx
systemctl reload nginx

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
echo "  2. Point DNS for ${DOMAIN} to your public IP."
echo "  3. Deploy your Rust services on ports 3000 and 3001."
echo "     (See systemd/ directory for service unit files if available.)"
echo "  4. To update nginx config later, just git pull and run:"
echo "       sudo cp nginx/${DOMAIN}.conf /etc/nginx/sites-available/"
echo "       sudo nginx -t && sudo systemctl reload nginx"
