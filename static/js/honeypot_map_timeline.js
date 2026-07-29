"use strict";

// ── Config ────────────────────────────────────────────────────────────────────
const WORLD_URL    = "https://cdn.jsdelivr.net/npm/world-atlas@2/countries-110m.json";
// Slim endpoint (no headers/body) — this page only needs ip/country/timestamp
// but wants the whole dataset at once, not a paginated chunk.
const HITS_URL     = "/api/honeypot/stats";
const FLASH_WALL_MS = 1400;  // real-ms a country stays "lit" after a new hit crosses the playhead
const FEED_MAX     = 14;

// ── ISO alpha-2 → ISO numeric mapping ─────────────────────────────────────────
const A2N = {
  AF:4,   AL:8,   DZ:12,  AS:16,  AD:20,  AO:24,  AG:28,  AR:32,  AM:51,
  AU:36,  AT:40,  AZ:31,  BS:44,  BH:48,  BD:50,  BB:52,  BY:112, BE:56,
  BZ:84,  BJ:204, BT:64,  BO:68,  BA:70,  BW:72,  BR:76,  BN:96,  BG:100,
  BF:854, BI:108, CV:132, KH:116, CM:120, CA:124, CF:140, TD:148, CL:152,
  CN:156, CO:170, KM:174, CG:178, CD:180, CR:188, HR:191, CU:192, CY:196,
  CZ:203, CI:384, DK:208, DJ:262, DM:212, DO:214, EC:218, EG:818, SV:222,
  GQ:226, ER:232, EE:233, SZ:748, ET:231, FJ:242, FI:246, FR:250, GA:266,
  GM:270, GE:268, DE:276, GH:288, GR:300, GD:308, GT:320, GN:324, GW:624,
  GY:328, HT:332, HN:340, HK:344, HU:348, IS:352, IN:356, ID:360, IR:364,
  IQ:368, IE:372, IL:376, IT:380, JM:388, JP:392, JO:400, KZ:398, KE:404,
  KI:296, KP:408, KR:410, KW:414, KG:417, LA:418, LV:428, LB:422, LS:426,
  LR:430, LY:434, LI:438, LT:440, LU:442, MO:446, MG:450, MW:454, MY:458,
  MV:462, ML:466, MT:470, MH:584, MR:478, MU:480, MX:484, FM:583, MD:498,
  MC:492, MN:496, ME:499, MA:504, MZ:508, MM:104, NA:516, NR:520, NP:524,
  NL:528, NZ:554, NI:558, NE:562, NG:566, MK:807, NO:578, OM:512, PK:586,
  PW:585, PS:275, PA:591, PG:598, PY:600, PE:604, PH:608, PL:616, PT:620,
  QA:634, RO:642, RU:643, RW:646, KN:659, LC:662, VC:670, WS:882, SM:674,
  ST:678, SA:682, SN:686, RS:688, SC:690, SL:694, SG:702, SK:703, SI:705,
  SB:90,  SO:706, ZA:710, SS:728, ES:724, LK:144, SD:729, SR:740, SE:752,
  CH:756, SY:760, TW:158, TJ:762, TZ:834, TH:764, TL:626, TG:768, TO:776,
  TT:780, TN:788, TR:792, TM:795, TV:798, UG:800, UA:804, AE:784, GB:826,
  US:840, UY:858, UZ:860, VU:548, VE:862, VN:704, YE:887, ZM:894, ZW:716,
};
const N2A = Object.fromEntries(Object.entries(A2N).map(([a2, n]) => [n, a2]));

// ── State ─────────────────────────────────────────────────────────────────────
let allHits   = [];
let startMs   = 0;
let endMs     = 0;
let currentMs = 0;
let speed     = 1;
let playing   = false;
let lastWall  = null;
let rafId     = null;

// countryData[numericId] = { hits: [ms, ...] sorted asc, a2: "US" }
let countryData       = {};
let countryLastHitIdx = {};  // numId(str) → count already "flashed"
let countryFlashWall  = {};  // numId(str) → performance.now() of last flash

// D3 selections
let svgSel, countryPaths, pathFn, projFn;
let CSS = {};

