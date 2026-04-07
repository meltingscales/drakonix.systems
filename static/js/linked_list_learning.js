// Linked List Learning — REPL + SVG diagram

// ─── Singly linked list ───────────────────────────────────────────────────────

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
        if (!this.head) { this.head = node; }
        else { let c = this.head; while (c.next) c = c.next; c.next = node; }
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
        let c = this.head;
        for (let i = 0; i < index - 1; i++) c = c.next;
        node.next = c.next; c.next = node;
        this._size++;
    }

    delete(value) {
        if (!this.head) return false;
        if (this.head.value === value) { this.head = this.head.next; this._size--; return true; }
        let c = this.head;
        while (c.next) {
            if (c.next.value === value) { c.next = c.next.next; this._size--; return true; }
            c = c.next;
        }
        return false;
    }

    deleteAt(index) {
        if (index < 0 || index >= this._size)
            throw new RangeError(`Index ${index} out of bounds (size: ${this._size})`);
        if (index === 0) { this.head = this.head.next; this._size--; return; }
        let c = this.head;
        for (let i = 0; i < index - 1; i++) c = c.next;
        c.next = c.next.next;
        this._size--;
    }

    find(value) {
        let c = this.head, i = 0;
        while (c) { if (c.value === value) return i; c = c.next; i++; }
        return -1;
    }

    get(index) {
        if (index < 0 || index >= this._size)
            throw new RangeError(`Index ${index} out of bounds (size: ${this._size})`);
        let c = this.head;
        for (let i = 0; i < index; i++) c = c.next;
        return c.value;
    }

    set(index, value) {
        if (index < 0 || index >= this._size)
            throw new RangeError(`Index ${index} out of bounds (size: ${this._size})`);
        let c = this.head;
        for (let i = 0; i < index; i++) c = c.next;
        c.value = value;
    }

    reverse() {
        let prev = null, c = this.head;
        while (c) { const nxt = c.next; c.next = prev; prev = c; c = nxt; }
        this.head = prev;
    }

    length()  { return this._size; }

    toArray() {
        const arr = []; let c = this.head;
        while (c) { arr.push(c.value); c = c.next; }
        return arr;
    }

    clear()   { this.head = null; this._size = 0; }

    toString() {
        return this._size === 0 ? 'null' : this.toArray().join(' → ') + ' → null';
    }
}

// ─── Doubly linked list ───────────────────────────────────────────────────────

class DoublyNode {
    constructor(value) {
        this.value = value;
        this.prev  = null;
        this.next  = null;
    }
}

class DoublyLinkedList {
    constructor() {
        this.head  = null;
        this.tail  = null;
        this._size = 0;
    }

    append(value) {
        const node = new DoublyNode(value);
        if (!this.tail) { this.head = this.tail = node; }
        else { node.prev = this.tail; this.tail.next = node; this.tail = node; }
        this._size++;
    }

    prepend(value) {
        const node = new DoublyNode(value);
        if (!this.head) { this.head = this.tail = node; }
        else { node.next = this.head; this.head.prev = node; this.head = node; }
        this._size++;
    }

    insertAt(index, value) {
        if (index < 0 || index > this._size)
            throw new RangeError(`Index ${index} out of bounds (size: ${this._size})`);
        if (index === 0)          { this.prepend(value); return; }
        if (index === this._size) { this.append(value);  return; }
        const node = new DoublyNode(value);
        let c = this.head;
        for (let i = 0; i < index; i++) c = c.next;
        node.prev = c.prev; node.next = c;
        c.prev.next = node; c.prev = node;
        this._size++;
    }

    _unlink(node) {
        if (node.prev) node.prev.next = node.next; else this.head = node.next;
        if (node.next) node.next.prev = node.prev; else this.tail = node.prev;
        this._size--;
    }

    delete(value) {
        let c = this.head;
        while (c) { if (c.value === value) { this._unlink(c); return true; } c = c.next; }
        return false;
    }

    deleteAt(index) {
        if (index < 0 || index >= this._size)
            throw new RangeError(`Index ${index} out of bounds (size: ${this._size})`);
        let c = this.head;
        for (let i = 0; i < index; i++) c = c.next;
        this._unlink(c);
    }

