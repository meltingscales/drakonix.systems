// ── Activity definitions ─────────────────────────────────────────────────────
const PRESET_ACTIVITIES = [
  { label: "eat a Big Mac \u{1f354}", duration: 900, color: "#e74c3c" },
  {
    label: "speedrun Minecraft any% \u{26cf}\ufe0f",
    duration: 720,
    color: "#3498db",
  },
  {
    label: "watch Titanic (1997) \u{1f6a2}",
    duration: 11640,
    color: "#2ecc71",
  },
  {
    label: "drive from Chicago to Texas \u{1f697}",
    duration: 50400,
    color: "#f39c12",
  },
  { label: "run a marathon \u{1f3c3}", duration: 14400, color: "#9b59b6" },
  {
    label: "play a full game of Monopoly \u{1f3b2}",
    duration: 10800,
    color: "#1abc9c",
  },
  { label: "read War and Peace \u{1f4da}", duration: 216000, color: "#e67e22" },
  {
    label: "beat Dark Souls \u{2694}\ufe0f",
    duration: 198000,
    color: "#27ae60",
  },
  {
    label: "watch all of Breaking Bad \u{1f9ea}",
    duration: 223200,
    color: "#8e44ad",
  },
  {
    label: "circumnavigate the globe by cargo ship \u{1f30d}",
    duration: 5184000,
    color: "#2980b9",
  },
];
const EXTRA_COLORS = ["#c0392b", "#7f8c8d", "#d35400", "#16a085", "#2c3e50"];

// Human-readable slug for each preset (order must match PRESET_ACTIVITIES)
const PRESET_SLUGS = [
  "big-mac",
  "minecraft",
  "titanic",
  "chicago-texas",
  "marathon",
  "monopoly",
  "war-and-peace",
  "dark-souls",
  "breaking-bad",
  "cargo-ship",
];

// ── State ────────────────────────────────────────────────────────────────────
const S = {
  age: 25,
  lifeExp: 78,
  bpSystolic: 120,
  bpDiastolic: 80,
  initialized: false, // true after first calculate; gates encodeHash
  secondsLeft: 0,
  primaryLabel: "",
  primaryDuration: 0,
  featuredIdx: 0, // index into getActiveItems()
  presetOn: new Array(PRESET_ACTIVITIES.length).fill(false),
  customActs: [], // { id, label, duration, color, on }
  nextId: 0,
  interval: null,
  chart: null,
  logScale: true,
  view: "chart",
  autoAdvance: false,
  autoAdvanceTimer: null,
};

// Per-card waffle state: label → { totalSquares, squareValue, lastFilled, gridEl, labelEl }
const waffleStates = new Map();

// ── DOM ──────────────────────────────────────────────────────────────────────
const el = (id) => document.getElementById(id);
const formEl = el("dt-form");
const resultEl = el("dt-result");
const deadEl = el("dt-dead");

// ── Formatting ───────────────────────────────────────────────────────────────
function fmt(n) {
  if (n < 0) return "0.00";
  const [i, d] = n.toFixed(2).split(".");
  return i.replace(/\B(?=(\d{3})+(?!\d))/g, ",") + "." + d;
}
function fmtShort(n) {
  if (n >= 1e9) return (n / 1e9).toFixed(1) + "B";
  if (n >= 1e6) return (n / 1e6).toFixed(1) + "M";
  if (n >= 1000) return (n / 1000).toFixed(1) + "K";
  return n.toFixed(1);
}

// ── Active items ──────────────────────────────────────────────────────────────
function getActiveItems() {
  const items = [];
  PRESET_ACTIVITIES.forEach((a, i) => {
    if (S.presetOn[i]) items.push(a);
  });
  S.customActs.forEach((a) => {
    if (a.on) items.push(a);
  });
  return items;
}

function calcTimes(duration) {
  return Math.max(0, S.secondsLeft / duration);
}

