(async () => {
  const root = document.getElementById("hp-root");
  const countEl = document.getElementById("hp-count");

  let hits;
  try {
    const res = await fetch("/api/honeypot/hits");
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    hits = await res.json();
  } catch (err) {
    root.innerHTML = `<p class="hp-empty">Failed to load hits: ${err.message}</p>`;
    return;
  }

  if (!hits || hits.length === 0) {
    root.innerHTML = `<p class="hp-empty">No hits recorded yet. The honeypot is waiting...</p>`;
    return;
  }

  countEl.textContent = `${hits.length} hit(s) recorded.`;

  const wrap = document.createElement("div");
  wrap.className = "hp-table-wrap";

  const table = document.createElement("table");
  table.className = "hp-table";
  table.innerHTML = `
    <thead>
      <tr>
        <th>#</th>
        <th>Slug</th>
        <th>IP</th>
        <th>Timestamp</th>
        <th>Headers</th>
      </tr>
    </thead>`;

  const tbody = document.createElement("tbody");

  for (const hit of hits) {
    const tr = document.createElement("tr");

    // Pretty-print the JSON headers blob
    let prettyHeaders = hit.headers;
    try {
      prettyHeaders = JSON.stringify(JSON.parse(hit.headers), null, 2);
    } catch (_) {}

    tr.innerHTML = `
      <td class="hp-id">${hit.id}</td>
      <td class="hp-slug"><code>${escHtml(hit.slug)}</code></td>
      <td class="hp-ip">
        <a href="https://ipinfo.io/${escHtml(hit.ip)}" target="_blank" rel="noopener noreferrer">
          <code>${escHtml(hit.ip)}</code>
        </a>
      </td>
      <td class="hp-ts">${escHtml(hit.timestamp)}</td>
      <td class="hp-headers">
        <details>
          <summary>show</summary>
          <pre class="hp-json">${escHtml(prettyHeaders)}</pre>
        </details>
      </td>`;

    tbody.appendChild(tr);
  }

  table.appendChild(tbody);
  wrap.appendChild(table);
  root.appendChild(wrap);
})();

function escHtml(str) {
  return String(str)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}
