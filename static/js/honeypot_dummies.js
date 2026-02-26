// ── Entry point ───────────────────────────────────────────────────────────────

(async () => {
  const root    = document.getElementById("hp-root");
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
    root.innerHTML = `<p class="hp-empty">No hits recorded yet. The honeypot is waiting…</p>`;
    return;
  }

  countEl.textContent = `${hits.length} hit(s) recorded.`;
  injectStyles();

  // Pre-compute indices (UTC dates throughout)
  const byDate     = {};  // "YYYY-MM-DD"   → [hit, …]
  const byDateHour = {};  // "YYYY-MM-DD-H" → [hit, …]
  for (const hit of hits) {
    const d   = hit.timestamp.slice(0, 10);
    const h   = new Date(hit.timestamp).getUTCHours();
    const key = `${d}-${h}`;
    if (!byDate[d])     byDate[d]     = [];
    if (!byDateHour[key]) byDateHour[key] = [];
    byDate[d].push(hit);
    byDateHour[key].push(hit);
  }

  root.appendChild(buildWeeklyHeatmap(byDate));
  root.appendChild(buildDailyHeatmap(byDate, byDateHour));
  root.appendChild(buildHourlyPattern(byDateHour));
  root.appendChild(buildTopStats(hits));
  root.appendChild(buildTable(hits));
})();

// ── 52-week calendar heatmap ──────────────────────────────────────────────────

function buildWeeklyHeatmap(byDate) {
  const counts = {};
  for (const [d, arr] of Object.entries(byDate)) counts[d] = arr.length;
  const maxCount = Math.max(1, ...Object.values(counts));

  // Grid starts on the Sunday 52 weeks back
  const today = new Date();
  today.setHours(0, 0, 0, 0);
  const start = new Date(today);
  start.setDate(start.getDate() - 52 * 7 + 1);
  start.setDate(start.getDate() - start.getDay());

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

  // Month label spans
  const monthSpans = [];
  let lastMonth = -1;
  for (const week of weeks) {
    const m = new Date(week[0].date + "T12:00:00").getMonth();
    if (m !== lastMonth) { monthSpans.push({ label: MONTHS[m], cols: 1 }); lastMonth = m; }
    else monthSpans[monthSpans.length - 1].cols++;
  }

  const wrap = el("div", "hp-hm-wrap");
  wrap.appendChild(sectionTitle("Hit activity — last 52 weeks"));

  // Month row
  const monthRow = el("div", "hp-hm-months");
  const spacer = document.createElement("div");
  spacer.style.cssText = "width:28px;flex-shrink:0";
  monthRow.appendChild(spacer);
  for (const span of monthSpans) {
    const lbl = el("div", "hp-hm-month");
    lbl.style.width = span.cols * 13 + "px";
    lbl.textContent = span.label;
    monthRow.appendChild(lbl);
  }
  wrap.appendChild(monthRow);

  // Body: day labels + grid
  const body = el("div", "hp-hm-body");

  const dayCol = el("div", "hp-hm-days");
  for (let d = 0; d < 7; d++) {
    const lbl = el("div", "hp-hm-day");
    if (d === 1 || d === 3 || d === 5) lbl.textContent = DAYS[d];
    dayCol.appendChild(lbl);
  }
  body.appendChild(dayCol);

  const grid = el("div", "hp-hm-grid");
  for (const week of weeks) {
    const col = el("div", "hp-hm-col");
    for (const day of week) {
      const cell = el("div", "hp-hm-cell");
      cell.style.backgroundColor = heatColor(day.count, maxCount);
      const dayHits = byDate[day.date] || [];
      cell.addEventListener("mouseenter", e => showTooltip(e, ttDate(day.date, dayHits)));
      cell.addEventListener("mouseleave", hideTooltip);
      col.appendChild(cell);
    }
    grid.appendChild(col);
  }
  body.appendChild(grid);
  wrap.appendChild(body);

  // Legend
  const legend = el("div", "hp-hm-legend");
  const less = document.createElement("span"); less.textContent = "Less"; legend.appendChild(less);
  for (let i = 0; i <= 4; i++) {
    const c = el("div", "hp-hm-cell");
    c.style.backgroundColor = `var(--hp-heat-${i})`;
    legend.appendChild(c);
  }
  const more = document.createElement("span"); more.textContent = "More"; legend.appendChild(more);
  wrap.appendChild(legend);

  return wrap;
}