// ── Heartbeat ────────────────────────────────────────────────────────────────
function updateHeartbeat() {
  const beats = Math.round((S.secondsLeft * 70) / 60);
  el("dt-heartbeat").textContent =
    "\u2764\ufe0f\u200a~\u202f" +
    beats.toLocaleString() +
    " heartbeats remaining";
}

// ── URL hash (human-readable) ─────────────────────────────────────────────────
// Format: #age=32&life=78&sys=120&dia=80&on=big-mac,marathon&custom=swim%20laps:1800|yoga:600&view=chart&feat=marathon
function encodeHash() {
  if (!S.initialized) return;
  const p = new URLSearchParams();
  p.set("age", S.age);
  p.set("life", S.lifeExp);
  p.set("sys", S.bpSystolic);
  p.set("dia", S.bpDiastolic);

  const onSlugs = PRESET_SLUGS.filter((_, i) => S.presetOn[i]);
  if (onSlugs.length) p.set("on", onSlugs.join(","));

  const onCustoms = S.customActs.filter((a) => a.on);
  if (onCustoms.length)
    p.set(
      "custom",
      onCustoms
        .map((a) => `${encodeURIComponent(a.label)}:${a.duration}`)
        .join("|"),
    );

  p.set("view", S.view);
  if (!S.logScale) p.set("log", "off");

  const items = getActiveItems();
  const feat = items[S.featuredIdx];
  if (feat) {
    const pi = PRESET_ACTIVITIES.findIndex((a) => a.label === feat.label);
    p.set("feat", pi >= 0 ? PRESET_SLUGS[pi] : encodeURIComponent(feat.label));
  }
  history.replaceState(null, "", "#" + p.toString());
}

function decodeHash() {
  if (!location.hash || location.hash.length <= 1) return null;
  const p = new URLSearchParams(location.hash.slice(1));
  const age = parseFloat(p.get("age"));
  const lifeExp = parseFloat(p.get("life"));
  if (!age || !lifeExp || age <= 0 || lifeExp <= 0) return null;

  const bpSystolic = parseInt(p.get("sys")) || 120;
  const bpDiastolic = parseInt(p.get("dia")) || 80;
  const onSlugs = p.get("on") ? p.get("on").split(",") : [];
  const custStr = p.get("custom") ? p.get("custom").split("|") : [];
  const view = p.get("view") || "chart";
  const logScale = p.get("log") !== "off";
  const featParam = p.get("feat") || null;

  const presetOn = PRESET_SLUGS.map((slug) => onSlugs.includes(slug));
  const customs = custStr
    .map((s) => {
      const ci = s.lastIndexOf(":");
      if (ci < 0) return null;
      const label = decodeURIComponent(s.slice(0, ci));
      const duration = parseFloat(s.slice(ci + 1));
      return label && duration > 0 ? { label, duration } : null;
    })
    .filter(Boolean);

  return { age, lifeExp, bpSystolic, bpDiastolic, presetOn, customs, view, logScale, featParam };
}

function applyHash(decoded) {
  const { age, lifeExp, bpSystolic, bpDiastolic, presetOn, customs, view, logScale, featParam } =
    decoded;

  // Update BP form inputs and display
  el("dt-bp-systolic").value = bpSystolic;
  el("dt-bp-diastolic").value = bpDiastolic;
  updateBPDisplay();

  if (lifeExp - age <= 0) {
    formEl.style.display = "none";
    resultEl.style.display = "none";
    deadEl.style.display = "";
    return;
  }
  const usedColors = new Set(
    presetOn
      .map((on, i) => (on ? PRESET_ACTIVITIES[i].color : null))
      .filter(Boolean),
  );
  let nextId = 0;
  const customActs = customs.map(({ label, duration }) => {
    const color =
      EXTRA_COLORS.find((c) => !usedColors.has(c)) ?? EXTRA_COLORS[0];
    usedColors.add(color);
    return { id: nextId++, label, duration, color, on: true };
  });
  runCalculate({
    age,
    lifeExp,
    bpSystolic,
    bpDiastolic,
    presetOn,
    customActs,
    nextId,
    view,
    logScale,
    featParam,
  });
}

