---
title: "Sparse vs Wide"
date: "2025-03-06T19:11:07-06:00"
author: "Henry Post"
authorTwitter: ""
tags:
  - "programming"
  - "neural-networks"
description: ""
showFullContent: false
readingTime: false
hideComments: false
---

<script type="module">
import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs';
mermaid.initialize({ startOnLoad: true });
</script>

I recently read an interesting article about a neural network that, instead of artificial neurons, has logic gates.

<https://google-research.github.io/self-organising-systems/difflogic-ca/?hn>

The paper described its neural network as "sparse", and this made me want to ask ChatGPT "Why does this paper describe logic-gate networks as sparse?"

I didn't realize this really important fact:

In "Deep Neural Networks", they're called "Deep" because it's a 2D matrix of nodes "neurons" that are FULLY CONNECTED to every single output in the previous layer.

So, for DNNs, this means that, a 2d matrix ("layer") of 8x8 neurons (64), every single neuron in the next layer receives 64 individual inputs - 1 for each neuron in the previous layer.

For logic gate-based networks, they are NOT fully connected at all - each logic gate can only receive 1 or 2 inputs from the previous layer.

My intuition tells me that this probably makes these networks far easier to optimize.

<pre class="mermaid">
flowchart LR
    subgraph DLGN["Differentiable Logic Gate Network (Sparse)"]
        inputA1("Input A1") --> AND1
        inputB1("Input B1") --> AND1
        inputA2("Input A2") --> OR1
        inputB2("Input B2") --> OR1
        
        AND1["AND Gate"] --> XOR1
        OR1["OR Gate"] --> XOR1
        
        inputC1("Input C1") --> XOR1
        XOR1["XOR Gate"] --> Output1["Output"]
    end

    subgraph DNN["Traditional Neural Network (Dense)"]
        inputX("Input X") --> h1["Neuron 1"]
        inputX --> h2["Neuron 2"]
        inputX --> h3["Neuron 3"]
        inputY("Input Y") --> h1
        inputY --> h2
        inputY --> h3
        
        h1 --> o1["Neuron 4"]
        h2 --> o1
        h3 --> o1
        o1 --> Output2["Output"]
    end

    classDef sparse fill:#aaf,stroke:#000,stroke-width:2px;
    classDef dense fill:#faa,stroke:#000,stroke-width:2px;

    class DLGN sparse;
    class DNN dense;
</pre>
