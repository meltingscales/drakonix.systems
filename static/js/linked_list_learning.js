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

// ─── SVG constants — singly (vertical) ───────────────────────────────────────
const SV_NODE_W  = 160;
const SV_NODE_H  = 60;
const SV_VAL_H   = 40;   // height of value compartment
const SV_NEXT_H  = 20;   // height of next pointer compartment
const SV_GAP_H   = 44;
const SV_STEP_H  = SV_NODE_H + SV_GAP_H;
const SV_CX      = 100;  // horizontal centre of nodes
const SV_NX      = SV_CX - SV_NODE_W / 2;
const SV_HEAD_Y  = 50;
const SV_NULL_H  = 34;
const SV_NULL_W  = 60;
const SV_SVG_W   = 220;

// ─── SVG constants — doubly (vertical) ───────────────────────────────────────
const DV_NODE_W  = 160;
const DV_PREV_H  = 18;
const DV_VAL_H   = 30;
const DV_NEXT_H  = 18;
const DV_NODE_H  = DV_PREV_H + DV_VAL_H + DV_NEXT_H;  // 66
const DV_GAP_H   = 50;
const DV_STEP_H  = DV_NODE_H + DV_GAP_H;
const DV_CX      = 100;
const DV_NX      = DV_CX - DV_NODE_W / 2;
const DV_HEAD_Y  = 50;
const DV_FWD_X   = DV_CX + 20;  // x for forward (↓ next) arrows
const DV_BCK_X   = DV_CX - 20;  // x for backward (↑ prev) arrows
const DV_NULL_H  = 34;
const DV_NULL_W  = 60;
const DV_SVG_W   = 220;

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