// ── Day × hour heatmap (pageable) ─────────────────────────────────────────────

function buildDailyHeatmap(byDate, byDateHour) {
  if (Object.keys(byDate).length === 0) return document.createDocumentFragment();

  // Full continuous range: first hit date → today
  const today = new Date(); today.setHours(0, 0, 0, 0);
  const allDates = [];
  const rangeStart = new Date(Object.keys(byDate).sort()[0] + "T12:00:00");
  for (const d = new Date(rangeStart); d <= today; d.setDate(d.getDate() + 1))
    allDates.push(toDateStr(d));

  const PAGE_SIZE = 14;
  let pageIdx = Math.max(0, allDates.length - PAGE_SIZE);

  const wrap = el("div", "hp-dh-wrap");
  wrap.appendChild(sectionTitle("Hourly breakdown (UTC)"));

  // Nav
  const nav      = el("div",    "hp-dh-nav");
  const prevBtn  = el("button", "hp-dh-btn"); prevBtn.textContent = "← Prev";
  const nextBtn  = el("button", "hp-dh-btn"); nextBtn.textContent = "Next →";
  const navLabel = el("span",   "hp-dh-nav-label");
  nav.appendChild(prevBtn); nav.appendChild(navLabel); nav.appendChild(nextBtn);
  wrap.appendChild(nav);

  const gridWrap = el("div", "hp-dh-grid-wrap");
  wrap.appendChild(gridWrap);

  function render() {
    const page = allDates.slice(pageIdx, pageIdx + PAGE_SIZE);
    navLabel.textContent = `${fmtDateShort(page[0])} – ${fmtDateShort(page[page.length - 1])}`;
    prevBtn.disabled = pageIdx === 0;
    nextBtn.disabled = pageIdx + PAGE_SIZE >= allDates.length;

    // Color scale relative to this page's max
    let maxCount = 1;
    for (const date of page)
      for (let h = 0; h < 24; h++) {
        const c = (byDateHour[`${date}-${h}`] || []).length;
        if (c > maxCount) maxCount = c;
      }

    gridWrap.innerHTML = "";
    const grid = el("div", "hp-dh-grid");

    // Hour header
    const headerRow = el("div", "hp-dh-row");
    headerRow.appendChild(el("div", "hp-dh-date-lbl")); // corner spacer
    for (let h = 0; h < 24; h++) {
      const lbl = el("div", "hp-dh-hour-lbl");
      lbl.textContent = h % 6 === 0 ? String(h).padStart(2, "0") : "";
      headerRow.appendChild(lbl);
    }
    grid.appendChild(headerRow);

    // Data rows
    for (const date of page) {
      const row    = el("div", "hp-dh-row");
      const dateLbl = el("div", "hp-dh-date-lbl");
      dateLbl.textContent = fmtDateRow(date);
      row.appendChild(dateLbl);

      for (let h = 0; h < 24; h++) {
        const cellHits = byDateHour[`${date}-${h}`] || [];
        const cell = el("div", "hp-dh-cell");
        cell.style.backgroundColor = heatColor(cellHits.length, maxCount);
        cell.addEventListener("mouseenter", e => showTooltip(e, ttHour(date, h, cellHits)));
        cell.addEventListener("mouseleave", hideTooltip);
        row.appendChild(cell);
      }
      grid.appendChild(row);
    }
    gridWrap.appendChild(grid);
  }

  prevBtn.addEventListener("click", () => { pageIdx = Math.max(0, pageIdx - PAGE_SIZE); render(); });
  nextBtn.addEventListener("click", () => { pageIdx = Math.min(allDates.length - PAGE_SIZE, pageIdx + PAGE_SIZE); render(); });
  render();
  return wrap;
}