// ── Core calculate ────────────────────────────────────────────────────────────
function runCalculate({
  age,
  lifeExp,
  bpSystolic,
  bpDiastolic,
  presetOn,
  customActs,
  nextId,
  view,
  logScale,
  featParam,
}) {
  S.initialized = false; // suppress encodeHash during setup
  if (S.interval) {
    clearInterval(S.interval);
    S.interval = null;
  }
  if (S.chart) {
    S.chart.destroy();
    S.chart = null;
  }
  setAutoAdvance(false);

  S.age = age;
  S.lifeExp = lifeExp;
  S.bpSystolic = bpSystolic ?? 120;
  S.bpDiastolic = bpDiastolic ?? 80;
  S.secondsLeft = (lifeExp - age) * 365.25 * 24 * 3600;
  S.presetOn = presetOn ?? new Array(PRESET_ACTIVITIES.length).fill(false);
  S.customActs = customActs ?? [];
  S.nextId = nextId != null ? nextId : S.customActs.length;
  S.featuredIdx = 0;
  S.logScale = logScale != null ? logScale : true;

  waffleStates.clear();
  el("dt-sidebar-list").innerHTML = "";
  el("dt-waffle-panel").innerHTML = "";
  formEl.style.display = "none";
  resultEl.style.display = "";
  deadEl.style.display = "none";

  initChart();
  buildSidebar();
  setFeatured(0);
  buildMultiWaffle();
  showView(view || "chart");
  updateHeartbeat();

  // restore featured item from hash param (after UI is built)
  if (featParam) {
    const items = getActiveItems();
    const pi = PRESET_SLUGS.indexOf(featParam);
    const fi =
      pi >= 0
        ? items.findIndex((it) => it.label === PRESET_ACTIVITIES[pi]?.label)
        : items.findIndex((it) => it.label === decodeURIComponent(featParam));
    if (fi >= 0) setFeatured(fi);
  }

  S.initialized = true;
  encodeHash();
  S.interval = setInterval(tick, 1000);
}

// ── Auto-advance ──────────────────────────────────────────────────────────────
function setAutoAdvance(on) {
  S.autoAdvance = on;
  if (S.autoAdvanceTimer) {
    clearInterval(S.autoAdvanceTimer);
    S.autoAdvanceTimer = null;
  }
  if (on) {
    S.autoAdvanceTimer = setInterval(() => {
      if (getActiveItems().length > 1) setFeatured(S.featuredIdx + 1);
    }, 30000);
  }
  const btn = el("dt-auto-advance");
  if (!btn) return;
  btn.textContent = "\u25b6\u25b6 Autoplay: " + (on ? "ON" : "OFF");
  btn.classList.toggle("dt-log-btn-active", on);
}

// ── Featured navigation ───────────────────────────────────────────────────────
function setFeatured(idx) {
  const items = getActiveItems();
  if (items.length === 0) {
    el("dt-feat-prev").style.visibility = "hidden";
    el("dt-feat-next").style.visibility = "hidden";
    el("dt-main-label").textContent = "";
    el("dt-count").textContent = "—";
    el("dt-context").textContent = "";
    updateSidebarFeatured();
    encodeHash();
    return;
  }
  S.featuredIdx = ((idx % items.length) + items.length) % items.length;
  const item = items[S.featuredIdx];
  S.primaryLabel = item.label;
  S.primaryDuration = item.duration;

  el("dt-main-label").textContent = item.label;
  el("dt-count").textContent = fmt(calcTimes(item.duration));
  el("dt-context").textContent =
    `(~${(86400 / item.duration).toFixed(1)}/day · ~${(604800 / item.duration).toFixed(1)}/week if that's all you did)`;

  const multi = items.length > 1;
  el("dt-feat-prev").style.visibility = multi ? "" : "hidden";
  el("dt-feat-next").style.visibility = multi ? "" : "hidden";

  updateSidebarFeatured();
  encodeHash();
}