    find(value) {
        let c = this.head, i = 0;
        while (c) { if (c.value === value) return i; c = c.next; i++; }
        return -1;
    }

    get(index) {
        if (index < 0 || index >= this._size)
            throw new RangeError(`Index ${index} out of bounds (size: ${this._size})`);
        let c = this.head;
        for (let i = 0; i < index; i++) c = c.next;
        return c.value;
    }

    set(index, value) {
        if (index < 0 || index >= this._size)
            throw new RangeError(`Index ${index} out of bounds (size: ${this._size})`);
        let c = this.head;
        for (let i = 0; i < index; i++) c = c.next;
        c.value = value;
    }

    reverse() {
        let c = this.head;
        while (c) { [c.prev, c.next] = [c.next, c.prev]; c = c.prev; }
        [this.head, this.tail] = [this.tail, this.head];
    }

    length()  { return this._size; }

    toArray() {
        const arr = []; let c = this.head;
        while (c) { arr.push(c.value); c = c.next; }
        return arr;
    }

    toArrayReverse() {
        const arr = []; let c = this.tail;
        while (c) { arr.push(c.value); c = c.prev; }
        return arr;
    }

    clear() { this.head = this.tail = null; this._size = 0; }

    toString() {
        return this._size === 0
            ? 'null'
            : 'null ← ' + this.toArray().join(' ⇄ ') + ' → null';
    }
}

// ─── State ───────────────────────────────────────────────────────────────────
let list = new LinkedList();
let currentMode = 'singly';
let _opLog = [];

// ─── SVG constants — singly ───────────────────────────────────────────────────
const NODE_W   = 140;
const NODE_H   = 60;
const GAP_W    = 50;
const STEP_W   = NODE_W + GAP_W;
const HEAD_X   = 80;
const CENTER_Y = 70;
const TOP_Y    = CENTER_Y - NODE_H / 2;
const NULL_W   = 60;
const SVG_H    = 140;

// ─── SVG constants — doubly ───────────────────────────────────────────────────
const DL_NODE_W  = 180;
const DL_PREV_W  = 50;
const DL_VAL_W   = 80;
const DL_NEXT_W  = 50;
const DL_NODE_H  = 60;
const DL_GAP_W   = 56;
const DL_STEP_W  = DL_NODE_W + DL_GAP_W;
const DL_HEAD_X  = 80;
const DL_CY      = 70;
const DL_TOP_Y   = DL_CY - DL_NODE_H / 2;
const DL_NEXT_Y  = DL_CY - 12;
const DL_PREV_Y  = DL_CY + 12;
const DL_SVG_H   = 150;

// ─── XML escaping ─────────────────────────────────────────────────────────────
function escXml(s) {
    return String(s)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;');
}

// ─── Operation tracking ───────────────────────────────────────────────────────
const TRACKED_OPS = new Set(['append','prepend','insertAt','delete','deleteAt','set','reverse','clear']);

function makeTrackedList(baseList) {
    return new Proxy(baseList, {
        get(target, prop) {
            const val = target[prop];
            if (typeof val !== 'function') return val;
            return function(...args) {
                const result = val.apply(target, args);
                if (TRACKED_OPS.has(String(prop))) {
                    _opLog.push({ op: String(prop), args: [...args] });
                }
                return result;
            };
        },
        set(target, prop, value) {
            target[prop] = value;
            return true;
        }
    });
}

