// Linked List Learning — REPL + SVG diagram
// The `list` variable is injected into user code at execution time.

// ─── Data structure ──────────────────────────────────────────────────────────

class Node {
    constructor(value) {
        this.value = value;
        this.next  = null;
    }
}

class LinkedList {
    constructor() {
        this.head  = null;
        this._size = 0;
    }

    append(value) {
        const node = new Node(value);
        if (!this.head) {
            this.head = node;
        } else {
            let curr = this.head;
            while (curr.next) curr = curr.next;
            curr.next = node;
        }
        this._size++;
    }

    prepend(value) {
        const node = new Node(value);
        node.next  = this.head;
        this.head  = node;
        this._size++;
    }

    insertAt(index, value) {
        if (index < 0 || index > this._size)
            throw new RangeError(`Index ${index} out of bounds (size: ${this._size})`);
        if (index === 0) { this.prepend(value); return; }
        const node = new Node(value);
        let curr = this.head;
        for (let i = 0; i < index - 1; i++) curr = curr.next;
        node.next  = curr.next;
        curr.next  = node;
        this._size++;
    }

    delete(value) {
        if (!this.head) return false;
        if (this.head.value === value) {
            this.head = this.head.next;
            this._size--;
            return true;
        }
        let curr = this.head;
        while (curr.next) {
            if (curr.next.value === value) {
                curr.next = curr.next.next;
                this._size--;
                return true;
            }
            curr = curr.next;
        }
        return false;
    }

    deleteAt(index) {
        if (index < 0 || index >= this._size)
            throw new RangeError(`Index ${index} out of bounds (size: ${this._size})`);
        if (index === 0) { this.head = this.head.next; this._size--; return; }
        let curr = this.head;
        for (let i = 0; i < index - 1; i++) curr = curr.next;
        curr.next = curr.next.next;
        this._size--;
    }

    find(value) {
        let curr = this.head, i = 0;
        while (curr) {
            if (curr.value === value) return i;
            curr = curr.next; i++;
        }
        return -1;
    }

    get(index) {
        if (index < 0 || index >= this._size)
            throw new RangeError(`Index ${index} out of bounds (size: ${this._size})`);
        let curr = this.head;
        for (let i = 0; i < index; i++) curr = curr.next;
        return curr.value;
    }

    set(index, value) {
        if (index < 0 || index >= this._size)
            throw new RangeError(`Index ${index} out of bounds (size: ${this._size})`);
        let curr = this.head;
        for (let i = 0; i < index; i++) curr = curr.next;
        curr.value = value;
    }

    reverse() {
        let prev = null, curr = this.head;
        while (curr) {
            const next = curr.next;
            curr.next  = prev;
            prev = curr;
            curr = next;
        }
        this.head = prev;
    }

    length()  { return this._size; }

    toArray() {
        const arr = [];
        let curr = this.head;
        while (curr) { arr.push(curr.value); curr = curr.next; }
        return arr;
    }

    clear()   { this.head = null; this._size = 0; }

    toString() {
        if (!this.head) return 'null';
        return this.toArray().join(' → ') + ' → null';
    }
}

// ─── Global list instance ────────────────────────────────────────────────────
let list = new LinkedList();

// ─── SVG constants ───────────────────────────────────────────────────────────
const NODE_W   = 140;
const NODE_H   = 60;
const GAP_W    = 50;
const STEP_W   = NODE_W + GAP_W;
const HEAD_X   = 80;      // x where the first node left-edge sits
const CENTER_Y = 70;
const TOP_Y    = CENTER_Y - NODE_H / 2;   // = 40
const NULL_W   = 60;
const SVG_H    = 140;

// ─── SVG rendering ───────────────────────────────────────────────────────────
function escXml(s) {
    return String(s)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;');
}