function updateSidebarFeatured() {
  const items = getActiveItems();
  const featLabel = items[S.featuredIdx]?.label ?? null;
  const multi = items.length > 1;
  document.querySelectorAll(".dt-sidebar-item").forEach((item) => {
    const isFeatured = multi && item.dataset.label === featLabel;
    const star = item.querySelector(".dt-feat-star");
    if (star) star.style.display = isFeatured ? "" : "none";
  });
}

// ── Sidebar helpers ───────────────────────────────────────────────────────────
function applySidebarItemStyle(div, color, on) {
  div.style.borderColor = on ? color : "transparent";
  div.style.backgroundColor = on ? color + "18" : "";
}
function applyDotStyle(dot, color, on) {
  dot.style.borderColor = color;
  dot.style.backgroundColor = on ? color : "transparent";
}
function sidebarItemByLabel(label) {
  for (const item of document.querySelectorAll(".dt-sidebar-item")) {
    if (item.dataset.label === label) return item;
  }
  return null;
}
function updateSidebarOnOff(label, on, color) {
  const item = sidebarItemByLabel(label);
  if (!item) return;
  applySidebarItemStyle(item, color, on);
  applyDotStyle(item.querySelector(".dt-sidebar-dot"), color, on);
}

function makeSidebarItem({
  label,
  duration,
  color,
  on,
  countAttr,
  onClick,
  onRemove,
}) {
  const div = document.createElement("div");
  div.className = "dt-sidebar-item";
  div.dataset.label = label;
  applySidebarItemStyle(div, color, on);

  div.innerHTML =
    `<span class="dt-sidebar-dot"></span>` +
    `<span class="dt-sidebar-name" title="${label}">${label}</span>` +
    `<span class="dt-sidebar-count" ${countAttr}>${fmtShort(calcTimes(duration))}</span>` +
    `<span class="dt-feat-star" style="display:none;color:${color}">&#x25b6;</span>` +
    (onRemove
      ? `<button class="dt-sidebar-remove" title="Remove">&#x2715;</button>`
      : "");

  applyDotStyle(div.querySelector(".dt-sidebar-dot"), color, on);

  div.addEventListener("click", (e) => {
    if (e.target.classList.contains("dt-sidebar-remove")) return;
    onClick();
  });
  if (onRemove) {
    div.querySelector(".dt-sidebar-remove").addEventListener("click", (e) => {
      e.stopPropagation();
      onRemove();
    });
  }
  return div;
}

function buildSidebar() {
  const list = el("dt-sidebar-list");
  list.innerHTML = "";
  PRESET_ACTIVITIES.forEach((act, i) => {
    list.appendChild(
      makeSidebarItem({
        label: act.label,
        duration: act.duration,
        color: act.color,
        on: S.presetOn[i],
        countAttr: `data-preset-count="${i}"`,
        onClick: () => togglePreset(i),
      }),
    );
  });
  S.customActs.forEach((act) => {
    const item = makeSidebarItem({
      label: act.label,
      duration: act.duration,
      color: act.color,
      on: act.on,
      countAttr: `data-custom-count="${act.id}"`,
      onClick: () => toggleCustom(act.id),
      onRemove: () => removeCustom(act.id),
    });
    item.dataset.customId = act.id;
    list.appendChild(item);
  });
}

// ── 3-state toggle: off → on+feature, on+featured → off, on+notFeatured → feature ──
function togglePreset(i) {
  const act = PRESET_ACTIVITIES[i];
  const on = S.presetOn[i];
  const items = getActiveItems();
  const isFeat = on && act.label === (items[S.featuredIdx]?.label ?? null);

  if (!on) {
    S.presetOn[i] = true;
    updateSidebarOnOff(act.label, true, act.color);
    updateChart();
    syncChartHeight();
    buildMultiWaffle();
    setFeatured(getActiveItems().findIndex((it) => it.label === act.label));
  } else if (isFeat) {
    S.presetOn[i] = false;
    updateSidebarOnOff(act.label, false, act.color);
    updateChart();
    syncChartHeight();
    buildMultiWaffle();
    const ni = getActiveItems();
    setFeatured(Math.min(S.featuredIdx, Math.max(0, ni.length - 1)));
  } else {
    setFeatured(items.findIndex((it) => it.label === act.label));
  }
}