// ── 24-hour aggregate distribution ───────────────────────────────────────────

function buildHourlyPattern(byDateHour) {
  const totals = new Array(24).fill(0);
  for (const [key, arr] of Object.entries(byDateHour)) {
    const h = parseInt(key.split("-").pop(), 10);
    totals[h] += arr.length;
  }
  const maxTotal = Math.max(1, ...totals);

  const wrap  = el("div", "hp-hourly-wrap");
  wrap.appendChild(sectionTitle("Hit distribution by hour (UTC, all time)"));

  const chart = el("div", "hp-hourly-chart");
  for (let h = 0; h < 24; h++) {
    const count  = totals[h];
    const col    = el("div", "hp-hourly-col");
    const bar    = el("div", "hp-hourly-bar");
    const lbl    = el("div", "hp-hourly-lbl");
    bar.style.height          = `${Math.max(2, (count / maxTotal) * 80)}px`;
    bar.style.backgroundColor = heatColor(count, maxTotal);
    lbl.textContent = h % 6 === 0 ? String(h).padStart(2, "0") : "";
    const html = `<div class="hp-tt-header">${String(h).padStart(2,"0")}:00 UTC</div>` +
                 `<div class="hp-tt-count">${count} hit${count !== 1 ? "s" : ""}</div>`;
    col.addEventListener("mouseenter", e => showTooltip(e, html));
    col.addEventListener("mouseleave", hideTooltip);
    col.appendChild(bar);
    col.appendChild(lbl);
    chart.appendChild(col);
  }
  wrap.appendChild(chart);
  return wrap;
}

// ── Top IPs + top slugs ───────────────────────────────────────────────────────

function buildTopStats(hits) {
  const ipCounts   = {};
  const slugCounts = {};
  for (const hit of hits) {
    ipCounts[hit.ip]     = (ipCounts[hit.ip]     || 0) + 1;
    slugCounts[hit.slug] = (slugCounts[hit.slug] || 0) + 1;
  }

  const topIPs   = Object.entries(ipCounts).sort((a, b) => b[1] - a[1]).slice(0, 10);
  const topSlugs = Object.entries(slugCounts).sort((a, b) => b[1] - a[1]).slice(0, 10);

  const wrap = el("div", "hp-stats-grid");
  wrap.appendChild(buildBarChart("Top IPs", topIPs,
    lbl => `<a href="https://ipinfo.io/${escHtml(lbl)}" target="_blank" rel="noopener noreferrer">${escHtml(lbl)}</a>`
  ));
  wrap.appendChild(buildBarChart("Top slugs", topSlugs,
    lbl => `<code>${escHtml(lbl)}</code>`
  ));
  return wrap;
}

function buildBarChart(title, entries, labelFn) {
  const maxVal = Math.max(1, ...entries.map(([, v]) => v));
  const wrap = el("div", "hp-bar-wrap");
  wrap.appendChild(sectionTitle(title));
  for (const [label, count] of entries) {
    const row   = el("div", "hp-bar-row");
    const lbl   = el("div", "hp-bar-label"); lbl.innerHTML = labelFn(label);
    const track = el("div", "hp-bar-track");
    const fill  = el("div", "hp-bar-fill");  fill.style.width = `${(count / maxVal) * 100}%`;
    const cnt   = el("div", "hp-bar-count"); cnt.textContent = count;
    track.appendChild(fill);
    row.appendChild(lbl); row.appendChild(track); row.appendChild(cnt);
    wrap.appendChild(row);
  }
  return wrap;
}

// ── Hits table ────────────────────────────────────────────────────────────────