// ─── Singly diagram (vertical) ───────────────────────────────────────────────
function renderSinglyDiagram(newIndices = new Set()) {
    const svg    = document.getElementById('ll-diagram');
    const values = list.toArray();
    const n      = values.length;

    const svgH = n === 0 ? 100 : SV_HEAD_Y + n * SV_STEP_H + SV_GAP_H + SV_NULL_H + 10;

    svg.setAttribute('viewBox', `0 0 ${SV_SVG_W} ${svgH}`);
    svg.setAttribute('width',   SV_SVG_W);
    svg.setAttribute('height',  svgH);

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
          <text x="${SV_CX}" y="22" text-anchor="middle" class="ll-head-label">head</text>
          <text x="${SV_CX}" y="42" text-anchor="middle" class="ll-null-label">↓ null</text>`);
        svg.innerHTML = parts.join('');
        return;
    }

    // head label + arrow pointing down
    parts.push(`
      <text x="${SV_CX}" y="20" text-anchor="middle" class="ll-head-label">head</text>
      <line x1="${SV_CX}" y1="26" x2="${SV_CX}" y2="${SV_HEAD_Y - 6}"
            class="ll-arrow" marker-end="url(#arr)"/>`);

    for (let i = 0; i < n; i++) {
        const ny     = SV_HEAD_Y + i * SV_STEP_H;
        const isLast = i === n - 1;
        const raw    = String(values[i]);
        const disp   = raw.length > 9 ? raw.slice(0, 8) + '…' : raw;
        const isNew  = newIndices.has(i);

        parts.push(`<g class="ll-node-group${isNew ? ' ll-node-new' : ''}" data-index="${i}" data-value="${escXml(raw)}">`);

        // Node rect
        parts.push(`
          <rect x="${SV_NX}" y="${ny}" width="${SV_NODE_W}" height="${SV_NODE_H}"
                rx="6" class="ll-node-rect"/>`);

        // Horizontal divider: value (top) / next (bottom)
        parts.push(`
          <line x1="${SV_NX}" y1="${ny + SV_VAL_H}"
                x2="${SV_NX + SV_NODE_W}" y2="${ny + SV_VAL_H}"
                class="ll-divider"/>`);

        // Value text (centred in top compartment)
        parts.push(`
          <text x="${SV_CX}" y="${ny + SV_VAL_H / 2}"
                text-anchor="middle" dominant-baseline="middle"
                class="ll-val-text">${escXml(disp)}</text>`);

        // "next" label in bottom compartment
        parts.push(`
          <text x="${SV_NX + 8}" y="${ny + SV_VAL_H + SV_NEXT_H / 2}"
                dominant-baseline="middle" class="ll-next-label">next</text>`);

        if (isLast) {
            // ∅ in next compartment
            parts.push(`
              <text x="${SV_NX + SV_NODE_W - 16}" y="${ny + SV_VAL_H + SV_NEXT_H / 2 + 1}"
                    text-anchor="middle" dominant-baseline="middle"
                    class="ll-null-ptr">∅</text>`);
            // Arrow + null box below
            const nullY = ny + SV_NODE_H + SV_GAP_H;
            parts.push(`
              <line x1="${SV_CX}" y1="${ny + SV_NODE_H}"
                    x2="${SV_CX}" y2="${nullY - 6}"
                    class="ll-arrow" marker-end="url(#arr)"/>`);
            parts.push(`
              <rect x="${SV_CX - SV_NULL_W / 2}" y="${nullY}"
                    width="${SV_NULL_W}" height="${SV_NULL_H}"
                    rx="4" class="ll-null-rect"/>`);
            parts.push(`
              <text x="${SV_CX}" y="${nullY + SV_NULL_H / 2}"
                    text-anchor="middle" dominant-baseline="middle"
                    class="ll-null-text">null</text>`);
        } else {
            // Dot in next compartment
            parts.push(`
              <circle cx="${SV_NX + SV_NODE_W - 16}" cy="${ny + SV_VAL_H + SV_NEXT_H / 2}"
                      r="5" class="ll-ptr-dot"/>`);
            // Arrow to next node
            parts.push(`
              <line x1="${SV_CX}" y1="${ny + SV_NODE_H}"
                    x2="${SV_CX}" y2="${ny + SV_STEP_H - 6}"
                    class="ll-arrow" marker-end="url(#arr)"/>`);
        }

        // Index label to the right
        parts.push(`
          <text x="${SV_NX + SV_NODE_W + 8}" y="${ny + SV_VAL_H / 2}"
                dominant-baseline="middle" class="ll-index-label">[${i}]</text>`);

        // Edit / delete icons (shown on hover, inside value compartment right edge)
        const iconY = ny + SV_VAL_H / 2;
        const editX = SV_NX + SV_NODE_W - 28;
        const delX  = SV_NX + SV_NODE_W - 10;
        parts.push(`
          <g class="ll-node-icon" data-action="edit" data-index="${i}">
            <circle cx="${editX}" cy="${iconY}" r="9" class="ll-icon-circle"/>
            <text x="${editX}" y="${iconY}" class="ll-icon-text">✎</text>
          </g>
          <g class="ll-node-icon" data-action="delete" data-index="${i}">
            <circle cx="${delX}" cy="${iconY}" r="9" class="ll-icon-circle"/>
            <text x="${delX}" y="${iconY}" class="ll-icon-text">✕</text>
          </g>`);

        parts.push(`</g>`);
    }

    svg.innerHTML = parts.join('');
}

// ─── Doubly diagram (vertical) ───────────────────────────────────────────────
function renderDoublyDiagram(newIndices = new Set()) {
    const svg    = document.getElementById('ll-diagram');
    const values = list.toArray();
    const n      = values.length;

    const svgH = n === 0 ? 100 : DV_HEAD_Y + n * DV_STEP_H + DV_GAP_H + DV_NULL_H + 10;

    svg.setAttribute('viewBox', `0 0 ${DV_SVG_W} ${svgH}`);
    svg.setAttribute('width',   DV_SVG_W);
    svg.setAttribute('height',  svgH);

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
          <text x="${DV_CX}" y="22" text-anchor="middle" class="ll-head-label">head / tail</text>
          <text x="${DV_CX}" y="42" text-anchor="middle" class="ll-null-label">↓ null</text>`);
        svg.innerHTML = parts.join('');
        return;
    }

    // head label + arrow pointing down
    parts.push(`
      <text x="${DV_CX}" y="20" text-anchor="middle" class="ll-head-label">head</text>
      <line x1="${DV_CX}" y1="26" x2="${DV_CX}" y2="${DV_HEAD_Y - 6}"
            class="ll-arrow" marker-end="url(#arr)"/>`);

    for (let i = 0; i < n; i++) {
        const ny      = DV_HEAD_Y + i * DV_STEP_H;
        const isFirst = i === 0;
        const isLast  = i === n - 1;
        const raw     = String(values[i]);
        const disp    = raw.length > 9 ? raw.slice(0, 8) + '…' : raw;
        const isNew   = newIndices.has(i);

        parts.push(`<g class="ll-node-group${isNew ? ' ll-node-new' : ''}" data-index="${i}" data-value="${escXml(raw)}">`);

        // Node rect
        parts.push(`
          <rect x="${DV_NX}" y="${ny}" width="${DV_NODE_W}" height="${DV_NODE_H}"
                rx="6" class="ll-node-rect"/>`);

        // Horizontal dividers: prev | value | next
        parts.push(`
          <line x1="${DV_NX}" y1="${ny + DV_PREV_H}"
                x2="${DV_NX + DV_NODE_W}" y2="${ny + DV_PREV_H}"
                class="ll-divider"/>`);
        parts.push(`
          <line x1="${DV_NX}" y1="${ny + DV_PREV_H + DV_VAL_H}"
                x2="${DV_NX + DV_NODE_W}" y2="${ny + DV_PREV_H + DV_VAL_H}"
                class="ll-divider"/>`);

        // Compartment labels
        parts.push(`
          <text x="${DV_NX + 8}" y="${ny + DV_PREV_H / 2}"
                dominant-baseline="middle" class="ll-next-label">prev</text>`);
        parts.push(`
          <text x="${DV_NX + 8}" y="${ny + DV_PREV_H + DV_VAL_H + DV_NEXT_H / 2}"
                dominant-baseline="middle" class="ll-next-label">next</text>`);

        // Value text (centred in middle compartment)
        parts.push(`
          <text x="${DV_CX}" y="${ny + DV_PREV_H + DV_VAL_H / 2}"
                text-anchor="middle" dominant-baseline="middle"
                class="ll-val-text">${escXml(disp)}</text>`);

        // Prev compartment pointer: ∅ for head, dot otherwise
        if (isFirst) {
            parts.push(`
              <text x="${DV_NX + DV_NODE_W - 16}" y="${ny + DV_PREV_H / 2}"
                    text-anchor="middle" dominant-baseline="middle"
                    class="ll-null-ptr">∅</text>`);
        } else {
            parts.push(`
              <circle cx="${DV_NX + DV_NODE_W - 16}" cy="${ny + DV_PREV_H / 2}"
                      r="5" class="ll-ptr-dot"/>`);
        }

        // Next compartment pointer: ∅ for tail, dot otherwise
        if (isLast) {
            parts.push(`
              <text x="${DV_NX + DV_NODE_W - 16}" y="${ny + DV_PREV_H + DV_VAL_H + DV_NEXT_H / 2}"
                    text-anchor="middle" dominant-baseline="middle"
                    class="ll-null-ptr">∅</text>`);
        } else {
            parts.push(`
              <circle cx="${DV_NX + DV_NODE_W - 16}" cy="${ny + DV_PREV_H + DV_VAL_H + DV_NEXT_H / 2}"
                      r="5" class="ll-ptr-dot"/>`);
        }

        // Arrows in the gap below (between this node and the next)
        if (!isLast) {
            const y0 = ny + DV_NODE_H;
            const y1 = ny + DV_STEP_H;
            // Forward (next ↓) arrow on right side
            parts.push(`
              <line x1="${DV_FWD_X}" y1="${y0}" x2="${DV_FWD_X}" y2="${y1 - 6}"
                    class="ll-arrow" marker-end="url(#arr)"/>`);
            // Backward (prev ↑) arrow on left side
            parts.push(`
              <line x1="${DV_BCK_X}" y1="${y1}" x2="${DV_BCK_X}" y2="${y0 + 6}"
                    class="ll-arrow" marker-end="url(#arr-back)"/>`);
        } else {
            // Null box below last node
            const nullY = ny + DV_NODE_H + DV_GAP_H;
            parts.push(`
              <line x1="${DV_CX}" y1="${ny + DV_NODE_H}"
                    x2="${DV_CX}" y2="${nullY - 6}"
                    class="ll-arrow" marker-end="url(#arr)"/>`);
            parts.push(`
              <rect x="${DV_CX - DV_NULL_W / 2}" y="${nullY}"
                    width="${DV_NULL_W}" height="${DV_NULL_H}"
                    rx="4" class="ll-null-rect"/>`);
            parts.push(`
              <text x="${DV_CX}" y="${nullY + DV_NULL_H / 2}"
                    text-anchor="middle" dominant-baseline="middle"
                    class="ll-null-text">null</text>`);
        }

        // Index label to the right
        parts.push(`
          <text x="${DV_NX + DV_NODE_W + 8}" y="${ny + DV_PREV_H + DV_VAL_H / 2}"
                dominant-baseline="middle" class="ll-index-label">[${i}]</text>`);

        // Edit / delete icons (shown on hover, inside value compartment right edge)
        const iconY = ny + DV_PREV_H + DV_VAL_H / 2;
        const editX = DV_NX + DV_NODE_W - 28;
        const delX  = DV_NX + DV_NODE_W - 10;
        parts.push(`
          <g class="ll-node-icon" data-action="edit" data-index="${i}">
            <circle cx="${editX}" cy="${iconY}" r="9" class="ll-icon-circle"/>
            <text x="${editX}" y="${iconY}" class="ll-icon-text">✎</text>
          </g>
          <g class="ll-node-icon" data-action="delete" data-index="${i}">
            <circle cx="${delX}" cy="${iconY}" r="9" class="ll-icon-circle"/>
            <text x="${delX}" y="${iconY}" class="ll-icon-text">✕</text>
          </g>`);

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
function computeDropGap(svgY, n) {
    // Returns gap index: 0 = before node[0], n = after node[n-1]
    if (currentMode === 'singly') {
        const raw = Math.round((svgY - SV_HEAD_Y + SV_STEP_H / 2) / SV_STEP_H);
        return Math.max(0, Math.min(n, raw));
    } else {
        const raw = Math.round((svgY - DV_HEAD_Y + DV_STEP_H / 2) / DV_STEP_H);
        return Math.max(0, Math.min(n, raw));
    }
}

function svgXFromClient(svg, clientX) {
    const rect = svg.getBoundingClientRect();
    const viewBox = svg.viewBox.baseVal;
    return (clientX - rect.left) * (viewBox.width / rect.width);
}

function svgYFromClient(svg, clientY) {
    const rect = svg.getBoundingClientRect();
    const viewBox = svg.viewBox.baseVal;
    return (clientY - rect.top) * (viewBox.height / rect.height);
}

function initDiagramInteraction() {
    const svg = document.getElementById('ll-diagram');
    let drag = null;

    svg.addEventListener('mousedown', (e) => {
        if (e.target.closest('[data-action]')) return;
        const g = e.target.closest('.ll-node-group');
        if (!g) return;
        e.preventDefault();
        drag = {
            fromIdx: parseInt(g.dataset.index),
            fromVal: g.dataset.value,
            startClientY: e.clientY,
            moved: false,
        };
        g.style.opacity = '0.6';
        g.style.cursor  = 'grabbing';
    });

    svg.addEventListener('mousemove', (e) => {
        if (!drag) return;
        if (Math.abs(e.clientY - drag.startClientY) > 8) drag.moved = true;
    });

    window.addEventListener('mouseup', (e) => {
        if (!drag) return;
        const state = drag;
        drag = null;

        // Restore opacity
        const g = svg.querySelector(`.ll-node-group[data-index="${state.fromIdx}"]`);
        if (g) { g.style.opacity = ''; g.style.cursor = ''; }

        const n = list.length();

        if (state.moved && n > 1) {
            // Drag-to-reorder: compute target gap using vertical position
            const svgY  = svgYFromClient(svg, e.clientY);
            const gap   = computeDropGap(svgY, n);
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
            // Click: fill textarea with get
            const idx = state.fromIdx;
            const ta  = document.getElementById('ll-input');
            ta.value  = `list.get(${idx})`;
            ta.focus();
            ta.select();
        }
    });

    svg.addEventListener('click', (e) => {
        const iconGroup = e.target.closest('[data-action]');
        if (!iconGroup) return;
        e.stopPropagation();
        const action = iconGroup.dataset.action;
        const idx    = parseInt(iconGroup.dataset.index);
        const ta     = document.getElementById('ll-input');
        if (action === 'edit') {
            const current = list.get(idx);
            const raw = window.prompt(`Edit node [${idx}] — new value:`, current);
            if (raw === null) return;
            const parsed = (raw.trim() !== '' && !isNaN(raw)) ? raw.trim() : JSON.stringify(raw);
            ta.value = `list.set(${idx}, ${parsed})`;
            handleRun();
        } else if (action === 'delete') {
            ta.value = `list.deleteAt(${idx})`;
            handleRun();
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