function toggleCustom(id) {
  const act = S.customActs.find((a) => a.id === id);
  if (!act) return;
  const on = act.on;
  const items = getActiveItems();
  const isFeat = on && act.label === (items[S.featuredIdx]?.label ?? null);

  if (!on) {
    act.on = true;
    updateSidebarOnOff(act.label, true, act.color);
    updateChart();
    syncChartHeight();
    buildMultiWaffle();
    setFeatured(getActiveItems().findIndex((it) => it.label === act.label));
  } else if (isFeat) {
    act.on = false;
    updateSidebarOnOff(act.label, false, act.color);
    updateChart();
    syncChartHeight();
    buildMultiWaffle();
    const ni = getActiveItems();
    setFeatured(Math.min(S.featuredIdx, Math.max(0, ni.length - 1)));
  } else {
    setFeatured(items.findIndex((it) => it.label === act.label));
  }
}

function removeCustom(id) {
  S.customActs = S.customActs.filter((a) => a.id !== id);
  buildSidebar();
  updateChart();
  syncChartHeight();
  buildMultiWaffle();
  const ni = getActiveItems();
  setFeatured(Math.min(S.featuredIdx, Math.max(0, ni.length - 1)));
}

function addCustomActivity(label, duration) {
  const usedColors = new Set([
    ...PRESET_ACTIVITIES.map((a) => a.color),
    ...S.customActs.map((a) => a.color),
  ]);
  const color =
    [...EXTRA_COLORS].find((c) => !usedColors.has(c)) ?? EXTRA_COLORS[0];
  const act = { id: S.nextId++, label, duration, color, on: true };
  S.customActs.push(act);

  const item = makeSidebarItem({
    label,
    duration,
    color,
    on: true,
    countAttr: `data-custom-count="${act.id}"`,
    onClick: () => toggleCustom(act.id),
    onRemove: () => removeCustom(act.id),
  });
  item.dataset.customId = act.id;
  el("dt-sidebar-list").appendChild(item);

  updateChart();
  syncChartHeight();
  buildMultiWaffle();
  setFeatured(getActiveItems().findIndex((it) => it.label === label));
}

function updateSidebarCounts() {
  PRESET_ACTIVITIES.forEach((act, i) => {
    const s = document.querySelector(`[data-preset-count="${i}"]`);
    if (s) s.textContent = fmtShort(calcTimes(act.duration));
  });
  S.customActs.forEach((act) => {
    const s = document.querySelector(`[data-custom-count="${act.id}"]`);
    if (s) s.textContent = fmtShort(calcTimes(act.duration));
  });
}

// ── Chart ────────────────────────────────────────────────────────────────────
function cssVar(name) {
  return getComputedStyle(document.documentElement)
    .getPropertyValue(name)
    .trim();
}

function initChart() {
  if (S.chart) {
    S.chart.destroy();
    S.chart = null;
  }
  const ctx = el("dt-chart").getContext("2d");
  S.chart = new Chart(ctx, {
    type: "bar",
    data: {
      labels: [],
      datasets: [
        {
          data: [],
          backgroundColor: [],
          borderColor: [],
          borderWidth: 2,
          maxBarThickness: 44,
        },
      ],
    },
    options: {
      indexAxis: "y",
      responsive: true,
      maintainAspectRatio: false,
      animation: false,
      plugins: {
        legend: { display: false },
        tooltip: { callbacks: { label: (c) => "  " + fmt(c.raw) + " times" } },
      },
      scales: {
        x: {
          type: S.logScale ? "logarithmic" : "linear",
          ticks: {
            color: cssVar("--text-primary") || "#333",
            callback: (v) => fmtShort(v),
          },
          grid: { color: cssVar("--border-primary") || "#ddd" },
        },
        y: {
          ticks: { color: cssVar("--text-primary") || "#333" },
          grid: { color: cssVar("--border-primary") || "#ddd" },
        },
      },
    },
  });
  syncChartHeight();
}