function buildTable(hits) {
  const wrap  = el("div", "hp-table-wrap");
  const table = el("table", "hp-table");
  table.innerHTML = `<thead><tr>
    <th>#</th><th>Slug</th><th>IP</th><th>Timestamp</th><th>Headers</th>
  </tr></thead>`;
  const tbody = document.createElement("tbody");
  for (const hit of hits) {
    let pretty = hit.headers;
    try { pretty = JSON.stringify(JSON.parse(hit.headers), null, 2); } catch (_) {}
    const tr = document.createElement("tr");
    tr.innerHTML = `
      <td class="hp-id">${hit.id}</td>
      <td class="hp-slug"><code>${escHtml(hit.slug)}</code></td>
      <td class="hp-ip"><a href="https://ipinfo.io/${escHtml(hit.ip)}" target="_blank" rel="noopener noreferrer"><code>${escHtml(hit.ip)}</code></a></td>
      <td class="hp-ts">${escHtml(hit.timestamp)}</td>
      <td class="hp-headers"><details><summary>show</summary><pre class="hp-json">${escHtml(pretty)}</pre></details></td>`;
    tbody.appendChild(tr);
  }
  table.appendChild(tbody);
  wrap.appendChild(table);
  return wrap;
}

// ── Tooltip ───────────────────────────────────────────────────────────────────

let _tt = null;
function getTooltip() {
  if (_tt) return _tt;
  _tt = el("div", "hp-tt");
  _tt.style.display = "none";
  document.body.appendChild(_tt);
  document.addEventListener("mousemove", e => {
    if (_tt.style.display === "none") return;
    // Flip tooltip left if it would overflow the right edge
    const tw = _tt.offsetWidth || 200;
    const x  = e.clientX + 14 + tw > window.innerWidth
                 ? e.clientX - tw - 6 : e.clientX + 14;
    _tt.style.left = x + "px";
    _tt.style.top  = (e.clientY + 14) + "px";
  });
  return _tt;
}
function showTooltip(e, html) {
  const tt = getTooltip();
  tt.innerHTML = html;
  tt.style.display = "block";
  tt.style.left = (e.clientX + 14) + "px";
  tt.style.top  = (e.clientY + 14) + "px";
}
function hideTooltip() { getTooltip().style.display = "none"; }

function ttDate(date, hits) {
  const ipCounts = ipCounter(hits);
  const top      = Object.entries(ipCounts).sort((a, b) => b[1] - a[1]).slice(0, 4);
  const extra    = Object.keys(ipCounts).length - top.length;
  return `<div class="hp-tt-header">${escHtml(date)}</div>` +
         `<div class="hp-tt-count">${hits.length} hit${hits.length !== 1 ? "s" : ""}</div>` +
         (top.length ? `<div class="hp-tt-ips">${top.map(([ip, c]) =>
           `<div class="hp-tt-ip">${escHtml(ip)}${c > 1 ? ` <span class="hp-tt-x">×${c}</span>` : ""}</div>`
         ).join("")}${extra > 0 ? `<div class="hp-tt-ip hp-tt-more">…and ${extra} more</div>` : ""}</div>` : "");
}

function ttHour(date, hour, hits) {
  const h0 = String(hour).padStart(2, "0");
  const h1 = String((hour + 1) % 24).padStart(2, "0");
  const ipCounts = ipCounter(hits);
  const top      = Object.entries(ipCounts).sort((a, b) => b[1] - a[1]).slice(0, 4);
  const extra    = Object.keys(ipCounts).length - top.length;
  return `<div class="hp-tt-header">${escHtml(date)} &nbsp; ${h0}:00–${h1}:00 UTC</div>` +
         `<div class="hp-tt-count">${hits.length} hit${hits.length !== 1 ? "s" : ""}</div>` +
         (top.length ? `<div class="hp-tt-ips">${top.map(([ip, c]) =>
           `<div class="hp-tt-ip">${escHtml(ip)}${c > 1 ? ` <span class="hp-tt-x">×${c}</span>` : ""}</div>`
         ).join("")}${extra > 0 ? `<div class="hp-tt-ip hp-tt-more">…and ${extra} more</div>` : ""}</div>` : "");
}

