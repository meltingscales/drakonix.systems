// ── Entry point ───────────────────────────────────────────────────────────────

const _root          = document.getElementById("hp-root");
const _countEl       = document.getElementById("hp-count");
const _subtitleEl    = document.getElementById("hp-subtitle");
const _filterEl      = document.getElementById("hp-filter");
const _lastUpdatedEl = document.getElementById("hp-last-updated");
let   _refreshTimer  = null;
let   _dt            = null;

// Wire up filter, debounced — each keystroke now costs a server round-trip
// (search runs server-side against the DB), not a free in-memory re-filter.
let _filterDebounce = null;
_filterEl.addEventListener("input", () => {
  clearTimeout(_filterDebounce);
  _filterDebounce = setTimeout(applyFilter, 300);
});

// Wire up auto-refresh selector
document.getElementById("hp-refresh-select").addEventListener("change", e => {
  clearInterval(_refreshTimer);
  const secs = parseInt(e.target.value, 10);
  if (secs > 0) _refreshTimer = setInterval(doRefresh, secs * 1000);
});

// Column index → DB column name, matching honeypot_db::SORTABLE_COLUMNS. Must
// stay in sync with the <th> order built in buildTableShell().
const SORT_COLUMNS = ["id", "slug", "ip", "country", "org", "timestamp"];

// Initial load: fetch config once, build the (empty) table shell + server-side
// DataTable, then load the stats-driven charts. The table lazy-loads its own
// rows a page at a time via DataTable's serverSide ajax; only the charts need
// the full (slim, headers/body-free) dataset up front.
(async () => {
  const config = await fetch("/api/honeypot/config")
    .then(r => r.ok ? r.json() : null).catch(() => null);
  const maxEntries = config?.max_entries;
  if (maxEntries != null) {
    _subtitleEl.textContent =
      `Recent hits to the markov-babble honeypot and catch-all 404 endpoints. ` +
      `Showing up to ${maxEntries.toLocaleString()} most-recent entries (oldest auto-pruned).`;
  }

  _root.innerHTML = "";
  const chartsEl = el("div", "hp-charts");
  _root.appendChild(chartsEl);
  _root.appendChild(buildTableShell());
  initTable();

  await refreshCharts(chartsEl);
})();

async function doRefresh() {
  const chartsEl = _root.querySelector(".hp-charts");
  await refreshCharts(chartsEl);
  if (_dt) _dt.ajax.reload(null, false); // false = keep current page
  _lastUpdatedEl.textContent = `Updated ${new Date().toLocaleTimeString()}`;
}

// Fetches the slim, whole-dataset stats endpoint and (re)builds the
// heatmaps/top-stats charts. Does not touch the (separately paginated) table.
async function refreshCharts(chartsEl) {
  let hits;
  try {
    const res = await fetch("/api/honeypot/stats");
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    hits = await res.json();
  } catch (err) {
    chartsEl.innerHTML = `<p class="hp-empty">Failed to load stats: ${err.message}</p>`;
    return;
  }

  if (!hits || hits.length === 0) {
    chartsEl.innerHTML = `<p class="hp-empty">No hits recorded yet. The honeypot is waiting…</p>`;
    _countEl.textContent = '';
    return;
  }

  _countEl.textContent = `${hits.length} hit(s) recorded.`;

  // Pre-compute indices (UTC dates throughout)
  const byDate     = {};
  const byDateHour = {};
  for (const hit of hits) {
    const d   = hit.timestamp.slice(0, 10);
    const h   = new Date(hit.timestamp).getUTCHours();
    const key = `${d}-${h}`;
    if (!byDate[d])       byDate[d]       = [];
    if (!byDateHour[key]) byDateHour[key] = [];
    byDate[d].push(hit);
    byDateHour[key].push(hit);
  }

  chartsEl.innerHTML = "";
  chartsEl.appendChild(buildWeeklyHeatmap(byDate));
  chartsEl.appendChild(buildDailyHeatmap(byDate, byDateHour));
  chartsEl.appendChild(buildHourlyPattern(byDateHour));
  chartsEl.appendChild(buildTopStats(hits));
}

