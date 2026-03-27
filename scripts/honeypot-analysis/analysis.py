#!/usr/bin/env python3
"""
Honeypot Hits Analysis — dashboard-style visualizations.

Usage:
    uv run analysis.py --input ../../honeypot-hits.json
    uv run analysis.py --input ../../honeypot-hits.json --output ./reports --top 25
    uv run analysis.py --input ../../honeypot-hits.json --report geo --show
    uv run analysis.py --input ../../honeypot-hits.json --report all --no-save

Reports: all, overview, geo, endpoints, actors, useragents, payloads, headers, campaigns
"""

import argparse
import json
import re
import sys
from pathlib import Path

import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import matplotlib.patches as mpatches
import seaborn as sns
from rich.console import Console
from rich.table import Table
from rich import box as rich_box

console = Console(highlight=False)

# ── Colour theme ──────────────────────────────────────────────────────────────
BG       = "#0d1117"
SURFACE  = "#161b22"
SURFACE2 = "#21262d"
TEXT     = "#e6edf3"
MUTED    = "#8b949e"
BORDER   = "#30363d"

P = [
    "#4cc9f0",  # cyan
    "#4361ee",  # blue
    "#7209b7",  # purple
    "#f72585",  # pink
    "#f77f00",  # orange
    "#2dc653",  # green
    "#ffd60a",  # yellow
    "#e63946",  # red
    "#06d6a0",  # teal
    "#118ab2",  # deep blue
    "#9b72cf",  # lavender
    "#ff6b6b",  # salmon
]

plt.rcParams.update({
    "figure.facecolor":              BG,
    "axes.facecolor":                SURFACE,
    "axes.edgecolor":                BORDER,
    "axes.labelcolor":               TEXT,
    "axes.titlecolor":               TEXT,
    "axes.titlesize":                11,
    "axes.labelsize":                9,
    "xtick.color":                   MUTED,
    "ytick.color":                   MUTED,
    "xtick.labelsize":               8,
    "ytick.labelsize":               8,
    "text.color":                    TEXT,
    "grid.color":                    SURFACE2,
    "grid.alpha":                    1.0,
    "font.family":                   "monospace",
    "figure.dpi":                    150,
    "axes.spines.top":               False,
    "axes.spines.right":             False,
    "legend.facecolor":              SURFACE,
    "legend.edgecolor":              BORDER,
    "legend.fontsize":               8,
})


# ── Classification rules ──────────────────────────────────────────────────────

CLOUD_PROVIDERS = [
    ("Amazon/AWS",      r"(?i)amazon|AS16509|AS14618|AS38895"),
    ("Microsoft/Azure", r"(?i)microsoft|AS8075|AS8070"),
    ("Tencent",         r"(?i)tencent|AS132203|AS45090"),
    ("Google/GCP",      r"(?i)google|AS15169|AS396982"),
    ("DigitalOcean",    r"(?i)digitalocean|AS14061"),
    ("Alibaba",         r"(?i)alibaba|alicloud|AS37963|AS45102"),
    ("OVH",             r"(?i)\bovh\b|AS16276"),
    ("Hetzner",         r"(?i)hetzner|AS24940"),
    ("Linode/Akamai",   r"(?i)linode|akamai|AS63949"),
    ("Vultr",           r"(?i)vultr|AS20473"),
    ("Contabo",         r"(?i)contabo|AS51167"),
]

SLUG_CATEGORIES = [
    ("WordPress",          r"(?i)/wp-|wordpress"),
    ("PHP Probe",          r"\.php($|\?)"),
    ("Login / Auth",       r"(?i)login|auth|signin|sign-in|logon|sso|oauth"),
    ("Admin / Panel",      r"(?i)admin|panel|console|manager|dashboard|portal|control"),
    ("Config / Secrets",   r"(?i)config|secret|\.env|\.git|credential"),
    ("API Endpoint",       r"(?i)/api/|api-"),
    ("Database",           r"(?i)mongo|mysql|redis|postgres|sql|phpmyadmin|adminer"),
    ("Shell / Backdoor",   r"(?i)shell|backdoor|cmd|exec|webshell|c99|r57|\.jsp"),
    ("Crawler Target",     r"(?i)sitemap|robots\.txt|favicon|feed"),
    ("Honeypot Generated", r"^[a-z]+-[a-z]+-\d+$"),
]

UA_FAMILIES = [
    ("Known Bot",   r"(?i)claudebot|googlebot|bingbot|yandexbot|baiduspider|semrushbot|ahrefsbot|mj12bot|petalbot"),
    ("Script/Tool", r"(?i)curl|wget|python|go-http|java/|ruby|perl|axios|node-fetch|libwww|okhttp|httpx|masscan|zgrab|nmap"),
    ("Mobile",      r"(?i)iphone|android.*mobile|blackberry"),
    ("Chrome",      r"(?i)chrome"),
    ("Firefox",     r"(?i)firefox"),
    ("Safari",      r"(?i)safari"),
]