// ── Helpers ───────────────────────────────────────────────────────────────────

function el(tag, cls) {
  const e = document.createElement(tag);
  if (cls) e.className = cls;
  return e;
}
function sectionTitle(text) {
  const d = el("div", "hp-sec-title"); d.textContent = text; return d;
}
function ipCounter(hits) {
  const c = {};
  for (const h of hits) c[h.ip] = (c[h.ip] || 0) + 1;
  return c;
}
function toDateStr(date) {
  return `${date.getFullYear()}-${String(date.getMonth()+1).padStart(2,"0")}-${String(date.getDate()).padStart(2,"0")}`;
}
function escHtml(str) {
  return String(str).replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;").replace(/"/g,"&quot;");
}
function heatColor(count, max) {
  if (count === 0) return "var(--hp-heat-0)";
  const p = count / max;
  if (p < 0.25) return "var(--hp-heat-1)";
  if (p < 0.50) return "var(--hp-heat-2)";
  if (p < 0.75) return "var(--hp-heat-3)";
  return "var(--hp-heat-4)";
}
const MONTHS = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
const DAYS   = ["Sun","Mon","Tue","Wed","Thu","Fri","Sat"];
function fmtDateShort(ds) {
  const d = new Date(ds + "T12:00:00");
  return `${d.getDate()} ${MONTHS[d.getMonth()]} ${d.getFullYear()}`;
}
function fmtDateRow(ds) {
  const d = new Date(ds + "T12:00:00");
  return `${DAYS[d.getDay()]} ${d.getDate()} ${MONTHS[d.getMonth()]}`;
}

// ── Styles ────────────────────────────────────────────────────────────────────