// ─── Singly diagram ───────────────────────────────────────────────────────────
function renderSinglyDiagram(newIndices = new Set()) {
    const svg    = document.getElementById('ll-diagram');
    const values = list.toArray();
    const n      = values.length;

    const svgW = n === 0 ? 320 : HEAD_X + n * STEP_W + NULL_W + 20;

    svg.setAttribute('viewBox', `0 0 ${svgW} ${SVG_H}`);
    svg.setAttribute('width',   svgW);
    svg.setAttribute('height',  SVG_H);

    const parts = [];

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

    parts.push(`
      <text x="5" y="${CENTER_Y - 5}" class="ll-head-label">head</text>
      <line x1="44" y1="${CENTER_Y}" x2="${HEAD_X - 6}" y2="${CENTER_Y}"
            class="ll-arrow" marker-end="url(#arr)"/>`);

    for (let i = 0; i < n; i++) {
        const nx     = HEAD_X + i * STEP_W;
        const isLast = i === n - 1;
        const raw    = String(values[i]);
        const disp   = raw.length > 9 ? raw.slice(0, 8) + '…' : raw;
        const isNew  = newIndices.has(i);

        parts.push(`<g class="ll-node-group${isNew ? ' ll-node-new' : ''}" data-index="${i}" data-value="${escXml(raw)}">`);

        parts.push(`
          <rect x="${nx}" y="${TOP_Y}" width="${NODE_W}" height="${NODE_H}"
                rx="6" class="ll-node-rect"/>`);
        parts.push(`
          <line x1="${nx + NODE_W / 2}" y1="${TOP_Y}"
                x2="${nx + NODE_W / 2}" y2="${TOP_Y + NODE_H}"
                class="ll-divider"/>`);
        parts.push(`
          <text x="${nx + NODE_W / 4}" y="${CENTER_Y}"
                text-anchor="middle" dominant-baseline="middle"
                class="ll-val-text">${escXml(disp)}</text>`);
        parts.push(`
          <text x="${nx + NODE_W * 3 / 4}" y="${TOP_Y + 13}"
                text-anchor="middle" class="ll-next-label">next</text>`);

        if (isLast) {
            parts.push(`
              <text x="${nx + NODE_W * 3 / 4}" y="${CENTER_Y + 8}"
                    text-anchor="middle" dominant-baseline="middle"
                    class="ll-null-ptr">∅</text>`);
            const nullX = nx + NODE_W + GAP_W;
            parts.push(`
              <line x1="${nx + NODE_W}" y1="${CENTER_Y}"
                    x2="${nullX - 6}"  y2="${CENTER_Y}"
                    class="ll-arrow" marker-end="url(#arr)"/>`);
            parts.push(`
              <rect x="${nullX}" y="${TOP_Y + 10}"
                    width="${NULL_W}" height="${NODE_H - 20}"
                    rx="4" class="ll-null-rect"/>`);
            parts.push(`
              <text x="${nullX + NULL_W / 2}" y="${CENTER_Y}"
                    text-anchor="middle" dominant-baseline="middle"
                    class="ll-null-text">null</text>`);
        } else {
            parts.push(`
              <circle cx="${nx + NODE_W * 3 / 4}" cy="${CENTER_Y + 8}" r="5"
                      class="ll-ptr-dot"/>`);
            parts.push(`
              <line x1="${nx + NODE_W}" y1="${CENTER_Y}"
                    x2="${nx + NODE_W + GAP_W - 6}" y2="${CENTER_Y}"
                    class="ll-arrow" marker-end="url(#arr)"/>`);
        }

        parts.push(`
          <text x="${nx + NODE_W / 4}" y="${TOP_Y + NODE_H + 17}"
                text-anchor="middle" class="ll-index-label">[${i}]</text>`);

        parts.push(`</g>`);
    }

    svg.innerHTML = parts.join('');
}