BODY_CATEGORIES = [
    ("PHP Probe",     r"(?i)<\?php|phpunit|md5\("),
    ("SQL Injection", r"(?i)select .+from|union .+select|' or '|1=1"),
    ("XML/SOAP",      r"(?i)<\?xml|<soap:|<s:envelope"),
    ("JSON Payload",  r"^\s*[\[{]"),
    ("Form Data",     r"(?i)username=|password=|user=|pass=|cmd="),
    ("Shell Command", r"(?i);cat |;ls |;id |/etc/passwd|/bin/sh"),
]

WEEKDAY_ORDER = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"]


def classify(value: str, rules: list[tuple[str, str]], default: str = "Other") -> str:
    for label, pattern in rules:
        if re.search(pattern, str(value)):
            return label
    return default


def extract_slug_pattern(slug: str) -> str | None:
    m = re.match(r'^([a-z]+-[a-z]+)-\d+$', slug)
    return m.group(1) if m else None


# ── Data loading ──────────────────────────────────────────────────────────────

def load_data(path: Path) -> pd.DataFrame:
    console.log(f"[cyan]Loading[/] {path}")
    with open(path, "r", encoding="utf-8") as f:
        raw = json.load(f)
    df = pd.DataFrame(raw)
    console.log(f"[green]OK[/] {len(df):,} records")

    # Timestamps
    df["ts"]      = pd.to_datetime(df["timestamp"], utc=True)
    df["date"]    = df["ts"].dt.date
    df["hour"]    = df["ts"].dt.hour
    df["weekday"] = df["ts"].dt.day_name()

    # Clean country — ipinfo rate-limit errors come back as JSON objects
    df["country"] = df["country"].apply(
        lambda c: "Unknown" if (not isinstance(c, str) or c.strip().startswith("{")) else c.strip()
    )

    # Parse embedded headers JSON
    console.log("[cyan]Parsing headers...[/]")
    def parse_headers(s):
        try:
            return json.loads(s) if s else {}
        except Exception:
            return {}

    h = df["headers"].apply(parse_headers)
    df["user_agent"]  = h.apply(lambda x: x.get("user-agent", ""))
    df["host_header"] = h.apply(lambda x: x.get("host", ""))
    df["accept_lang"] = h.apply(lambda x: x.get("accept-language", ""))
    df["proto"]       = h.apply(lambda x: x.get("x-forwarded-proto", "unknown"))

    # ASN + org name
    df["asn"]      = df["org"].str.extract(r"^(AS\d+)").fillna("Unknown")
    df["org_name"] = df["org"].str.replace(r"^AS\d+\s*", "", regex=True).str.strip()

    # Derived classifications
    console.log("[cyan]Classifying...[/]")
    df["slug_category"] = df["slug"].apply(lambda s: classify(s, SLUG_CATEGORIES))
    df["slug_pattern"]  = df["slug"].apply(extract_slug_pattern)
    df["ua_family"]     = df["user_agent"].apply(lambda u: classify(u, UA_FAMILIES) if u else "No UA")
    df["body_category"] = df["body"].apply(
        lambda b: classify(b, BODY_CATEGORIES) if b else "Empty"
    )
    df["cloud"]         = df["org"].apply(lambda o: classify(o, CLOUD_PROVIDERS, "Other/ISP"))
    df["host_type"]     = df["host_header"].apply(
        lambda h: "Raw IP" if re.match(r"^\d+\.\d+\.\d+\.\d+", str(h)) else ("Domain" if h else "None")
    )

    console.log("[green]OK Data ready[/]")
    return df


# ── Plotting helpers ──────────────────────────────────────────────────────────

def fig_title(fig, title: str, subtitle: str = ""):
    y = 0.995
    fig.text(0.012, y, title, fontsize=14, fontweight="bold", color=TEXT, va="top")
    if subtitle:
        fig.text(0.012, y - 0.032, subtitle, fontsize=8, color=MUTED, va="top")


def hbar(ax, labels, values, colors=None, total=None):
    """Horizontal bar chart. Returns bars."""
    if colors is None:
        colors = [P[i % len(P)] for i in range(len(labels))]
    y = range(len(labels))
    bars = ax.barh(list(y), values, color=colors, height=0.65, zorder=2)
    ax.set_yticks(list(y))
    ax.set_yticklabels(labels, fontsize=7.5)
    ax.invert_yaxis()
    ax.grid(axis="x", zorder=1)
    ax.set_axisbelow(True)
    max_val = max(values) if values else 1
    for bar, val in zip(bars, values):
        pct = f"  {val/total*100:.1f}%" if total else ""
        ax.text(
            bar.get_width() + max_val * 0.01,
            bar.get_y() + bar.get_height() / 2,
            f"{val:,}{pct}",
            va="center", ha="left", fontsize=7, color=MUTED,
        )
    ax.set_xlim(0, max_val * 1.22)
    return bars


