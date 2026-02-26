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

  root.appendChild(buildHeatmap(hits));
  root.appendChild(buildTable(hits));
})();

// ── Heatmap ────────────────────────────────────────────────────────────────

function buildHeatmap(hits) {
  injectHeatmapStyles();

  // date string (YYYY-MM-DD) → hit count
  const counts = {};
  for (const hit of hits) {
    const d = hit.timestamp.slice(0, 10);
    counts[d] = (counts[d] || 0) + 1;
  }
  const maxCount = Math.max(1, ...Object.values(counts));

  // date range: last 52 weeks, grid starts on the Sunday before that
  const today = new Date();
  today.setHours(0, 0, 0, 0);
  const start = new Date(today);
  start.setDate(start.getDate() - 52 * 7 + 1);
  start.setDate(start.getDate() - start.getDay()); // rewind to Sunday

  // build week columns
  const weeks = [];
  const cur = new Date(start);
  while (cur <= today) {
    const week = [];
    for (let d = 0; d < 7; d++) {
      const ds = toDateStr(cur);
      week.push({ date: ds, count: counts[ds] || 0 });
      cur.setDate(cur.getDate() + 1);
    }
    weeks.push(week);
  }

  // group weeks into month spans for labels
  const MONTHS = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
  const monthSpans = [];
  let lastMonth = -1;
  for (const week of weeks) {
    const m = new Date(week[0].date).getMonth();
    if (m !== lastMonth) {
      monthSpans.push({ label: MONTHS[m], cols: 1 });
      lastMonth = m;
    } else {
      monthSpans[monthSpans.length - 1].cols++;
    }
  }

  function cellColor(count) {
    if (count === 0) return "var(--hp-heat-0)";
    const pct = count / maxCount;
    if (pct < 0.25) return "var(--hp-heat-1)";
    if (pct < 0.50) return "var(--hp-heat-2)";
    if (pct < 0.75) return "var(--hp-heat-3)";
    return "var(--hp-heat-4)";
  }

  // ── DOM ──

  const wrap = document.createElement("div");
  wrap.className = "hp-hm-wrap";

  const title = document.createElement("div");
  title.className = "hp-hm-title";
  title.textContent = "Hit activity — last 52 weeks";
  wrap.appendChild(title);

  // month label row
  const monthRow = document.createElement("div");
  monthRow.className = "hp-hm-months";
  // leading spacer for the day-label column
  const spacer = document.createElement("div");
  spacer.style.cssText = "width:28px;flex-shrink:0";
  monthRow.appendChild(spacer);
  for (const span of monthSpans) {
    const lbl = document.createElement("div");
    lbl.className = "hp-hm-month";
    lbl.style.width = span.cols * 13 + "px"; // 11px cell + 2px gap
    lbl.textContent = span.label;
    monthRow.appendChild(lbl);
  }
  wrap.appendChild(monthRow);

  // body: day labels + cell grid
  const body = document.createElement("div");
  body.className = "hp-hm-body";

  const DAY_LABELS = ["Sun","Mon","Tue","Wed","Thu","Fri","Sat"];
  const dayCol = document.createElement("div");
  dayCol.className = "hp-hm-days";
  for (let d = 0; d < 7; d++) {
    const lbl = document.createElement("div");
    lbl.className = "hp-hm-day";
    if (d === 1 || d === 3 || d === 5) lbl.textContent = DAY_LABELS[d];
    dayCol.appendChild(lbl);
  }
  body.appendChild(dayCol);

  const grid = document.createElement("div");
  grid.className = "hp-hm-grid";
  for (const week of weeks) {
    const col = document.createElement("div");
    col.className = "hp-hm-col";
    for (const day of week) {
      const cell = document.createElement("div");
      cell.className = "hp-hm-cell";
      cell.style.backgroundColor = cellColor(day.count);
      cell.title = `${day.date}: ${day.count} hit${day.count !== 1 ? "s" : ""}`;
      col.appendChild(cell);
    }
    grid.appendChild(col);
  }
  body.appendChild(grid);
  wrap.appendChild(body);

  // legend
  const legend = document.createElement("div");
  legend.className = "hp-hm-legend";
  const less = document.createElement("span");
  less.textContent = "Less";
  legend.appendChild(less);
  for (let i = 0; i <= 4; i++) {
    const cell = document.createElement("div");
    cell.className = "hp-hm-cell";
    cell.style.backgroundColor = `var(--hp-heat-${i})`;
    legend.appendChild(cell);
  }
  const more = document.createElement("span");
  more.textContent = "More";
  legend.appendChild(more);
  wrap.appendChild(legend);

  return wrap;
}

function injectHeatmapStyles() {
  if (document.getElementById("hp-hm-style")) return;
  const s = document.createElement("style");
  s.id = "hp-hm-style";
  s.textContent = `
    :root {
      --hp-heat-0: var(--bg-tertiary, #2d333b);
      --hp-heat-1: #0e4429;
      --hp-heat-2: #006d32;
      --hp-heat-3: #26a641;
      --hp-heat-4: #39d353;
    }
    @media (prefers-color-scheme: light) {
      :root {
        --hp-heat-0: #ebedf0;
        --hp-heat-1: #9be9a8;
        --hp-heat-2: #40c463;
        --hp-heat-3: #30a14e;
        --hp-heat-4: #216e39;
      }
    }
    .hp-hm-wrap { margin-bottom: 2rem; overflow-x: auto; }
    .hp-hm-title { font-size: 1rem; font-weight: 600; margin-bottom: 0.6rem; }
    .hp-hm-months { display: flex; align-items: flex-end; margin-bottom: 3px; }
    .hp-hm-month {
      font-size: 0.72rem;
      color: var(--text-secondary);
      overflow: hidden;
      white-space: nowrap;
      flex-shrink: 0;
    }
    .hp-hm-body { display: flex; gap: 0; }
    .hp-hm-days {
      display: flex;
      flex-direction: column;
      gap: 2px;
      margin-right: 4px;
      width: 24px;
      flex-shrink: 0;
    }
    .hp-hm-day {
      font-size: 0.68rem;
      color: var(--text-secondary);
      height: 11px;
      line-height: 11px;
      white-space: nowrap;
    }
    .hp-hm-grid { display: flex; gap: 2px; }
    .hp-hm-col { display: flex; flex-direction: column; gap: 2px; }
    .hp-hm-cell { width: 11px; height: 11px; border-radius: 2px; flex-shrink: 0; }
    .hp-hm-legend {
      display: flex;
      align-items: center;
      gap: 3px;
      margin-top: 8px;
      font-size: 0.72rem;
      color: var(--text-secondary);
    }
    .hp-hm-legend span { margin: 0 2px; }
  `;
  document.head.appendChild(s);
}

// ── Table ──────────────────────────────────────────────────────────────────

function buildTable(hits) {
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
    let prettyHeaders = hit.headers;
    try { prettyHeaders = JSON.stringify(JSON.parse(hit.headers), null, 2); } catch (_) {}

    const tr = document.createElement("tr");
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
  return wrap;
}

// ── Helpers ────────────────────────────────────────────────────────────────

function toDateStr(date) {
  // Local YYYY-MM-DD without UTC shift
  const y = date.getFullYear();
  const m = String(date.getMonth() + 1).padStart(2, "0");
  const d = String(date.getDate()).padStart(2, "0");
  return `${y}-${m}-${d}`;
}

function escHtml(str) {
  return String(str)
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}