// ─── Doubly diagram ───────────────────────────────────────────────────────────
function renderDoublyDiagram(newIndices = new Set()) {
    const svg    = document.getElementById('ll-diagram');
    const values = list.toArray();
    const n      = values.length;

    const svgW = n === 0 ? 320 : DL_HEAD_X + n * DL_STEP_W + NULL_W + 20;

    svg.setAttribute('viewBox', `0 0 ${svgW} ${DL_SVG_H}`);
    svg.setAttribute('width',   svgW);
    svg.setAttribute('height',  DL_SVG_H);

    const parts = [];

    parts.push(`
      <defs>
        <marker id="arr" markerWidth="8" markerHeight="6"
                refX="7" refY="3" orient="auto" markerUnits="strokeWidth">
          <polygon class="ll-arrowhead" points="0 0, 8 3, 0 6"/>
        </marker>
        <marker id="arr-back" markerWidth="8" markerHeight="6"
                refX="1" refY="3" orient="auto" markerUnits="strokeWidth">
          <polygon class="ll-arrowhead" points="8 0, 0 3, 8 6"/>
        </marker>
      </defs>`);

    if (n === 0) {
        parts.push(`
          <text x="10" y="${DL_CY - 6}"  class="ll-head-label">head / tail</text>
          <text x="10" y="${DL_CY + 14}" class="ll-null-label">→ null</text>`);
        svg.innerHTML = parts.join('');
        return;
    }

    parts.push(`
      <text x="5" y="${DL_CY - 5}" class="ll-head-label">head</text>
      <line x1="44" y1="${DL_CY}" x2="${DL_HEAD_X - 6}" y2="${DL_CY}"
            class="ll-arrow" marker-end="url(#arr)"/>`);

    for (let i = 0; i < n; i++) {
        const nx      = DL_HEAD_X + i * DL_STEP_W;
        const isFirst = i === 0;
        const isLast  = i === n - 1;
        const raw     = String(values[i]);
        const disp    = raw.length > 7 ? raw.slice(0, 6) + '…' : raw;
        const isNew   = newIndices.has(i);

        parts.push(`<g class="ll-node-group${isNew ? ' ll-node-new' : ''}" data-index="${i}" data-value="${escXml(raw)}">`);

        parts.push(`
          <rect x="${nx}" y="${DL_TOP_Y}" width="${DL_NODE_W}" height="${DL_NODE_H}"
                rx="6" class="ll-node-rect"/>`);
        parts.push(`
          <line x1="${nx + DL_PREV_W}" y1="${DL_TOP_Y}"
                x2="${nx + DL_PREV_W}" y2="${DL_TOP_Y + DL_NODE_H}"
                class="ll-divider"/>`);
        parts.push(`
          <line x1="${nx + DL_PREV_W + DL_VAL_W}" y1="${DL_TOP_Y}"
                x2="${nx + DL_PREV_W + DL_VAL_W}" y2="${DL_TOP_Y + DL_NODE_H}"
                class="ll-divider"/>`);
        parts.push(`
          <text x="${nx + DL_PREV_W / 2}" y="${DL_TOP_Y + 12}"
                text-anchor="middle" class="ll-next-label">prev</text>`);
        parts.push(`
          <text x="${nx + DL_PREV_W + DL_VAL_W + DL_NEXT_W / 2}" y="${DL_TOP_Y + 12}"
                text-anchor="middle" class="ll-next-label">next</text>`);
        parts.push(`
          <text x="${nx + DL_PREV_W + DL_VAL_W / 2}" y="${DL_CY}"
                text-anchor="middle" dominant-baseline="middle"
                class="ll-val-text">${escXml(disp)}</text>`);

        if (isFirst) {
            parts.push(`
              <text x="${nx + DL_PREV_W / 2}" y="${DL_CY + 7}"
                    text-anchor="middle" dominant-baseline="middle"
                    class="ll-null-ptr">∅</text>`);
        } else {
            parts.push(`
              <circle cx="${nx + DL_PREV_W / 2}" cy="${DL_PREV_Y}" r="5"
                      class="ll-ptr-dot"/>`);
        }

        if (isLast) {
            parts.push(`
              <text x="${nx + DL_PREV_W + DL_VAL_W + DL_NEXT_W / 2}" y="${DL_CY + 7}"
                    text-anchor="middle" dominant-baseline="middle"
                    class="ll-null-ptr">∅</text>`);
        } else {
            parts.push(`
              <circle cx="${nx + DL_PREV_W + DL_VAL_W + DL_NEXT_W / 2}" cy="${DL_NEXT_Y}" r="5"
                      class="ll-ptr-dot"/>`);
        }

        if (!isLast) {
            const x0 = nx + DL_NODE_W;
            const x1 = nx + DL_STEP_W;
            parts.push(`
              <line x1="${x0}" y1="${DL_NEXT_Y}" x2="${x1 - 6}" y2="${DL_NEXT_Y}"
                    class="ll-arrow" marker-end="url(#arr)"/>`);
            parts.push(`
              <line x1="${x1}" y1="${DL_PREV_Y}" x2="${x0 + 6}" y2="${DL_PREV_Y}"
                    class="ll-arrow" marker-end="url(#arr-back)"/>`);
        } else {
            const nullX = nx + DL_NODE_W + DL_GAP_W;
            parts.push(`
              <line x1="${nx + DL_NODE_W}" y1="${DL_NEXT_Y}"
                    x2="${nullX - 6}" y2="${DL_NEXT_Y}"
                    class="ll-arrow" marker-end="url(#arr)"/>`);
            parts.push(`
              <rect x="${nullX}" y="${DL_TOP_Y + 10}"
                    width="${NULL_W}" height="${DL_NODE_H - 20}"
                    rx="4" class="ll-null-rect"/>`);
            parts.push(`
              <text x="${nullX + NULL_W / 2}" y="${DL_CY}"
                    text-anchor="middle" dominant-baseline="middle"
                    class="ll-null-text">null</text>`);
        }

        parts.push(`
          <text x="${nx + DL_PREV_W + DL_VAL_W / 2}" y="${DL_TOP_Y + DL_NODE_H + 17}"
                text-anchor="middle" class="ll-index-label">[${i}]</text>`);

        parts.push(`</g>`);
    }

    svg.innerHTML = parts.join('');
}