def vbar(ax, labels, values, color=P[0]):
    ax.bar(labels, values, color=color, zorder=2, width=0.7)
    ax.grid(axis="y", zorder=1)
    ax.set_axisbelow(True)
    ax.tick_params(axis="x", rotation=35)


def pie(ax, labels, values, colors=None):
    if colors is None:
        colors = [P[i % len(P)] for i in range(len(labels))]
    wedges, texts, autotexts = ax.pie(
        values, labels=None, colors=colors,
        autopct="%1.1f%%", pctdistance=0.78,
        wedgeprops={"linewidth": 0.5, "edgecolor": BG},
        startangle=140,
    )
    for t in autotexts:
        t.set_fontsize(7)
        t.set_color(BG)
    ax.legend(
        wedges, [f"{l} ({v:,})" for l, v in zip(labels, values)],
        loc="lower center", bbox_to_anchor=(0.5, -0.18),
        ncol=2, fontsize=7, framealpha=0.3,
    )


def kpi_box(ax, label, value, color=P[0]):
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.axis("off")
    rect = mpatches.FancyBboxPatch(
        (0.05, 0.12), 0.9, 0.76,
        boxstyle="round,pad=0.04", linewidth=1.2,
        edgecolor=color, facecolor=SURFACE2,
    )
    ax.add_patch(rect)
    ax.text(0.5, 0.68, str(value), ha="center", va="center", fontsize=15,
            fontweight="bold", color=color)
    ax.text(0.5, 0.25, label, ha="center", va="center", fontsize=7.5, color=MUTED)


def save_or_show(fig, output_dir: Path | None, name: str, show: bool):
    if output_dir:
        out = output_dir / f"{name}.png"
        fig.savefig(out, bbox_inches="tight", facecolor=BG)
        console.log(f"[green]OK[/] {out}")
    if show:
        plt.show()
    plt.close(fig)


# ── Report 1: Overview ────────────────────────────────────────────────────────

def report_overview(df: pd.DataFrame, output_dir, top: int, show: bool):
    total     = len(df)
    unique_ip = df["ip"].nunique()
    unique_sl = df["slug"].nunique()
    days      = (df["ts"].max() - df["ts"].min()).days + 1
    avg_day   = total / days
    date_counts = df.groupby("date").size()
    peak_date = date_counts.idxmax()
    peak_val  = date_counts.max()

    fig = plt.figure(figsize=(16, 10))
    fig.patch.set_facecolor(BG)
    fig_title(fig, "Overview", f"{df['ts'].min().date()} -> {df['ts'].max().date()}  |  {total:,} total hits")

    gs = gridspec.GridSpec(3, 5, figure=fig, hspace=0.55, wspace=0.45,
                           top=0.90, bottom=0.06, left=0.04, right=0.98)

    # KPI row
    kpis = [
        ("Total Hits",    f"{total:,}",       P[0]),
        ("Unique IPs",    f"{unique_ip:,}",   P[1]),
        ("Unique Slugs",  f"{unique_sl:,}",   P[2]),
        ("Days Covered",  str(days),           P[4]),
        (f"Peak ({peak_date})", f"{peak_val:,}", P[7]),
    ]
    for i, (label, val, color) in enumerate(kpis):
        ax = fig.add_subplot(gs[0, i])
        kpi_box(ax, label, val, color)

    # Hits per day
    ax2 = fig.add_subplot(gs[1, :])
    daily = df.groupby("date").size().reset_index(name="hits")
    ax2.plot(daily["date"].astype(str), daily["hits"], color=P[0], lw=2, marker="o",
             markersize=3, zorder=2)
    ax2.fill_between(daily["date"].astype(str), daily["hits"], alpha=0.15, color=P[0])
    ax2.axhline(avg_day, color=MUTED, lw=1, ls="--", label=f"avg {avg_day:,.0f}/day")
    ax2.set_title("Hits per Day")
    ax2.grid(axis="y", zorder=1)
    ax2.set_axisbelow(True)
    ax2.tick_params(axis="x", rotation=30)
    ax2.legend()

    # Hits per hour
    ax3 = fig.add_subplot(gs[2, :3])
    hourly = df.groupby("hour").size()
    ax3.bar(hourly.index, hourly.values, color=P[1], zorder=2, width=0.8)
    ax3.set_title("Hits by Hour of Day (UTC)")
    ax3.set_xlabel("Hour")
    ax3.set_xticks(range(0, 24, 2))
    ax3.grid(axis="y", zorder=1)
    ax3.set_axisbelow(True)

    # Hits per weekday
    ax4 = fig.add_subplot(gs[2, 3:])
    wd = df.groupby("weekday").size().reindex(WEEKDAY_ORDER, fill_value=0)
    ax4.bar(wd.index, wd.values, color=P[2], zorder=2, width=0.7)
    ax4.set_title("Hits by Day of Week")
    ax4.tick_params(axis="x", rotation=35)
    ax4.grid(axis="y", zorder=1)
    ax4.set_axisbelow(True)

    save_or_show(fig, output_dir, "01_overview", show)