function initTable() {
  _dt = new DataTable(_root.querySelector(".hp-table"), {
    serverSide: true,
    processing:  true,
    ajax: async (data, callback) => {
      const params = new URLSearchParams({
        offset: data.start,
        limit:  data.length,
        search: data.search.value || "",
      });
      const order = data.order?.[0];
      if (order) {
        params.set("sort", SORT_COLUMNS[order.column] || "id");
        params.set("dir", order.dir);
      }
      let page;
      try {
        const res = await fetch(`/api/honeypot/hits?${params}`);
        if (!res.ok) throw new Error(`HTTP ${res.status}`);
        page = await res.json();
      } catch (err) {
        callback({ data: [], recordsTotal: 0, recordsFiltered: 0, error: err.message });
        return;
      }
      callback({ data: page.rows, recordsTotal: page.total, recordsFiltered: page.filtered });
    },
    layout: {
      topStart:    "info",
      topEnd:      "buttons",
      bottomStart: "pageLength",
      bottomEnd:   "paging",
    },
    buttons: [{ extend: "colvis", text: "Columns" }],
    pageLength: 50,
    lengthMenu: [[25, 50, 100], ["25", "50", "100"]], // no "All" — table is paginated server-side
    order:      [[5, "desc"]],
    columns: [
      { data: "id",        className: "hp-id" },
      { data: "slug",      className: "hp-slug",    render: (v) => `<code>${escHtml(v)}</code>` },
      { data: "ip",        className: "hp-ip",      render: (v) =>
          `<a href="https://ipinfo.io/${escHtml(v)}" target="_blank" rel="noopener noreferrer"><code>${escHtml(v)}</code></a>` },
      { data: "country",   className: "hp-country", render: (v) => v ? `${countryFlag(v)} ${escHtml(v)}` : '<span class="hp-unknown">—</span>' },
      { data: "org",       className: "hp-org",     render: (v) => v ? escHtml(v) : '<span class="hp-unknown">—</span>' },
      { data: "timestamp", className: "hp-ts",      render: (v) => escHtml(v) },
      { data: "headers", className: "hp-headers", orderable: false, render: (v) => {
          let pretty = v;
          try { pretty = JSON.stringify(JSON.parse(v), null, 2); } catch (_) {}
          return `<details><summary>show</summary><pre class="hp-json">${escHtml(pretty)}</pre></details>`;
        } },
      { data: "body", className: "hp-body", orderable: false, render: (v) =>
          v ? `<details><summary>show</summary><pre class="hp-json">${escHtml(v)}</pre></details>` : '<span class="hp-unknown">—</span>' },
    ],
    language: {
      info:         "Showing _START_–_END_ of _TOTAL_ hits",
      infoFiltered: " (filtered from _MAX_)",
      lengthMenu:   "Show _MENU_ rows",
      processing:   "Loading…",
    },
  });
}

function applyFilter() {
  if (!_dt) return;
  const term = (_filterEl?.value || "").trim();
  _dt.search(term).draw();
}

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
      addTooltipEvents(cell, () => ttDate(day.date, dayHits));
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
  const todayStr = toDateStr(new Date());
  const allDates = [];
  const rangeStart = new Date(Object.keys(byDate).sort()[0] + "T12:00:00");
  for (let d = new Date(rangeStart); toDateStr(d) <= todayStr; d.setDate(d.getDate() + 1))
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
        addTooltipEvents(cell, () => ttHour(date, h, cellHits));
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
    addTooltipEvents(col, () => html);
    col.appendChild(bar);
    col.appendChild(lbl);
    chart.appendChild(col);
  }
  wrap.appendChild(chart);
  return wrap;
}

// ── Top IPs + top slugs ───────────────────────────────────────────────────────