function syncChartHeight() {
  const n = getActiveItems().length;
  el("dt-chart").parentElement.style.height = Math.max(60, n * 68) + "px";
  if (S.chart) S.chart.resize();
}

function updateChart() {
  const items = getActiveItems();
  const emptyMsg = el("dt-chart-empty");
  if (items.length === 0) {
    emptyMsg.classList.add("visible");
    el("dt-chart").parentElement.style.height = "0";
    if (S.chart) {
      S.chart.data.labels = [];
      S.chart.data.datasets[0].data = [];
      S.chart.update("none");
    }
    return;
  }
  emptyMsg.classList.remove("visible");
  syncChartHeight();
  if (!S.chart) return;
  const ch = S.chart;
  ch.data.labels = items.map((i) => i.label);
  ch.data.datasets[0].data = items.map((i) =>
    Math.max(0.0001, calcTimes(i.duration)),
  );
  ch.data.datasets[0].backgroundColor = items.map((i) => i.color + "55");
  ch.data.datasets[0].borderColor = items.map((i) => i.color);
  ch.options.scales.x.type = S.logScale ? "logarithmic" : "linear";
  ch.update("none");
}

// ── Multi-waffle ──────────────────────────────────────────────────────────────
function buildMultiWaffle() {
  const panel = el("dt-waffle-panel");
  panel.innerHTML = "";
  waffleStates.clear();

  const items = getActiveItems();
  if (items.length === 0) {
    const p = document.createElement("p");
    p.className = "dt-panel-empty visible";
    p.textContent = "Toggle activities on the right to see waffles.";
    panel.appendChild(p);
    return;
  }

  const container = document.createElement("div");
  container.className = "dt-multi-waffle";
  items.forEach((item) => container.appendChild(buildWaffleCard(item)));
  panel.appendChild(container);
}

function buildWaffleCard(item) {
  const initCount = calcTimes(item.duration);

  const card = document.createElement("div");
  card.className = "dt-waffle-card";
  card.style.borderTopColor = item.color;

  const titleEl = document.createElement("p");
  titleEl.className = "dt-waffle-card-title";
  titleEl.style.color = item.color;
  titleEl.textContent = item.label;
  titleEl.title = item.label;

  const gridEl = document.createElement("div");
  gridEl.className = "dt-waffle-card-grid";
  gridEl.style.setProperty("--sq-color", item.color);

  const labelEl = document.createElement("p");
  labelEl.className = "dt-waffle-card-label";

  card.appendChild(titleEl);
  card.appendChild(gridEl);
  card.appendChild(labelEl);

  const totalSquares = Math.min(400, Math.max(1, Math.ceil(initCount)));
  const squareValue = initCount / totalSquares;

  for (let i = 0; i < totalSquares; i++) {
    const sq = document.createElement("div");
    sq.className = "dt-waffle-sq filled";
    gridEl.appendChild(sq);
  }

  waffleStates.set(item.label, {
    totalSquares,
    squareValue,
    lastFilled: -1,
    gridEl,
    labelEl,
  });
  renderWaffleCard(item.label, initCount);
  return card;
}

function renderWaffleCard(label, currentCount) {
  const ws = waffleStates.get(label);
  if (!ws) return;
  const filled = Math.min(
    ws.totalSquares,
    Math.max(0, Math.floor(currentCount / ws.squareValue)),
  );
  if (filled !== ws.lastFilled) {
    ws.lastFilled = filled;
    ws.gridEl.querySelectorAll(".dt-waffle-sq").forEach((sq, i) => {
      sq.className = "dt-waffle-sq " + (i < filled ? "filled" : "empty");
    });
  }
  const sv = ws.squareValue;
  ws.labelEl.innerHTML =
    (sv < 1.5
      ? "&#x25a0;=1"
      : `&#x25a0;&asymp;${Math.round(sv).toLocaleString()}`) +
    ` &middot; ${fmt(currentCount)}`;
}