// ─── Diagram dispatcher ───────────────────────────────────────────────────────
function renderDiagram(newIndices = new Set()) {
    if (currentMode === 'singly') renderSinglyDiagram(newIndices);
    else                          renderDoublyDiagram(newIndices);
}

// ─── Templates / snippets ─────────────────────────────────────────────────────
const SNIPPETS = {
    singly: [
        {
            label: 'Build 1–5',
            code: `[1, 2, 3, 4, 5].forEach(v => list.append(v));`,
        },
        {
            label: 'Inspect',
            code: `console.log(list.toString());\nconsole.log('length:', list.length());`,
        },
        {
            label: 'Find middle',
            code: `// Slow/fast pointer technique
let slow = list.head, fast = list.head;
while (fast && fast.next) {
  slow = slow.next;
  fast = fast.next.next;
}
console.log('middle:', slow ? slow.value : null);`,
        },
        {
            label: 'Remove duplicates',
            code: `// O(n) with a Set
const seen = new Set();
let curr = list.head, prev = null;
while (curr) {
  if (seen.has(curr.value)) {
    prev.next = curr.next;
    list._size--;
  } else {
    seen.add(curr.value);
    prev = curr;
  }
  curr = curr.next;
}`,
        },
        {
            label: 'Fibonacci',
            code: `let a = 0, b = 1;
for (let i = 0; i < 8; i++) {
  list.append(a);
  [a, b] = [b, a + b];
}`,
        },
        {
            label: 'Reverse & verify',
            code: `const before = list.toArray().join(', ');
list.reverse();
const after = list.toArray().join(', ');
console.log('before:', before);
console.log('after: ', after);`,
        },
        {
            label: 'Sum all values',
            code: `const sum = list.toArray().reduce((acc, v) => acc + v, 0);
console.log('sum:', sum);`,
        },
        {
            label: 'Detect cycle',
            code: `// Floyd's cycle detection (inject cycle manually to test)
let slow = list.head, fast = list.head;
while (fast && fast.next) {
  slow = slow.next;
  fast = fast.next.next;
  if (slow === fast) { console.log('cycle detected!'); slow; }
}
console.log('no cycle');`,
        },
    ],
    doubly: [
        {
            label: 'Build 1–5',
            code: `[1, 2, 3, 4, 5].forEach(v => list.append(v));`,
        },
        {
            label: 'Inspect',
            code: `console.log(list.toString());
console.log('head:', list.head?.value, '  tail:', list.tail?.value);`,
        },
        {
            label: 'Traverse forward',
            code: `let curr = list.head;
while (curr) {
  console.log(curr.value);
  curr = curr.next;
}`,
        },
        {
            label: 'Traverse backward',
            code: `// Use .prev pointers from tail
let curr = list.tail;
while (curr) {
  console.log(curr.value);
  curr = curr.prev;
}`,
        },
        {
            label: 'Remove duplicates',
            code: `// O(n) — _unlink handles prev/next rewiring
const seen = new Set();
let curr = list.head;
while (curr) {
  const next = curr.next;
  if (seen.has(curr.value)) list._unlink(curr);
  else seen.add(curr.value);
  curr = next;
}`,
        },
        {
            label: 'Reverse & verify',
            code: `const before = list.toArray().join(', ');
list.reverse();
console.log('forward: ', list.toArray().join(', '));
console.log('backward:', list.toArrayReverse().join(', '));`,
        },
        {
            label: 'Palindrome check',
            code: `// Compare forward and backward traversal
const fwd = list.toArray();
const bwd = list.toArrayReverse();
const ok  = fwd.every((v, i) => v === bwd[i]);
console.log(ok ? 'palindrome ✓' : 'not a palindrome');`,
        },
        {
            label: 'Sum all values',
            code: `const sum = list.toArray().reduce((acc, v) => acc + v, 0);
console.log('sum:', sum);`,
        },
    ],
};