# ── Report 2: Geography ───────────────────────────────────────────────────────

def report_geo(df: pd.DataFrame, output_dir, top: int, show: bool):
    fig = plt.figure(figsize=(16, 11))
    fig.patch.set_facecolor(BG)
    fig_title(fig, "Geography", f"Top {top} countries | ASNs | cloud providers")

    gs = gridspec.GridSpec(2, 3, figure=fig, hspace=0.5, wspace=0.6,
                           top=0.90, bottom=0.06, left=0.10, right=0.98)

    # Top countries
    ax1 = fig.add_subplot(gs[:, 0])
    ctry = df[df["country"] != "Unknown"]["country"].value_counts().head(top)
    hbar(ax1, ctry.index.tolist(), ctry.values.tolist(),
         colors=[P[i % len(P)] for i in range(len(ctry))], total=len(df))
    ax1.set_title(f"Top {top} Countries")

    # Top ASNs
    ax2 = fig.add_subplot(gs[0, 1:])
    asn_counts = df.groupby(["asn", "org_name"]).size().reset_index(name="n")
    asn_counts["label"] = asn_counts["asn"] + " " + asn_counts["org_name"].str[:30]
    top_asns = asn_counts.nlargest(top, "n")
    hbar(ax2, top_asns["label"].tolist(), top_asns["n"].tolist(),
         colors=[P[i % len(P)] for i in range(len(top_asns))], total=len(df))
    ax2.set_title(f"Top {top} ASNs / Organisations")

    # Cloud provider breakdown
    ax3 = fig.add_subplot(gs[1, 1:])
    cloud = df["cloud"].value_counts()
    hbar(ax3, cloud.index.tolist(), cloud.values.tolist(),
         colors=[P[i % len(P)] for i in range(len(cloud))], total=len(df))
    ax3.set_title("Hits by Infrastructure Provider")

    save_or_show(fig, output_dir, "02_geography", show)


# ── Report 3: Endpoints ───────────────────────────────────────────────────────

def report_endpoints(df: pd.DataFrame, output_dir, top: int, show: bool):
    fig = plt.figure(figsize=(16, 11))
    fig.patch.set_facecolor(BG)
    fig_title(fig, "Endpoints / Slugs", "What attackers are probing")

    gs = gridspec.GridSpec(2, 2, figure=fig, hspace=0.5, wspace=0.55,
                           top=0.90, bottom=0.06, left=0.18, right=0.98)

    # Top raw slugs (truncated for display)
    ax1 = fig.add_subplot(gs[:, 0])
    slugs = df["slug"].value_counts().head(top)
    labels = [s[:50] + "..." if len(s) > 50 else s for s in slugs.index]
    hbar(ax1, labels, slugs.values.tolist(), total=len(df))
    ax1.set_title(f"Top {top} Slugs")

    # Slug category breakdown
    ax2 = fig.add_subplot(gs[0, 1])
    cats = df["slug_category"].value_counts()
    pie(ax2, cats.index.tolist(), cats.values.tolist())
    ax2.set_title("Slug Categories")

    # Top honeypot-generated slug patterns
    ax3 = fig.add_subplot(gs[1, 1])
    patterns = (
        df[df["slug_pattern"].notna()]["slug_pattern"]
        .value_counts().head(top)
    )
    if not patterns.empty:
        hbar(ax3, patterns.index.tolist(), patterns.values.tolist(),
             colors=[P[i % len(P)] for i in range(len(patterns))], total=len(df))
        ax3.set_title(f"Top Honeypot Slug Patterns\n(word-word-NNN base patterns)")
    else:
        ax3.text(0.5, 0.5, "No honeypot-pattern slugs found",
                 ha="center", va="center", color=MUTED)
        ax3.axis("off")
        ax3.set_title("Honeypot Slug Patterns")

    save_or_show(fig, output_dir, "03_endpoints", show)


# ── Report 4: Actors ──────────────────────────────────────────────────────────

