// Renders live Minecraft server status into any element with
// [data-mc-status]. Value "compact" renders a terse inline list (for the
// homepage); anything else renders full cards (for the minecraft-servers
// services page). Data comes from /api/mc-status, which is cached
// server-side for a minute, so polling here is cheap.
(function () {
  function escapeHtml(str) {
    const div = document.createElement("div");
    div.textContent = str == null ? "" : str;
    return div.innerHTML;
  }

  function renderCompact(container, servers) {
    container.innerHTML = servers
      .map((s) => {
        const dot = s.online ? "🟢" : "🔴";
        const players = s.online
          ? `${s.players_online}/${s.players_max} players`
          : "offline";
        return `<li>${dot} <strong>${escapeHtml(s.name)}</strong> — ${escapeHtml(players)}</li>`;
      })
      .join("");
  }

  function renderFull(container, servers) {
    container.innerHTML = servers
      .map((s) => {
        const favicon = s.favicon
          ? `<img src="${s.favicon}" alt="${escapeHtml(s.name)} banner" class="mc-status-favicon">`
          : "";
        const body = s.online
          ? `<p>${escapeHtml(s.motd || "")}</p>
             <p>${s.players_online}/${s.players_max} players &middot; ${escapeHtml(s.version || "unknown version")}</p>`
          : `<p>Offline${s.error ? " — " + escapeHtml(s.error) : ""}</p>`;
        const dot = s.online ? "🟢" : "🔴";
        return `<div class="mc-status-card">
          ${favicon}
          <div class="mc-status-info">
            <h3>${dot} ${escapeHtml(s.name)}</h3>
            <p class="mc-status-address"><code>${escapeHtml(s.address)}</code></p>
            ${body}
          </div>
        </div>`;
      })
      .join("");
  }

  function init() {
    const containers = document.querySelectorAll("[data-mc-status]");
    if (!containers.length) return;

    fetch("/api/mc-status")
      .then((r) => r.json())
      .then((servers) => {
        containers.forEach((el) => {
          if (el.dataset.mcStatus === "compact") {
            renderCompact(el, servers);
          } else {
            renderFull(el, servers);
          }
        });
      })
      .catch(() => {
        containers.forEach((el) => {
          el.innerHTML = "<p>Couldn't load server status.</p>";
        });
      });
  }

  document.addEventListener("DOMContentLoaded", init);
})();