// ── Init ──────────────────────────────────────────────────────────────────────
(async function init() {
  const loadEl = document.getElementById("hmt-loading");

  let world, rawHits;
  try {
    [world, rawHits] = await Promise.all([
      d3.json(WORLD_URL),
      fetch(HITS_URL).then(r => { if (!r.ok) throw new Error(`HTTP ${r.status}`); return r.json(); }),
    ]);
  } catch (err) {
    loadEl.textContent = `Failed to load data: ${err.message}`;
    return;
  }

  if (!rawHits || rawHits.length === 0) {
    loadEl.textContent = "No hits recorded yet.";
    return;
  }

  allHits = rawHits
    .map(h => ({ ...h, ms: new Date(h.timestamp).getTime() }))
    .sort((a, b) => a.ms - b.ms);

  startMs   = allHits[0].ms;
  endMs     = allHits[allHits.length - 1].ms;
  currentMs = startMs;

  for (const h of allHits) {
    const numId = A2N[h.country];
    if (!numId) continue;
    if (!countryData[numId]) countryData[numId] = { hits: [], a2: h.country };
    countryData[numId].hits.push(h.ms);
  }

  resolveCSS();
  buildMap(world);
  wireUI();

  loadEl.style.display = "none";
  document.getElementById("hmt-page").style.display = "flex";
  renderFrame();
})();

// ── CSS variable resolution ────────────────────────────────────────────────────
function resolveCSS() {
  const s = getComputedStyle(document.documentElement);
  const g = v => s.getPropertyValue(v).trim();
  CSS = {
    base:   g("--hmt-country-base") || "#2d333b",
    water:  g("--hmt-water")        || "#151b23",
    border: g("--hmt-border")       || "#444c56",
    flash:  g("--hmt-flash")        || "#ffffff",
    h1:     g("--hp-heat-1")        || "#0e4429",
    h2:     g("--hp-heat-2")        || "#006d32",
    h3:     g("--hp-heat-3")        || "#26a641",
    h4:     g("--hp-heat-4")        || "#39d353",
  };
}

// ── Map build ─────────────────────────────────────────────────────────────────
function buildMap(world) {
  const wrap = document.getElementById("hmt-map-wrap");
  const w = wrap.clientWidth  || 900;
  const h = wrap.clientHeight || 520;

  projFn = d3.geoNaturalEarth1().scale(w / 6.3).translate([w / 2, h / 2]);
  pathFn = d3.geoPath().projection(projFn);

  svgSel = d3.select("#hmt-svg").attr("viewBox", `0 0 ${w} ${h}`);

  // Ocean
  svgSel.append("path")
    .datum({ type: "Sphere" })
    .attr("class", "hmt-sphere")
    .attr("d", pathFn)
    .style("fill", CSS.water);

  // Graticule
  svgSel.append("path")
    .datum(d3.geoGraticule()())
    .attr("class", "hmt-graticule")
    .attr("d", pathFn)
    .style("fill", "none")
    .style("stroke", CSS.border)
    .style("stroke-width", "0.3")
    .style("opacity", "0.25");

  // Countries
  const countries = topojson.feature(world, world.objects.countries);
  countryPaths = svgSel.selectAll("path.hmt-country")
    .data(countries.features)
    .join("path")
    .attr("class", "hmt-country")
    .attr("d", pathFn)
    .style("fill", d => countryColor(d.id))
    .style("stroke", CSS.border)
    .style("stroke-width", "0.4")
    .on("mouseover", onCountryHover)
    .on("mousemove",  onCountryMove)
    .on("mouseleave", onCountryLeave);

  // Borders mesh
  svgSel.append("path")
    .datum(topojson.mesh(world, world.objects.countries, (a, b) => a !== b))
    .attr("class", "hmt-borders")
    .attr("d", pathFn)
    .style("fill", "none")
    .style("stroke", CSS.border)
    .style("stroke-width", "0.5")
    .style("opacity", "0.4");

  // Resize
  const ro = new ResizeObserver(debounce(() => {
    const w2 = wrap.clientWidth  || 900;
    const h2 = wrap.clientHeight || 520;
    projFn.scale(w2 / 6.3).translate([w2 / 2, h2 / 2]);
    pathFn = d3.geoPath().projection(projFn);
    svgSel.attr("viewBox", `0 0 ${w2} ${h2}`);
    svgSel.selectAll("path").attr("d", pathFn);
  }, 80));
  ro.observe(wrap);
}