def report_actors(df: pd.DataFrame, output_dir, top: int, show: bool):
    fig = plt.figure(figsize=(16, 11))
    fig.patch.set_facecolor(BG)
    fig_title(fig, "Actors / IPs", "Who is attacking and how")

    gs = gridspec.GridSpec(2, 2, figure=fig, hspace=0.55, wspace=0.55,
                           top=0.90, bottom=0.06, left=0.14, right=0.98)

    # Top IPs by hit count
    ax1 = fig.add_subplot(gs[:, 0])
    ip_hits = df["ip"].value_counts().head(top)
    hbar(ax1, ip_hits.index.tolist(), ip_hits.values.tolist(), total=len(df))
    ax1.set_title(f"Top {top} IPs by Hit Count")

    # Hit frequency distribution (log-scale histogram)
    ax2 = fig.add_subplot(gs[0, 1])
    ip_counts = df["ip"].value_counts()
    bins = [1, 2, 3, 5, 10, 25, 50, 100, 500, 2000, 10000]
    counts, edges = np.histogram(ip_counts.values, bins=bins)
    labels_hist = [f"{edges[i]:.0f}–{edges[i+1]:.0f}" for i in range(len(edges) - 1)]
    ax2.bar(range(len(counts)), counts, color=P[1], zorder=2)
    ax2.set_xticks(range(len(counts)))
    ax2.set_xticklabels(labels_hist, rotation=40, ha="right", fontsize=7)
    ax2.set_title("IP Hit-Frequency Distribution")
    ax2.set_ylabel("Number of IPs")
    ax2.set_xlabel("Hits per IP")
    ax2.grid(axis="y", zorder=1)
    ax2.set_axisbelow(True)

    # Scanning breadth — IPs with most unique slugs
    ax3 = fig.add_subplot(gs[1, 1])
    breadth = df.groupby("ip")["slug"].nunique().nlargest(top)
    hbar(ax3, breadth.index.tolist(), breadth.values.tolist(),
         colors=[P[i % len(P)] for i in range(len(breadth))], total=None)
    ax3.set_title(f"Top {top} IPs by Unique Slugs Probed\n(scanning breadth)")

    save_or_show(fig, output_dir, "04_actors", show)


# ── Report 4b: Burst Detection ────────────────────────────────────────────────

def report_bursts(df: pd.DataFrame, output_dir, top: int, show: bool):
    """IPs with the most hits within any rolling 5-minute window."""
    console.log("[cyan]Computing bursts (5-min rolling window)...[/]")
    window = pd.Timedelta("5min")
    results = []
    for ip, grp in df.groupby("ip"):
        ts_sorted = grp["ts"].sort_values().reset_index(drop=True)
        if len(ts_sorted) < 5:
            continue
        max_burst = 0
        for i in range(len(ts_sorted)):
            end = ts_sorted[i] + window
            burst = int((ts_sorted <= end).sum()) - i  # hits from i onward within window
            # More accurately: window ending at ts_sorted[i]
            start_t = ts_sorted[i] - window
            count = int(((ts_sorted >= start_t) & (ts_sorted <= ts_sorted[i])).sum())
            max_burst = max(max_burst, count)
        results.append({"ip": ip, "max_burst_5min": max_burst,
                         "total_hits": len(grp),
                         "country": grp["country"].iloc[0],
                         "cloud": grp["cloud"].iloc[0]})

    burst_df = pd.DataFrame(results).sort_values("max_burst_5min", ascending=False)

    fig = plt.figure(figsize=(16, 7))
    fig.patch.set_facecolor(BG)
    fig_title(fig, "Burst Detection", "Max hits by a single IP in any 5-minute window")

    gs = gridspec.GridSpec(1, 2, figure=fig, wspace=0.5,
                           top=0.88, bottom=0.10, left=0.14, right=0.98)

    ax1 = fig.add_subplot(gs[0, 0])
    top_burst = burst_df.head(top)
    labels = [f"{r['ip']} ({r['country']})" for _, r in top_burst.iterrows()]
    hbar(ax1, labels, top_burst["max_burst_5min"].tolist(),
         colors=[P[7]] * len(top_burst), total=None)
    ax1.set_title(f"Top {top} IPs — Max Burst (5 min)")
    ax1.set_xlabel("Hits in 5-min window")

    ax2 = fig.add_subplot(gs[0, 1])
    ax2.scatter(burst_df["total_hits"], burst_df["max_burst_5min"],
                alpha=0.3, s=12, color=P[0], zorder=2)
    ax2.set_xlabel("Total Hits")
    ax2.set_ylabel("Max Burst (5 min)")
    ax2.set_title("Total Hits vs. Max Burst")
    ax2.grid(zorder=1)
    ax2.set_axisbelow(True)

    save_or_show(fig, output_dir, "04b_bursts", show)
    return burst_df


# ── Report 5: User Agents ─────────────────────────────────────────────────────