function updateMultiWaffle() {
  const items = getActiveItems();
  items.forEach((item) => {
    if (waffleStates.has(item.label))
      renderWaffleCard(item.label, calcTimes(item.duration));
  });
}

// ── View switching ────────────────────────────────────────────────────────────
function showView(v) {
  S.view = v;
  const isChart = v === "chart";
  el("dt-chart-panel").style.display = isChart ? "" : "none";
  el("dt-waffle-panel").style.display = isChart ? "none" : "";
  el("dt-tab-chart").classList.toggle("dt-tab-active", isChart);
  el("dt-tab-waffle").classList.toggle("dt-tab-active", !isChart);
  el("dt-log-toggle").style.display = isChart ? "" : "none";
  if (isChart) updateChart();
  encodeHash();
}

// ── Tick ──────────────────────────────────────────────────────────────────────
function tick() {
  S.secondsLeft = Math.max(0, S.secondsLeft - 1);
  if (S.primaryDuration > 0)
    el("dt-count").textContent = fmt(calcTimes(S.primaryDuration));
  updateSidebarCounts();
  updateChart();
  updateMultiWaffle();
  updateHeartbeat();
}

// ── Reset ─────────────────────────────────────────────────────────────────────
function reset() {
  if (S.interval) {
    clearInterval(S.interval);
    S.interval = null;
  }
  if (S.chart) {
    S.chart.destroy();
    S.chart = null;
  }
  setAutoAdvance(false);
  S.initialized = false;
  S.presetOn = new Array(PRESET_ACTIVITIES.length).fill(false);
  S.customActs = [];
  S.featuredIdx = 0;
  waffleStates.clear();
  el("dt-sidebar-list").innerHTML = "";
  el("dt-waffle-panel").innerHTML = "";
  formEl.style.display = "";
  resultEl.style.display = "none";
  deadEl.style.display = "none";
  history.replaceState(null, "", location.pathname + location.search);
}

// ── Blood pressure ─────────────────────────────────────────────────────────────
function getBPCategory(systolic, diastolic) {
  if (systolic >= 180 || diastolic >= 120)
    return { category: "Hypertensive Crisis", class: "bp-crisis", penalty: 15 };
  if (systolic >= 140 || diastolic >= 90)
    return { category: "Stage 2 Hypertension", class: "bp-stage2", penalty: 8 };
  if (systolic >= 130 || diastolic >= 80)
    return { category: "Stage 1 Hypertension", class: "bp-stage1", penalty: 4 };
  if (systolic >= 120 && diastolic < 80)
    return { category: "Elevated", class: "bp-elevated", penalty: 2 };
  return { category: "Normal", class: "bp-normal", penalty: 0 };
}

function updateBPDisplay() {
  const systolic = parseInt(el("dt-bp-systolic").value);
  const diastolic = parseInt(el("dt-bp-diastolic").value);
  el("dt-bp-systolic-val").textContent = systolic;
  el("dt-bp-diastolic-val").textContent = diastolic;

  const { category, class: catClass } = getBPCategory(systolic, diastolic);
  const catEl = el("dt-bp-category");
  catEl.textContent = category;
  catEl.className = "dt-hint " + catClass;
}

function adjustLifeExpectancyForBP(baseLifeExp, systolic, diastolic) {
  const { penalty } = getBPCategory(systolic, diastolic);
  return Math.max(1, baseLifeExp - penalty);
}

// Blood pressure slider listeners
el("dt-bp-systolic").addEventListener("input", updateBPDisplay);
el("dt-bp-diastolic").addEventListener("input", updateBPDisplay);

// ── Event listeners ───────────────────────────────────────────────────────────
el("dt-activity").addEventListener("change", function () {
  const c = this.value === "custom";
  el("dt-custom-name-row").style.display = c ? "" : "none";
  el("dt-custom-dur-row").style.display = c ? "" : "none";
});