function renderDiagram() {
    const svg    = document.getElementById('ll-diagram');
    const values = list.toArray();
    const n      = values.length;

    const svgW = n === 0
        ? 320
        : HEAD_X + n * STEP_W + NULL_W + 20;

    svg.setAttribute('viewBox', `0 0 ${svgW} ${SVG_H}`);
    svg.setAttribute('width',   svgW);
    svg.setAttribute('height',  SVG_H);

    const parts = [];

    // Arrowhead marker — fill is controlled by CSS class .ll-arrowhead
    parts.push(`
      <defs>
        <marker id="arr" markerWidth="8" markerHeight="6"
                refX="7" refY="3" orient="auto" markerUnits="strokeWidth">
          <polygon class="ll-arrowhead" points="0 0, 8 3, 0 6"/>
        </marker>
      </defs>`);

    if (n === 0) {
        parts.push(`
          <text x="10" y="${CENTER_Y - 6}"  class="ll-head-label">head</text>
          <text x="10" y="${CENTER_Y + 14}" class="ll-null-label">→ null</text>`);
        svg.innerHTML = parts.join('');
        return;
    }

    // "head" label + arrow into first node
    parts.push(`
      <text x="5" y="${CENTER_Y - 5}" class="ll-head-label">head</text>
      <line x1="44" y1="${CENTER_Y}" x2="${HEAD_X - 6}" y2="${CENTER_Y}"
            class="ll-arrow" marker-end="url(#arr)"/>`);

    for (let i = 0; i < n; i++) {
        const nx     = HEAD_X + i * STEP_W;
        const isLast = i === n - 1;
        const raw    = String(values[i]);
        const disp   = raw.length > 9 ? raw.slice(0, 8) + '…' : raw;

        // Node rectangle
        parts.push(`
          <rect x="${nx}" y="${TOP_Y}" width="${NODE_W}" height="${NODE_H}"
                rx="6" class="ll-node-rect"/>`);

        // Vertical divider between value | next compartments
        parts.push(`
          <line x1="${nx + NODE_W / 2}" y1="${TOP_Y}"
                x2="${nx + NODE_W / 2}" y2="${TOP_Y + NODE_H}"
                class="ll-divider"/>`);

        // Value text (left compartment, centered)
        parts.push(`
          <text x="${nx + NODE_W / 4}" y="${CENTER_Y}"
                text-anchor="middle" dominant-baseline="middle"
                class="ll-val-text">${escXml(disp)}</text>`);

        // "next" micro-label (top of right compartment)
        parts.push(`
          <text x="${nx + NODE_W * 3 / 4}" y="${TOP_Y + 13}"
                text-anchor="middle" class="ll-next-label">next</text>`);

        if (isLast) {
            // ∅ symbol = null pointer
            parts.push(`
              <text x="${nx + NODE_W * 3 / 4}" y="${CENTER_Y + 8}"
                    text-anchor="middle" dominant-baseline="middle"
                    class="ll-null-ptr">∅</text>`);

            // Arrow from right edge → null box
            const nullX = nx + NODE_W + GAP_W;
            parts.push(`
              <line x1="${nx + NODE_W}" y1="${CENTER_Y}"
                    x2="${nullX - 6}"  y2="${CENTER_Y}"
                    class="ll-arrow" marker-end="url(#arr)"/>`);

            // Null box (dashed)
            parts.push(`
              <rect x="${nullX}" y="${TOP_Y + 10}"
                    width="${NULL_W}" height="${NODE_H - 20}"
                    rx="4" class="ll-null-rect"/>`);
            parts.push(`
              <text x="${nullX + NULL_W / 2}" y="${CENTER_Y}"
                    text-anchor="middle" dominant-baseline="middle"
                    class="ll-null-text">null</text>`);
        } else {
            // Filled dot = live pointer
            parts.push(`
              <circle cx="${nx + NODE_W * 3 / 4}" cy="${CENTER_Y + 8}" r="5"
                      class="ll-ptr-dot"/>`);

            // Arrow → next node
            parts.push(`
              <line x1="${nx + NODE_W}" y1="${CENTER_Y}"
                    x2="${nx + NODE_W + GAP_W - 6}" y2="${CENTER_Y}"
                    class="ll-arrow" marker-end="url(#arr)"/>`);
        }

        // Index label below the value compartment
        parts.push(`
          <text x="${nx + NODE_W / 4}" y="${TOP_Y + NODE_H + 17}"
                text-anchor="middle" class="ll-index-label">[${i}]</text>`);
    }

    svg.innerHTML = parts.join('');
}