def report_useragents(df: pd.DataFrame, output_dir, top: int, show: bool):
    fig = plt.figure(figsize=(16, 11))
    fig.patch.set_facecolor(BG)
    fig_title(fig, "User Agents", "Browser families, bots, and spoofing indicators")

    gs = gridspec.GridSpec(2, 2, figure=fig, hspace=0.55, wspace=0.55,
                           top=0.90, bottom=0.08, left=0.26, right=0.98)

    # Top raw UAs (truncated)
    ax1 = fig.add_subplot(gs[:, 0])
    ua_counts = df["user_agent"].replace("", "(empty)").value_counts().head(top)
    labels = [u[:55] + "..." if len(u) > 55 else u for u in ua_counts.index]
    hbar(ax1, labels, ua_counts.values.tolist(), total=len(df))
    ax1.set_title(f"Top {top} User Agents")

    # UA family pie
    ax2 = fig.add_subplot(gs[0, 1])
    fam = df["ua_family"].value_counts()
    pie(ax2, fam.index.tolist(), fam.values.tolist())
    ax2.set_title("UA Family Breakdown")

    # Spoofed mobile UA: "Mobile" UA from cloud providers
    ax3 = fig.add_subplot(gs[1, 1])
    spoofed = (
        df[(df["ua_family"] == "Mobile") & (df["cloud"] != "Other/ISP")]
        .groupby("cloud").size().sort_values(ascending=False)
    )
    if not spoofed.empty:
        hbar(ax3, spoofed.index.tolist(), spoofed.values.tolist(),
             colors=[P[7]] * len(spoofed), total=None)
        ax3.set_title("Mobile UA from Cloud Infra\n(likely spoofed)")
    else:
        ax3.text(0.5, 0.5, "No spoofed mobile UAs detected",
                 ha="center", va="center", color=MUTED)
        ax3.axis("off")
        ax3.set_title("Mobile UA from Cloud Infra")

    save_or_show(fig, output_dir, "05_useragents", show)


# ── Report 6: Payloads ────────────────────────────────────────────────────────

def report_payloads(df: pd.DataFrame, output_dir, top: int, show: bool):
    fig = plt.figure(figsize=(16, 10))
    fig.patch.set_facecolor(BG)
    non_empty = df["body"].astype(bool).sum()
    fig_title(fig, "Payloads / Bodies",
              f"{non_empty:,} non-empty bodies ({non_empty/len(df)*100:.1f}% of {len(df):,} total)")

    gs = gridspec.GridSpec(2, 2, figure=fig, hspace=0.55, wspace=0.55,
                           top=0.90, bottom=0.10, left=0.22, right=0.98)

    # Empty vs non-empty pie
    ax1 = fig.add_subplot(gs[0, 0])
    pie(ax1,
        ["Empty", "Non-empty"],
        [len(df) - non_empty, non_empty],
        colors=[MUTED, P[4]])
    ax1.set_title("Body Presence")

    # Body category breakdown
    ax2 = fig.add_subplot(gs[0, 1])
    cats = df[df["body"].astype(bool)]["body_category"].value_counts()
    hbar(ax2, cats.index.tolist(), cats.values.tolist(),
         colors=[P[i % len(P)] for i in range(len(cats))], total=non_empty)
    ax2.set_title("Body Category (non-empty only)")

    # Top body strings — strip control chars before handing to matplotlib
    def sanitize(s: str) -> str:
        s = "".join(c if c.isprintable() else "?" for c in s)
        return s[:70] + "..." if len(s) > 70 else s

    ax3 = fig.add_subplot(gs[1, :])
    top_bodies = df[df["body"].astype(bool)]["body"].value_counts().head(top)
    labels = [sanitize(b) for b in top_bodies.index]
    hbar(ax3, labels, top_bodies.values.tolist(),
         colors=[P[i % len(P)] for i in range(len(top_bodies))], total=non_empty)
    ax3.set_title(f"Top {top} Body Strings (non-empty)")

    save_or_show(fig, output_dir, "06_payloads", show)


# ── Report 7: Headers ─────────────────────────────────────────────────────────

def report_headers(df: pd.DataFrame, output_dir, top: int, show: bool):
    fig = plt.figure(figsize=(16, 10))
    fig.patch.set_facecolor(BG)
    fig_title(fig, "Headers Intelligence", "Host targeting, protocol, accept-language, connection patterns")

    gs = gridspec.GridSpec(2, 2, figure=fig, hspace=0.55, wspace=0.55,
                           top=0.90, bottom=0.08, left=0.18, right=0.98)

    # Host header type (domain vs raw IP vs none)
    ax1 = fig.add_subplot(gs[0, 0])
    host_types = df["host_type"].value_counts()
    pie(ax1, host_types.index.tolist(), host_types.values.tolist(),
        colors=[P[0], P[4], MUTED])
    ax1.set_title("Host Header: Domain vs Raw IP\n(Raw IP = mass scan, not domain-targeted)")

    # Protocol
    ax2 = fig.add_subplot(gs[0, 1])
    proto = df["proto"].value_counts()
    pie(ax2, proto.index.tolist(), proto.values.tolist(),
        colors=[P[1], P[7], MUTED])
    ax2.set_title("Protocol (x-forwarded-proto)")

    # Accept-Language top values
    ax3 = fig.add_subplot(gs[1, 0])
    # Normalise: take the primary language code (before comma or ;)
    lang_primary = (
        df["accept_lang"]
        .replace("", pd.NA).dropna()
        .str.split(r"[,;]").str[0].str.strip()
        .value_counts().head(top)
    )
    hbar(ax3, lang_primary.index.tolist(), lang_primary.values.tolist(),
         colors=[P[i % len(P)] for i in range(len(lang_primary))],
         total=df["accept_lang"].astype(bool).sum())
    ax3.set_title("Accept-Language (primary locale)")

    # Country × protocol cross-tab as stacked bar (top 10 countries)
    ax4 = fig.add_subplot(gs[1, 1])
    top_countries = df[df["country"] != "Unknown"]["country"].value_counts().head(12).index
    ct = (
        df[df["country"].isin(top_countries)]
        .groupby(["country", "proto"]).size().unstack(fill_value=0)
    )
    ct = ct.loc[ct.sum(axis=1).sort_values(ascending=False).index]
    bottom = np.zeros(len(ct))
    for i, col in enumerate(ct.columns):
        ax4.barh(ct.index.tolist(), ct[col].values, left=bottom,
                 color=P[i % len(P)], label=col, height=0.7)
        bottom += ct[col].values
    ax4.invert_yaxis()
    ax4.set_title("Top Countries × Protocol")
    ax4.legend(loc="lower right", fontsize=7)
    ax4.grid(axis="x", zorder=1)
    ax4.set_axisbelow(True)
    ax4.tick_params(axis="y", labelsize=7.5)

    save_or_show(fig, output_dir, "07_headers", show)