// ─── Populate template buttons ────────────────────────────────────────────────
function updateTemplateButtons() {
    const grid = document.getElementById('ll-template-grid');
    grid.innerHTML = '';
    for (const s of SNIPPETS[currentMode]) {
        const btn = document.createElement('button');
        btn.className   = 'll-template-btn';
        btn.textContent = s.label;
        btn.dataset.code = s.code;
        grid.appendChild(btn);
    }
}

// ─── Code execution ──────────────────────────────────────────────────────────
function runCode(code) {
    _opLog = [];
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

    const trackedList = makeTrackedList(list);

    let result, error;
    const lines    = code.trim().split('\n');
    const lastLine = lines[lines.length - 1].trim();
    const wrapped  = [...lines.slice(0, -1), 'return ' + lastLine].join('\n');

    try {
        result = new Function('list', 'console', wrapped)(trackedList, fakeConsole);
    } catch (_) {
        try {
            result = new Function('list', 'console', code)(trackedList, fakeConsole);
        } catch (e) {
            error = e.message;
        }
    }

    return { logs, result, error, ops: [..._opLog] };
}

// ─── History / output panel ──────────────────────────────────────────────────
function escHtml(s) {
    return String(s)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;');
}

function fmtOpArgs(args) {
    return args.map(a =>
        a === null ? 'null' :
        a === undefined ? 'undefined' :
        typeof a === 'string' ? `"${a}"` :
        String(a)
    ).join(', ');
}