// ─── Code execution ──────────────────────────────────────────────────────────
function runCode(code) {
    const logs = [];
    const fakeConsole = {
        log: (...args) => logs.push(
            args.map(v =>
                v === null      ? 'null'      :
                v === undefined ? 'undefined' :
                typeof v === 'object' ? JSON.stringify(v) : String(v)
            ).join(' ')
        )
    };

    let result, error;

    // Try to auto-return the value of the last expression so the user
    // can write `list.length()` and see the result without `return`.
    const lines    = code.trim().split('\n');
    const lastLine = lines[lines.length - 1].trim();
    const wrapped  = [...lines.slice(0, -1), 'return ' + lastLine].join('\n');

    try {
        result = new Function('list', 'console', wrapped)(list, fakeConsole);
    } catch (_syntaxError) {
        // Last line was a statement (e.g. if/for/let) — run without return
        try {
            result = new Function('list', 'console', code)(list, fakeConsole);
        } catch (e) {
            error = e.message;
        }
    }

    return { logs, result, error };
}

// ─── History / output panel ──────────────────────────────────────────────────
function escHtml(s) {
    return String(s)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;');
}

function addToHistory(code, result) {
    const panel = document.getElementById('ll-history');
    const entry = document.createElement('div');
    entry.className = 'll-entry';

    const codeHtml = code.trim().split('\n')
        .map(l => `<div class="ll-code-line">&gt; ${escHtml(l)}</div>`)
        .join('');

    let outHtml = '';
    for (const line of result.logs)
        outHtml += `<div class="ll-log-line">${escHtml(line)}</div>`;
    if (result.error)
        outHtml += `<div class="ll-err-line">&#x26A0; ${escHtml(result.error)}</div>`;
    else if (result.result !== undefined)
        outHtml += `<div class="ll-ret-line">&#x2190; ${escHtml(JSON.stringify(result.result))}</div>`;

    entry.innerHTML = codeHtml + outHtml;
    panel.appendChild(entry);
    panel.scrollTop = panel.scrollHeight;
}

// ─── Stats ───────────────────────────────────────────────────────────────────
function updateStats() {
    document.getElementById('ll-length').textContent = list.length();
}

// ─── Button handlers ─────────────────────────────────────────────────────────
function handleRun() {
    const code = document.getElementById('ll-input').value.trim();
    if (!code) return;
    const result = runCode(code);
    addToHistory(code, result);
    renderDiagram();
    updateStats();
}

function handleReset() {
    list.clear();
    renderDiagram();
    updateStats();
    document.getElementById('ll-history').innerHTML =
        '<div class="ll-entry ll-log-line">List reset.</div>';
}

function handleClearInput() {
    const ta = document.getElementById('ll-input');
    ta.value = '';
    ta.focus();
}

// ─── Init ────────────────────────────────────────────────────────────────────
document.addEventListener('DOMContentLoaded', () => {
    document.getElementById('ll-run-btn')  .addEventListener('click', handleRun);
    document.getElementById('ll-reset-btn').addEventListener('click', handleReset);
    document.getElementById('ll-clear-btn').addEventListener('click', handleClearInput);

    document.getElementById('ll-input').addEventListener('keydown', e => {
        if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) {
            e.preventDefault();
            handleRun();
        }
    });

    renderDiagram();
    updateStats();

    document.getElementById('ll-history').innerHTML =
        '<div class="ll-entry ll-log-line">Ready. Type a command and press Run (or Ctrl+Enter).</div>';
});