# ── Report 8: Campaigns ───────────────────────────────────────────────────────

def report_campaigns(df: pd.DataFrame, output_dir, top: int, show: bool):
    fig = plt.figure(figsize=(16, 11))
    fig.patch.set_facecolor(BG)
    fig_title(fig, "Campaigns & Correlation",
              "Country × slug category | slug patterns by unique IP count")

    gs = gridspec.GridSpec(2, 2, figure=fig, hspace=0.55, wspace=0.45,
                           top=0.90, bottom=0.08, left=0.14, right=0.98)

    # Heatmap: top countries × slug categories
    ax1 = fig.add_subplot(gs[:, 0])
    top_ctry = df[df["country"] != "Unknown"]["country"].value_counts().head(15).index
    hm = (
        df[df["country"].isin(top_ctry)]
        .groupby(["country", "slug_category"]).size()
        .unstack(fill_value=0)
    )
    hm = hm.loc[hm.sum(axis=1).sort_values(ascending=False).index]
    sns.heatmap(
        hm, ax=ax1, cmap="YlOrRd",
        linewidths=0.3, linecolor=BG,
        annot=True, fmt="d", annot_kws={"size": 6},
        cbar_kws={"shrink": 0.6},
    )
    ax1.set_title("Country × Slug Category Heatmap")
    ax1.set_xlabel("")
    ax1.tick_params(axis="x", rotation=40, labelsize=7)
    ax1.tick_params(axis="y", labelsize=7.5)

    # Most "targeted" slugs by unique IP count
    ax2 = fig.add_subplot(gs[0, 1])
    targeted = df.groupby("slug")["ip"].nunique().nlargest(top)
    labels = [s[:45] + "..." if len(s) > 45 else s for s in targeted.index]
    hbar(ax2, labels, targeted.values.tolist(),
         colors=[P[i % len(P)] for i in range(len(targeted))], total=None)
    ax2.set_title(f"Top {top} Slugs by Unique Attacker IPs\n(highest coordination)")

    # Cloud × slug category
    ax3 = fig.add_subplot(gs[1, 1])
    cloud_slug = (
        df.groupby(["cloud", "slug_category"]).size().unstack(fill_value=0)
    )
    cloud_slug = cloud_slug.loc[cloud_slug.sum(axis=1).sort_values(ascending=False).index]
    bottom = np.zeros(len(cloud_slug))
    for i, col in enumerate(cloud_slug.columns):
        ax3.barh(cloud_slug.index.tolist(), cloud_slug[col].values, left=bottom,
                 color=P[i % len(P)], label=col, height=0.7)
        bottom += cloud_slug[col].values
    ax3.invert_yaxis()
    ax3.set_title("Infrastructure Provider × Slug Category")
    ax3.legend(loc="lower right", fontsize=6, ncol=2)
    ax3.grid(axis="x", zorder=1)
    ax3.set_axisbelow(True)
    ax3.tick_params(axis="y", labelsize=7.5)

    save_or_show(fig, output_dir, "08_campaigns", show)


# ── Rich terminal summary ─────────────────────────────────────────────────────