el("dt-calc-btn").addEventListener("click", () => {
  const age = parseFloat(el("dt-age").value) || 0;
  const baseLifeExp = parseFloat(el("dt-life-exp").value) || 78;
  const systolic = parseInt(el("dt-bp-systolic").value) || 120;
  const diastolic = parseInt(el("dt-bp-diastolic").value) || 80;
  const lifeExp = adjustLifeExpectancyForBP(baseLifeExp, systolic, diastolic);

  if (lifeExp - age <= 0) {
    formEl.style.display = "none";
    resultEl.style.display = "none";
    deadEl.style.display = "";
    history.replaceState(null, "", location.pathname + location.search);
    return;
  }

  const presetOn = new Array(PRESET_ACTIVITIES.length).fill(false);
  const customActs = [];
  const sel = el("dt-activity");
  if (sel.value === "custom") {
    const label = el("dt-custom-name").value.trim() || "custom activity";
    const unit = parseFloat(el("dt-custom-dur-unit").value) || 3600;
    const duration = Math.max(
      1,
      (parseFloat(el("dt-custom-duration").value) || 1) * unit,
    );
    customActs.push({
      id: 0,
      label,
      duration,
      color: EXTRA_COLORS[0],
      on: true,
    });
  } else {
    const duration = parseFloat(sel.value);
    const idx = PRESET_ACTIVITIES.findIndex((a) => a.duration === duration);
    if (idx >= 0) presetOn[idx] = true;
  }

  runCalculate({
    age,
    lifeExp,
    bpSystolic: systolic,
    bpDiastolic: diastolic,
    presetOn,
    customActs,
    nextId: customActs.length,
  });
});

el("dt-feat-prev").addEventListener("click", () =>
  setFeatured(S.featuredIdx - 1),
);
el("dt-feat-next").addEventListener("click", () =>
  setFeatured(S.featuredIdx + 1),
);

el("dt-auto-advance").addEventListener("click", () =>
  setAutoAdvance(!S.autoAdvance),
);

el("dt-log-toggle").addEventListener("click", () => {
  S.logScale = !S.logScale;
  el("dt-log-toggle").textContent = "Log scale: " + (S.logScale ? "ON" : "OFF");
  updateChart();
  encodeHash();
});

el("dt-tab-chart").addEventListener("click", () => showView("chart"));
el("dt-tab-waffle").addEventListener("click", () => showView("waffle"));

el("dt-sb-add-btn").addEventListener("click", () => {
  const label = el("dt-sb-name").value.trim();
  const unit = parseFloat(el("dt-sb-dur-unit").value) || 3600;
  const duration = Math.max(1, (parseFloat(el("dt-sb-dur").value) || 1) * unit);
  if (!label) {
    el("dt-sb-name").focus();
    return;
  }
  if (S.customActs.some((a) => a.label === label)) return;
  addCustomActivity(label, duration);
  el("dt-sb-name").value = "";
  el("dt-sb-dur").value = "1";
});

el("dt-reset-btn").addEventListener("click", reset);
el("dt-dead-reset").addEventListener("click", reset);

el("dt-share-btn").addEventListener("click", () => {
  navigator.clipboard.writeText(location.href).then(() => {
    const btn = el("dt-share-btn");
    btn.textContent = "\u2713 Copied!";
    setTimeout(() => {
      btn.innerHTML = "&#x1f517; Copy link";
    }, 2000);
  });
});

// ── Hash restore on load ──────────────────────────────────────────────────────
(function () {
  const decoded = decodeHash();
  if (decoded) {
    el("dt-age").value = decoded.age;
    el("dt-life-exp").value = decoded.lifeExp;
    el("dt-bp-systolic").value = decoded.bpSystolic;
    el("dt-bp-diastolic").value = decoded.bpDiastolic;
    applyHash(decoded);
  } else {
    updateBPDisplay();
  }
})();