function injectStyles() {
  if (document.getElementById("hp-styles")) return;
  const s = el("style"); s.id = "hp-styles";
  s.textContent = `
    :root {
      --hp-heat-0: var(--bg-tertiary, #2d333b);
      --hp-heat-1: #0e4429; --hp-heat-2: #006d32;
      --hp-heat-3: #26a641; --hp-heat-4: #39d353;
    }
    @media (prefers-color-scheme: light) {
      :root {
        --hp-heat-0: #ebedf0; --hp-heat-1: #9be9a8;
        --hp-heat-2: #40c463; --hp-heat-3: #30a14e; --hp-heat-4: #216e39;
      }
    }

    /* shared */
    .hp-hm-wrap,.hp-dh-wrap,.hp-hourly-wrap,.hp-stats-grid,.hp-table-wrap { margin-bottom: 2.5rem; }
    .hp-sec-title { font-size: 1rem; font-weight: 600; margin-bottom: 0.6rem; }

    /* 52-week heatmap */
    .hp-hm-wrap    { overflow-x: auto; }
    .hp-hm-months  { display: flex; align-items: flex-end; margin-bottom: 3px; }
    .hp-hm-month   { font-size: .72rem; color: var(--text-secondary); overflow: hidden; white-space: nowrap; flex-shrink: 0; }
    .hp-hm-body    { display: flex; }
    .hp-hm-days    { display: flex; flex-direction: column; gap: 2px; margin-right: 4px; width: 24px; flex-shrink: 0; }
    .hp-hm-day     { font-size: .68rem; color: var(--text-secondary); height: 11px; line-height: 11px; white-space: nowrap; }
    .hp-hm-grid    { display: flex; gap: 2px; }
    .hp-hm-col     { display: flex; flex-direction: column; gap: 2px; }
    .hp-hm-cell    { width: 11px; height: 11px; border-radius: 2px; flex-shrink: 0; cursor: default; }
    .hp-hm-legend  { display: flex; align-items: center; gap: 3px; margin-top: 8px; font-size: .72rem; color: var(--text-secondary); }
    .hp-hm-legend span { margin: 0 2px; }

    /* daily heatmap */
    .hp-dh-nav         { display: flex; align-items: center; gap: .75rem; margin-bottom: .6rem; flex-wrap: wrap; }
    .hp-dh-btn         { padding: .3rem .8rem; border: 1px solid var(--border-primary); border-radius: 4px; background: var(--bg-secondary); color: var(--text-primary); font-size: .82rem; cursor: pointer; }
    .hp-dh-btn:disabled { opacity: .35; cursor: default; }
    .hp-dh-btn:not(:disabled):hover { background: var(--bg-hover); }
    .hp-dh-nav-label   { font-size: .85rem; color: var(--text-secondary); }
    .hp-dh-grid-wrap   { overflow-x: auto; }
    .hp-dh-grid        { display: inline-flex; flex-direction: column; gap: 2px; }
    .hp-dh-row         { display: flex; align-items: center; gap: 2px; }
    .hp-dh-date-lbl    { width: 88px; font-size: .68rem; color: var(--text-secondary); flex-shrink: 0; white-space: nowrap; }
    .hp-dh-hour-lbl    { width: 14px; height: 14px; font-size: .6rem; color: var(--text-secondary); text-align: center; line-height: 14px; flex-shrink: 0; }
    .hp-dh-cell        { width: 14px; height: 14px; border-radius: 2px; flex-shrink: 0; cursor: default; }

    /* hourly pattern */
    .hp-hourly-wrap  { }
    .hp-hourly-chart { display: flex; align-items: flex-end; gap: 3px; height: 100px; }
    .hp-hourly-col   { display: flex; flex-direction: column; align-items: center; justify-content: flex-end; gap: 3px; cursor: default; }
    .hp-hourly-bar   { width: 20px; border-radius: 2px 2px 0 0; min-height: 2px; flex-shrink: 0; transition: opacity .1s; }
    .hp-hourly-col:hover .hp-hourly-bar { opacity: .75; }
    .hp-hourly-lbl   { font-size: .6rem; color: var(--text-secondary); height: 12px; line-height: 12px; }

    /* top stats */
    .hp-stats-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 2rem; }
    @media (max-width: 600px) { .hp-stats-grid { grid-template-columns: 1fr; } }
    .hp-bar-row    { display: flex; align-items: center; gap: .5rem; margin-bottom: .35rem; font-size: .82rem; }
    .hp-bar-label  { width: 130px; flex-shrink: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
    .hp-bar-track  { flex: 1; height: 12px; background: var(--bg-tertiary, #2d333b); border-radius: 2px; overflow: hidden; }
    .hp-bar-fill   { height: 100%; background: var(--hp-heat-3); border-radius: 2px; transition: width .4s ease; }
    .hp-bar-count  { width: 36px; text-align: right; flex-shrink: 0; color: var(--text-secondary); font-size: .78rem; }

    /* tooltip */
    .hp-tt {
      position: fixed; pointer-events: none; z-index: 9999;
      background: var(--bg-secondary, #1c2128); border: 1px solid var(--border-primary, #444c56);
      border-radius: 6px; padding: .5rem .65rem; font-size: .8rem;
      min-width: 140px; max-width: 260px; box-shadow: 0 4px 12px rgba(0,0,0,.45);
    }
    .hp-tt-header { font-weight: 700; margin-bottom: .2rem; white-space: nowrap; }
    .hp-tt-count  { color: var(--hp-heat-4); font-size: .88rem; margin-bottom: .3rem; }
    .hp-tt-ips    { display: flex; flex-direction: column; gap: 1px; }
    .hp-tt-ip     { font-family: monospace; font-size: .75rem; color: var(--text-secondary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
    .hp-tt-more   { font-style: italic; }
    .hp-tt-x      { color: var(--hp-heat-3); margin-left: 3px; }
  `;
  document.head.appendChild(s);
}