// ── Tooltip ────────────────────────────────────────────────────────────────────
const ttEl = document.getElementById("hmt-tooltip");

function onCountryHover(event, d) {
  const a2  = N2A[d.id] || null;
  const cd  = countryData[d.id];
  const n   = cd ? countUpTo(cd.hits, currentMs) : 0;
  const flag = a2 ? countryFlag(a2) : "";
  ttEl.innerHTML =
    `<span class="hmt-tt-flag">${flag}</span> <strong>${a2 || `#${d.id}`}</strong>` +
    `<br><span class="hmt-tt-count">${n.toLocaleString()} hit${n !== 1 ? "s" : ""}</span>`;
  ttEl.style.display = "block";
  posTooltip(event);
}
function onCountryMove(event)  { posTooltip(event); }
function onCountryLeave()      { ttEl.style.display = "none"; }
function posTooltip(event) {
  const r = document.getElementById("hmt-map-wrap").getBoundingClientRect();
  let x = event.clientX - r.left + 14;
  let y = event.clientY - r.top  + 14;
  if (x + 160 > r.width)  x = event.clientX - r.left - 160;
  if (y + 60  > r.height) y = event.clientY - r.top  - 60;
  ttEl.style.left = x + "px";
  ttEl.style.top  = y + "px";
}

// ── Animation ─────────────────────────────────────────────────────────────────
function tick(wallNow) {
  if (lastWall !== null) {
    const delta  = wallNow - lastWall;
    const prevMs = currentMs;
    currentMs    = Math.min(endMs, currentMs + delta * speed);
    checkNewHits(prevMs, currentMs);
    if (currentMs >= endMs) {
      playing = false;
      document.getElementById("hmt-playpause").textContent = "▶ Play";
    }
  }
  lastWall = wallNow;
  renderFrame();
  if (playing) rafId = requestAnimationFrame(tick);
}

function checkNewHits(prevMs, nextMs) {
  if (nextMs <= prevMs) return;
  for (const [key, cd] of Object.entries(countryData)) {
    const prevN = countryLastHitIdx[key] ?? 0;
    const newN  = countUpTo(cd.hits, nextMs);
    if (newN > prevN) {
      countryFlashWall[key]  = performance.now();
      countryLastHitIdx[key] = newN;
    }
  }
}

function renderFrame() {
  updateMap();
  updateScrubber();
  updatePanel();
}

function updateMap() {
  if (!countryPaths) return;
  countryPaths.style("fill", d => countryColor(d.id));
}

function updateScrubber() {
  const range = endMs - startMs || 1;
  document.getElementById("hmt-scrubber").value =
    ((currentMs - startMs) / range * 1000).toFixed(0);
  document.getElementById("hmt-current-time").textContent = fmtTimeFull(currentMs);
}

function updatePanel() {
  const visHits = allHits.filter(h => h.ms <= currentMs);

  document.getElementById("hmt-stat-hits").textContent =
    visHits.length.toLocaleString();

  let ctrHit = 0;
  for (const cd of Object.values(countryData))
    if (countUpTo(cd.hits, currentMs) > 0) ctrHit++;
  document.getElementById("hmt-stat-countries").textContent = ctrHit;

  // Top countries
  const byCtr = {};
  for (const h of visHits) {
    if (!h.country) continue;
    byCtr[h.country] = (byCtr[h.country] || 0) + 1;
  }
  const top    = Object.entries(byCtr).sort((a, b) => b[1] - a[1]).slice(0, 8);
  const maxCtr = top[0]?.[1] || 1;
  document.getElementById("hmt-top-countries").innerHTML = top.map(([a2, n]) =>
    `<div class="hmt-top-row">
      <span class="hmt-top-flag">${countryFlag(a2)}</span>
      <span class="hmt-top-name">${a2}</span>
      <div class="hmt-top-track"><div class="hmt-top-fill" style="width:${(n / maxCtr * 100).toFixed(1)}%"></div></div>
      <span class="hmt-top-count">${n.toLocaleString()}</span>
    </div>`
  ).join("");

  // Feed: most recent hits
  const recent = visHits.slice(-FEED_MAX).reverse();
  document.getElementById("hmt-feed").innerHTML = recent.map(h =>
    `<div class="hmt-feed-item">
      <span class="hmt-feed-flag">${countryFlag(h.country)}</span>
      <code class="hmt-feed-ip">${escHtml(h.ip)}</code>
      <span class="hmt-feed-time">${fmtTimeShort(h.ms)}</span>
    </div>`
  ).join("");
}