def print_summary(df: pd.DataFrame):
    console.rule("[bold cyan]Honeypot Summary[/]")

    days = (df["ts"].max() - df["ts"].min()).days + 1
    t = Table(box=rich_box.ROUNDED, show_header=False, padding=(0, 2))
    t.add_column(style="dim")
    t.add_column(style="bold cyan")
    rows = [
        ("Total hits",      f"{len(df):,}"),
        ("Unique IPs",      f"{df['ip'].nunique():,}"),
        ("Unique slugs",    f"{df['slug'].nunique():,}"),
        ("Unique countries",f"{(df['country'] != 'Unknown').sum() and df[df['country'] != 'Unknown']['country'].nunique():,}"),
        ("Date range",      f"{df['ts'].min().date()} -> {df['ts'].max().date()} ({days} days)"),
        ("Avg hits/day",    f"{len(df)/days:,.0f}"),
        ("Non-empty bodies",f"{df['body'].astype(bool).sum():,} ({df['body'].astype(bool).mean()*100:.1f}%)"),
    ]
    for k, v in rows:
        t.add_row(k, v)
    console.print(t)

    console.rule("[bold cyan]Top 10 Countries[/]")
    ctry_t = Table(box=rich_box.SIMPLE, show_header=True)
    ctry_t.add_column("Country", style="cyan")
    ctry_t.add_column("Hits", justify="right")
    ctry_t.add_column("%", justify="right", style="dim")
    for country, cnt in df[df["country"] != "Unknown"]["country"].value_counts().head(10).items():
        ctry_t.add_row(country, f"{cnt:,}", f"{cnt/len(df)*100:.1f}%")
    console.print(ctry_t)

    console.rule("[bold cyan]Top 10 IPs[/]")
    ip_t = Table(box=rich_box.SIMPLE, show_header=True)
    ip_t.add_column("IP", style="yellow")
    ip_t.add_column("Hits", justify="right")
    ip_t.add_column("Country")
    ip_t.add_column("Cloud")
    for ip, cnt in df["ip"].value_counts().head(10).items():
        row = df[df["ip"] == ip].iloc[0]
        ip_t.add_row(ip, f"{cnt:,}", row["country"], row["cloud"])
    console.print(ip_t)

    console.rule("[bold cyan]Slug Categories[/]")
    sc_t = Table(box=rich_box.SIMPLE, show_header=True)
    sc_t.add_column("Category", style="magenta")
    sc_t.add_column("Hits", justify="right")
    sc_t.add_column("%", justify="right", style="dim")
    for cat, cnt in df["slug_category"].value_counts().items():
        sc_t.add_row(cat, f"{cnt:,}", f"{cnt/len(df)*100:.1f}%")
    console.print(sc_t)


# ── CLI ───────────────────────────────────────────────────────────────────────

ALL_REPORTS = ["overview", "geo", "endpoints", "actors", "bursts",
               "useragents", "payloads", "headers", "campaigns"]

REPORT_FNS = {
    "overview":   report_overview,
    "geo":        report_geo,
    "endpoints":  report_endpoints,
    "actors":     report_actors,
    "useragents": report_useragents,
    "payloads":   report_payloads,
    "headers":    report_headers,
    "campaigns":  report_campaigns,
}


def main():
    parser = argparse.ArgumentParser(
        description="Honeypot hits dashboard analysis",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="Reports: " + ", ".join(ALL_REPORTS) + ", all",
    )
    parser.add_argument("--input",  "-i", required=True, type=Path, help="Path to honeypot-hits.json")
    parser.add_argument("--output", "-o", type=Path, default=Path("reports"),
                        help="Output directory for PNG reports (default: ./reports)")
    parser.add_argument("--top",    "-n", type=int, default=20,
                        help="Top-N entries in ranked charts (default: 20)")
    parser.add_argument("--report", "-r", default="all",
                        help="Which report to run: all | " + " | ".join(ALL_REPORTS))
    parser.add_argument("--show",   action="store_true",
                        help="Display figures interactively (in addition to saving)")
    parser.add_argument("--no-save", action="store_true",
                        help="Don't save PNG files (useful with --show)")
    args = parser.parse_args()

    if not args.input.exists():
        console.print(f"[red]File not found:[/] {args.input}")
        sys.exit(1)

    output_dir = None if args.no_save else args.output
    if output_dir:
        output_dir.mkdir(parents=True, exist_ok=True)
        console.log(f"[cyan]Output ->[/] {output_dir.resolve()}")

    df = load_data(args.input)
    print_summary(df)

    selected = ALL_REPORTS if args.report == "all" else [args.report]
    for name in selected:
        if name == "bursts":
            report_bursts(df, output_dir, args.top, args.show)
        elif name in REPORT_FNS:
            console.log(f"[cyan]Rendering[/] {name}...")
            REPORT_FNS[name](df, output_dir, args.top, args.show)
        else:
            console.print(f"[yellow]Unknown report:[/] {name}. Valid: {', '.join(ALL_REPORTS)}, all")

    if output_dir:
        pngs = list(output_dir.glob("*.png"))
        console.rule("[bold green]Done[/]")
        console.print(f"[green]OK[/] {len(pngs)} report(s) saved to [cyan]{output_dir.resolve()}[/]")


if __name__ == "__main__":
    main()