function addToHistory(code, result) {
    const panel = document.getElementById('ll-history');
    const entry = document.createElement('div');
    entry.className = 'll-entry';

    const codeHtml = code.trim().split('\n')
        .map(l => `<div class="ll-code-line">&gt; ${escHtml(l)}</div>`)
        .join('');

    let outHtml = '';

    // Operation log entries
    if (result.ops && result.ops.length > 0) {
        for (const { op, args } of result.ops) {
            outHtml += `<div class="ll-op-line">&#x2713; list.${escHtml(op)}(${escHtml(fmtOpArgs(args))})</div>`;
        }
    }

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

// ─── Mode switching ───────────────────────────────────────────────────────────
function switchMode(mode) {
    currentMode = mode;
    list = mode === 'singly' ? new LinkedList() : new DoublyLinkedList();

    document.getElementById('ll-workspace').dataset.mode = mode;

    document.querySelectorAll('.ll-tab').forEach(t =>
        t.classList.toggle('ll-tab-active', t.dataset.mode === mode)
    );

    updateTemplateButtons();
    renderDiagram();
    updateStats();

    document.getElementById('ll-history').innerHTML =
        `<div class="ll-entry ll-log-line">Switched to ${mode === 'singly' ? 'singly' : 'doubly'} linked list. List reset.</div>`;
}

// ─── Button handlers ─────────────────────────────────────────────────────────
function handleRun() {
    const code = document.getElementById('ll-input').value.trim();
    if (!code) return;
    const before = list.toArray();
    const result = runCode(code);
    const after  = list.toArray();

    // Determine which indices are new or changed (for animation)
    const newIndices = new Set();
    if (!result.error) {
        after.forEach((v, i) => {
            if (i >= before.length || String(before[i]) !== String(v)) {
                newIndices.add(i);
            }
        });
    }

    addToHistory(code, result);
    renderDiagram(newIndices);
    updateStats();
}

function handleReset() {
    list = currentMode === 'singly' ? new LinkedList() : new DoublyLinkedList();
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

// ─── Diagram interaction (click + drag) ──────────────────────────────────────
function computeDropGap(svgX, n) {
    // Returns gap index: 0 = before node[0], n = after node[n-1]
    if (currentMode === 'singly') {
        const raw = Math.round((svgX - HEAD_X + STEP_W / 2) / STEP_W);
        return Math.max(0, Math.min(n, raw));
    } else {
        const raw = Math.round((svgX - DL_HEAD_X + DL_STEP_W / 2) / DL_STEP_W);
        return Math.max(0, Math.min(n, raw));
    }
}

function svgXFromClient(svg, clientX) {
    const rect = svg.getBoundingClientRect();
    const viewBox = svg.viewBox.baseVal;
    return (clientX - rect.left) * (viewBox.width / rect.width);
}

function initDiagramInteraction() {
    const svg = document.getElementById('ll-diagram');
    let drag = null;

    svg.addEventListener('mousedown', (e) => {
        const g = e.target.closest('.ll-node-group');
        if (!g) return;
        e.preventDefault();
        drag = {
            fromIdx: parseInt(g.dataset.index),
            fromVal: g.dataset.value,
            startClientX: e.clientX,
            moved: false,
        };
        g.style.opacity = '0.6';
        g.style.cursor  = 'grabbing';
    });

    svg.addEventListener('mousemove', (e) => {
        if (!drag) return;
        if (Math.abs(e.clientX - drag.startClientX) > 8) drag.moved = true;
    });

    window.addEventListener('mouseup', (e) => {
        if (!drag) return;
        const state = drag;
        drag = null;

        // Restore opacity
        const g = svg.querySelector(`[data-index="${state.fromIdx}"]`);
        if (g) { g.style.opacity = ''; g.style.cursor = ''; }

        const n = list.length();

        if (state.moved && n > 1) {
            // Drag-to-reorder: compute target gap
            const svgX  = svgXFromClient(svg, e.clientX);
            const gap   = computeDropGap(svgX, n);
            const from  = state.fromIdx;
            // Effective insertAt index after deleteAt(from)
            const insertIdx = gap > from ? gap - 1 : gap;

            if (insertIdx !== from) {
                const ta = document.getElementById('ll-input');
                ta.value =
                    `// Move node from [${from}] to [${insertIdx}]\n` +
                    `const _v = list.get(${from});\n` +
                    `list.deleteAt(${from});\n` +
                    `list.insertAt(${insertIdx}, _v);`;
                ta.focus();
            }
        } else {
            // Click: fill textarea with get, delete, or set options as a comment menu
            const idx = state.fromIdx;
            const val = state.fromVal;
            const ta  = document.getElementById('ll-input');
            ta.value  = `list.get(${idx})`;
            ta.focus();
            // Select all so user can easily replace
            ta.select();
        }
    });
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

    document.querySelectorAll('.ll-tab').forEach(tab => {
        tab.addEventListener('click', () => switchMode(tab.dataset.mode));
    });

    document.getElementById('ll-template-grid').addEventListener('click', e => {
        const btn = e.target.closest('.ll-template-btn');
        if (!btn) return;
        document.getElementById('ll-input').value = btn.dataset.code;
        document.getElementById('ll-input').focus();
    });

    initDiagramInteraction();
    updateTemplateButtons();
    renderDiagram();
    updateStats();

    document.getElementById('ll-history').innerHTML =
        '<div class="ll-entry ll-log-line">Ready. Pick a template or type a command and press Run (or Ctrl+Enter). Click a node to inspect it, drag to reorder.</div>';
});