function buildTopStats(hits) {
  const ipCounts      = {};
  const slugCounts    = {};
  const countryCounts = {};
  for (const hit of hits) {
    ipCounts[hit.ip]       = (ipCounts[hit.ip]       || 0) + 1;
    slugCounts[hit.slug]   = (slugCounts[hit.slug]   || 0) + 1;
    const c = hit.country || "Unknown";
    countryCounts[c]       = (countryCounts[c]       || 0) + 1;
  }

  const topIPs       = Object.entries(ipCounts).sort((a, b) => b[1] - a[1]).slice(0, 10);
  const topSlugs     = Object.entries(slugCounts).sort((a, b) => b[1] - a[1]).slice(0, 10);
  const topCountries = Object.entries(countryCounts).sort((a, b) => b[1] - a[1]).slice(0, 10);

  const wrap = el("div", "hp-stats-grid");
  wrap.appendChild(buildBarChart("Top IPs", topIPs,
    lbl => `<a href="https://ipinfo.io/${escHtml(lbl)}" target="_blank" rel="noopener noreferrer">${escHtml(lbl)}</a>`
  ));
  wrap.appendChild(buildBarChart("Top slugs", topSlugs,
    lbl => `<code>${escHtml(lbl)}</code>`
  ));
  wrap.appendChild(buildBarChart("Top countries", topCountries,
    lbl => escHtml(lbl)
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

// Table rows are lazy-loaded a page at a time by DataTable's serverSide ajax
// (see initTable()) — this just builds the empty shell it attaches to.
function buildTableShell() {
  const wrap  = el("div", "hp-table-wrap");
  const table = el("table", "hp-table");
  table.innerHTML = `<thead><tr>
    <th>#</th><th>Slug</th><th>IP</th><th>Country</th><th>Org</th><th>Timestamp</th><th>Headers</th><th>Body</th>
  </tr></thead><tbody></tbody>`;
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
  // Dismiss on touch outside the tooltip
  document.addEventListener("touchstart", e => {
    if (_tt && _tt.style.display !== "none" && !_tt.contains(e.target)) hideTooltip();
  }, { passive: true });
  return _tt;
}
function showTooltip(e, html) {
  const tt = getTooltip();
  tt.innerHTML = html;
  tt.style.display = "block";
  // Clamp to viewport
  const tw = tt.offsetWidth || 200;
  const x  = e.clientX + 14 + tw > window.innerWidth
               ? e.clientX - tw - 6 : e.clientX + 14;
  tt.style.left = x + "px";
  tt.style.top  = (e.clientY + 14) + "px";
}
function hideTooltip() { getTooltip().style.display = "none"; }

// Attach both mouse-hover and touch-tap tooltip events
function addTooltipEvents(target, htmlFn) {
  target.addEventListener("mouseenter", e => showTooltip(e, htmlFn()));
  target.addEventListener("mouseleave", hideTooltip);
  target.addEventListener("touchstart", e => {
    e.preventDefault();
    e.stopPropagation();
    const touch = e.changedTouches[0];
    showTooltip({ clientX: touch.clientX, clientY: touch.clientY }, htmlFn());
  }, { passive: false });
}

function ttDate(date, hits) {
  const ipCounts = ipCounter(hits);
  const top      = Object.entries(ipCounts).sort((a, b) => b[1] - a[1]).slice(0, 4);
  const extra    = Object.keys(ipCounts).length - top.length;
  const countries = [...new Set(hits.map(h => h.country).filter(Boolean))];
  const orgs      = [...new Set(hits.map(h => h.org).filter(Boolean))].slice(0, 3);
  const flagStr   = countries.map(c => `${countryFlag(c)} ${c}`).join(", ");
  return `<div class="hp-tt-header">${escHtml(date)}</div>` +
         `<div class="hp-tt-count">${hits.length} hit${hits.length !== 1 ? "s" : ""}${flagStr ? ` &nbsp;·&nbsp; ${escHtml(flagStr)}` : ""}</div>` +
         (orgs.length ? `<div class="hp-tt-orgs">${orgs.map(o => `<div class="hp-tt-org">${escHtml(o)}</div>`).join("")}</div>` : "") +
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
  const countries = [...new Set(hits.map(h => h.country).filter(Boolean))];
  const orgs      = [...new Set(hits.map(h => h.org).filter(Boolean))].slice(0, 3);
  const flagStr   = countries.map(c => `${countryFlag(c)} ${c}`).join(", ");
  return `<div class="hp-tt-header">${escHtml(date)} &nbsp; ${h0}:00–${h1}:00 UTC</div>` +
         `<div class="hp-tt-count">${hits.length} hit${hits.length !== 1 ? "s" : ""}${flagStr ? ` &nbsp;·&nbsp; ${escHtml(flagStr)}` : ""}</div>` +
         (orgs.length ? `<div class="hp-tt-orgs">${orgs.map(o => `<div class="hp-tt-org">${escHtml(o)}</div>`).join("")}</div>` : "") +
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
function countryFlag(code) {
  if (!code || code.length !== 2) return "";
  return [...code.toUpperCase()].map(c =>
    String.fromCodePoint(0x1F1E6 + c.charCodeAt(0) - 65)
  ).join("");
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