// ── UI wiring ─────────────────────────────────────────────────────────────────
function wireUI() {
  const scrubber = document.getElementById("hmt-scrubber");
  document.getElementById("hmt-time-start").textContent = fmtTimeShort(startMs);
  document.getElementById("hmt-time-end").textContent   = fmtTimeShort(endMs);

  scrubber.addEventListener("input", () => {
    currentMs = startMs + (parseInt(scrubber.value) / 1000) * (endMs - startMs);
    lastWall  = null;
    // Sync flash state to current position
    for (const [key, cd] of Object.entries(countryData))
      countryLastHitIdx[key] = countUpTo(cd.hits, currentMs);
    countryFlashWall = {};
    renderFrame();
  });

  document.getElementById("hmt-playpause").addEventListener("click", () => {
    if (currentMs >= endMs) { currentMs = startMs; countryFlashWall = {}; countryLastHitIdx = {}; }
    playing = !playing;
    document.getElementById("hmt-playpause").textContent = playing ? "⏸ Pause" : "▶ Play";
    if (playing) { lastWall = null; rafId = requestAnimationFrame(tick); }
    else { cancelAnimationFrame(rafId); rafId = null; }
  });

  document.getElementById("hmt-restart").addEventListener("click", () => {
    if (rafId) { cancelAnimationFrame(rafId); rafId = null; }
    playing           = false;
    currentMs         = startMs;
    countryFlashWall  = {};
    countryLastHitIdx = {};
    document.getElementById("hmt-playpause").textContent = "▶ Play";
    renderFrame();
  });

  document.querySelectorAll(".hmt-speed").forEach(btn => {
    btn.addEventListener("click", () => {
      speed = parseFloat(btn.dataset.speed);
      document.querySelectorAll(".hmt-speed").forEach(b => b.classList.remove("hmt-speed-active"));
      btn.classList.add("hmt-speed-active");
    });
  });
}

// ── Color helpers ─────────────────────────────────────────────────────────────
function countryColor(dId) {
  const cd = countryData[dId];
  if (!cd) return CSS.base;
  const n = countUpTo(cd.hits, currentMs);
  if (n === 0) return CSS.base;
  const flashAge = performance.now() - (countryFlashWall[dId] || 0);
  if (flashAge < FLASH_WALL_MS) {
    const t = Math.max(0, flashAge / FLASH_WALL_MS);
    return d3.interpolateRgb(CSS.flash, heatColor(n))(t);
  }
  return heatColor(n);
}

function heatColor(n) {
  if (n <= 1)  return CSS.h1;
  if (n <= 4)  return CSS.h2;
  if (n <= 15) return CSS.h3;
  return CSS.h4;
}

// ── Helpers ────────────────────────────────────────────────────────────────────
function countUpTo(arr, ms) {
  let lo = 0, hi = arr.length;
  while (lo < hi) { const mid = (lo + hi) >> 1; if (arr[mid] <= ms) lo = mid + 1; else hi = mid; }
  return lo;
}

function countryFlag(code) {
  if (!code || code.length !== 2) return "";
  return [...code.toUpperCase()].map(c => String.fromCodePoint(0x1F1E6 + c.charCodeAt(0) - 65)).join("");
}

function escHtml(s) {
  return String(s).replace(/&/g,"&amp;").replace(/</g,"&lt;").replace(/>/g,"&gt;");
}

function fmtTimeFull(ms) {
  return new Date(ms).toLocaleString(undefined, {
    year:"numeric", month:"short", day:"numeric",
    hour:"2-digit", minute:"2-digit", second:"2-digit",
  });
}

function fmtTimeShort(ms) {
  return new Date(ms).toLocaleString(undefined, {
    month:"short", day:"numeric",
    hour:"2-digit", minute:"2-digit",
  });
}

function debounce(fn, ms) {
  let t;
  return (...args) => { clearTimeout(t); t = setTimeout(() => fn(...args), ms); };
}
