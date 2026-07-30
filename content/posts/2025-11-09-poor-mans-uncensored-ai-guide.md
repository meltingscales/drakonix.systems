---
title: "Poor Man's Uncensored AI Guide 2025/2026 [WIP v2]"
date: "2025-11-09"
author: ""
authorTwitter: ""
cover: ""
tags:
  - "code"
  - "programming"
  - "llm"
  - "ai"
  - "claude"
  - "censorship"
  - "government"
description: ""
showFullContent: false
readingTime: false
hideComments: false
---

Hello there! I work in tech and have had access to a lifetime of computer tools and teachings. If you've read my bio, you'll know that when I was only 8 years old, my dad taught me how to replace computer parts.

![](/static/henrys-first-pc-dafuq-is-this.png)

I've been really, really busy since then. I recently built an AIPC/HEPC (Artificial Intelligence Personal Computer/High End Personal Computer) for a close friend of mine.

<https://pcpartpicker.com/user/henryfbp/saved/#view=kGf9t6>

I had a conversation at a dinner place recently with a person who had grown up without access to the same opportunities as I did, and I showed them my uncensored AI. Immediately, they wanted access. Permanent access. I let them play with my phone for a bit, and realized that an easy-to-follow guide to self-host uncensored AI would be useful. This is my attempt to write that.

A lot of this content will be stored in this guide, which I link to from `~/tools`: <https://github.com/meltingscales/cachyos-whitedragon-ai-lab>.

## Intro

This guide is for people who are interested in self-hosted AI, who don't have $1,400 to $4,000+ to just drop on some shiny tech gadget.

I will try my best to put my real-world experience in here and if you find a bug in my setup,
[please click here to create an issue](https://github.com/meltingscales/meltingscales.github.io-rust/issues) and provide as much context as possible - full error logs, original commands, `msinfo32`/`fastfetch`/`neofetch` output, etc.

## Outcome

A fully self-hosted AI server (images/text gen) without any censorship or "safety" imposed by your corporate overlords :)

## Brief Overview

A lot of people think that AI needs high specs. Not true! Google Coral is an "edge compute" device that loads ~22MB (megabytes - yes! tiny!) models and is used for object recognition in images. I use it in my security cam setup (see my article on Ring). There are so many variations of image/text/audio gen/detection models that all need various different sizes of hardware to run on. 

I truly think that given enough time, you can run an AI model on almost any piece of semi-modern hardware, because people just keep making them and re-quantizing them and re-finetuning them.

## Technology Used

Here's a list of tools and concepts with brief explanations:

### Hardware

- CPU: Part that does normal calculations. Fast. Single-operation.
- RAM: Temporary storage for running programs.
- GPU: Part that does multiplication in parallel. Not fast but massively parallel compared to a CPU.
  - AMD GPU
  - NVIDIA GPU
  - VRAM: Temporary storage for running programs on a GPU. Expensive.
- TPU: Part that does special addition in parallel, meant to run large language models. Not that fast but again, massively parallel.

### Software

- ollama: backend for running text generation models.
- llama.cpp: another backend for running text generation models.
- openwebui: frontend (web UI) for text gen.
- comfyui: frontend for image/video/audio/text gen. very nice.
- stable diffusion: family of open image generation models (SD1.5, SDXL, SD3, Flux, etc.) — what comfyui usually runs under the hood.
- tensorflow: data science library
- LM Studio: easy, beginner friendly version of openwebui.
- gguf: file format for LLM weights (the actual data that makes up an LLM) as well as metadata.
- huggingface: place people store model files

## AMD VS Nvidia

**AMD drivers for Linux are dog shit**. AMD should just open source ROCm. TL;DR use NVIDIA for ease of use.

I personally run an AMD GPU, but it took a week+ to get it working. I had to compile `llama.cpp` with a bunch of custom flags hidden in some GitHub discussion.

If you want an easy experience, use NVidia GPUs. You don't need a 24GB+ VRAM card; certainly not for text gen, and you can likely get good image gen working with lower VRAM cards.

## A note on framework desktops

"Unified memory" means the CPU and GPU share one physical pool of RAM instead of the usual split — a fixed slug of system RAM plus separate, smaller VRAM soldered to the GPU. For LLMs this matters because VRAM capacity is normally the hard ceiling on model size, and unified memory lets you point almost the whole pool at the GPU side instead of being stuck with whatever a discrete card shipped with.

The [Framework Desktop](https://frame.work/desktop) is the consumer example: an AMD Ryzen AI Max+ 395 mini PC with up to 128GB (192GB on newer configs) of LPDDR5X unified memory. On Linux, roughly 96GB of a 128GB unit is addressable as GPU memory — more usable model space than a discrete RTX 5090 (32GB VRAM) at a fraction of the power draw, in a form factor small enough to sit on a desk.

It's not free lunch, though. Memory bandwidth is the tradeoff: the 192GB config tops out around 273GB/s, well under a 3090's 936GB/s or a 4090's ~1TB/s, so token generation is slower than a discrete GPU with the same VRAM would give you — you're trading speed for capacity. Price has also crept up since launch: the 128GB config started at $1,999, and has been seen as high as $3,449 in 2026 depending on availability. Compared to PATH_D (build your own 3090 rig), a Framework Desktop is easier (no case/PSU/compatibility puzzle, just plug it in) and lets you run bigger models than a single 24GB card, but you'll pay more per token/sec and per dollar of raw VRAM than the used-GPU route. Good middle ground if you want big-model capacity without building a PC and can tolerate slower generation.

## Actual Content

This is where the guide actually starts ![](https://media3.giphy.com/media/v1.Y2lkPTc5MGI3NjExMjlrYW1jbXZ4czVhemR6bTNrcnR4MThuYzBzbXFubTdubjQzaDdpeSZlcD12MV9naWZzX3NlYXJjaCZjdD1n/Dps6uX4XPOKeA/200.webp)

### PATH_A: GCP_VM_WITH_RENTED_GPU

#### Specs

| Feature | GCP VM with Rented GPU (PATH_A) |
|-------------------|-----------------------|
| Running Cost      | $150-250/mo           |
| Idle Cost         | $60-100/mo            |
| CPU               | n1-standard-4         |
| GPU               | NVIDIA T4             |
| RAM               | 16GB                  |

#### Steps

1. start a GCP VM with these specs:  
   region: us-central1  
   machine: n1-standard-4  
   GPU: NVIDIA T4  
   OS: Ubuntu LTS  
   Disk: 100GB  
1. install ubuntu desktop
1. install LM Studio
   1. download the .appimage file
   1. open terminal
   1. run `cd` to change your directory to where you downloaded it
   1. run `chmod +x THE_FILE_NAME.appimage` to make it executable
   1. run `./THE_FILE_NAME.appimage` to run the file
1. run `sudo apt update; sudo apt install fastfetch neofetch -y`
1. run `fastfetch` to get system specs.
1. pick <https://huggingface.co/mlabonne/gemma-3-27b-it-abliterated> as a model and download it 
1. tweak context window (you can just tell Gemma 3 what your GPU and RAM and CPU are and ask it to guess, it's smart enough to be correct)
1. load the model in LM Studio and start chatting — no filters, no refusals
1. profit!

### PATH_B: RUNPOD.IO

#### Specs

| Feature      | RunPod (PATH_B)                                          |
|--------------|-----------------------------------------------------------|
| Running Cost | $0.34-0.69/hr (RTX 4090, Community/Secure Cloud)          |
| Idle Cost    | $0/hr while stopped — only storage is billed (~$0.10/GB/mo) |
| GPU          | RTX 4090 (24GB) or RTX A6000 (48GB)                       |
| Billing      | per-second                                                |

RunPod is the "managed" version of GPU rental: pick a GPU, pick a template, get a pod with SSH/Jupyter access in under a minute. Two tiers — Community Cloud (cheaper, less guaranteed uptime) and Secure Cloud (verified datacenters, roughly 2x the price).

#### Steps

1. sign up at <https://runpod.io> and add billing
1. click "Deploy" → GPU Pod
1. pick a GPU (RTX 4090 for a good VRAM/price ratio, or A6000 if you need 48GB for bigger models)
1. pick a template — "RunPod PyTorch" or a community Ollama/text-gen-webui template both work
1. once the pod is running, connect via the provided SSH command or the web terminal
1. install ollama or llama.cpp same as you would locally (see Software above)
1. download your abliterated model of choice from huggingface, same as PATH_A
1. **stop (don't just close the tab) the pod when you're done** — you're billed per-second while it's running, whether or not you're actually using it

### PATH_C: DIY_CLOUD_HOSTING

#### Specs

| Feature      | Vast.ai (PATH_C)                                                        |
|--------------|---------------------------------------------------------------------------|
| Running Cost | $0.35-0.55/hr on-demand (RTX 4090) — spot pricing as low as $0.03/hr for older cards |
| Idle Cost    | $0/hr while stopped, storage billed separately                          |
| GPU          | whatever a random host is renting out — filter by VRAM and reliability score |
| Billing      | per-second, peer-to-peer marketplace                                    |

This is PATH_B but cheaper and rawer: Vast.ai is a marketplace of other people's GPUs rather than a managed datacenter, so pricing runs 30-50% below RunPod but reliability and network speed vary per-host. No polished "serverless" option here — you're renting a box, not a service.

#### Steps

1. sign up at <https://vast.ai> and add billing
1. search offers, filter by GPU VRAM (24GB+ recommended) and sort by reliability — skip anything below ~95%
1. rent an instance with a PyTorch/CUDA template
1. connect via the provided SSH command
1. install ollama or llama.cpp, download your model, same as PATH_A/PATH_B
1. **destroy the instance when done** — same per-second billing gotcha as RunPod

### PATH_D: GAMING_PC

#### Specs

| Feature      | Gaming PC (PATH_D)                                                |
|--------------|---------------------------------------------------------------------|
| Upfront Cost | $1,000-1,200 one-time (used RTX 3090 + budget rest of the build)  |
| Ongoing Cost | electricity only — a 3090 pulls ~350W under load                  |
| GPU          | RTX 3090, 24GB VRAM (same VRAM as a 4090, for a third of the used price) |
| RAM          | 32GB+ recommended                                                  |

This is the "own the hardware" path. A used RTX 3090 is still the best VRAM-per-dollar card for local AI in 2026 — 24GB is enough for most 13B-30B models at reasonable quantization, and 3090s are common on the used market from mining/gamer upgrade cycles. Budget $800-925 for the card alone, leaving $200-400 for a used/budget CPU, motherboard, 32GB RAM, PSU (1000W+, a 3090 is power-hungry), and storage.

#### Steps

1. build a parts list — see mine for inspiration: <https://pcpartpicker.com/user/henryfbp/saved/#view=kGf9t6>
1. buy a used RTX 3090 (eBay, local marketplace, r/hardwareswap) — check for reference cooling issues and coil whine before finalizing
1. assemble the PC
1. install Ubuntu/CachyOS/your Linux distro of choice
1. install NVIDIA drivers (see AMD vs Nvidia above — NVIDIA is the easy path)
1. install ollama or LM Studio
1. download your model, same as PATH_A

### PATH_E: CPU_ONLY_HIGH_RAM

#### Specs

| Feature      | CPU-Only High-RAM (PATH_E)                                              |
|--------------|-----------------------------------------------------------------------------|
| Upfront Cost | $400-800 one-time (64-128GB DDR5, assuming a decent CPU/motherboard already) |
| Ongoing Cost | electricity only, and a lot less of it than a GPU rig                    |
| GPU          | none                                                                       |
| RAM          | 64GB minimum, 128GB+ to comfortably run 30B+ models                       |

No GPU means no VRAM ceiling — the ceiling becomes how much RAM you can afford, and RAM is a lot cheaper than VRAM per gigabyte. llama.cpp runs quantized GGUF models entirely on CPU. Expect roughly 10-18 tokens/sec for 7B-13B models on a modern 16+ core CPU with DDR5, scaling down as model size goes up. This won't touch image gen (way too slow on CPU) but is a legitimate, cheap way to run a decent text model.

#### Steps

1. check your motherboard's max supported RAM and channel count (more channels = more bandwidth = faster inference)
1. buy DDR5 RAM to fill it out — 64GB minimum, 128GB+ if your board/wallet allows
1. install llama.cpp (compiles CPU-only out of the box, no special flags needed)
1. download a GGUF model sized to fit comfortably in RAM (leave headroom for context + OS)
1. run with `--threads` set to your physical core count (not thread count — hyperthreading doesn't help much here)

### PATH_F: CPU_ONLY_HIGH_RAM_SWAP_TO_DISK___EVIL_CHOICE_>;3
Cost: $0 extra — uses your existing NVMe drive

This is PATH_E but evil: instead of buying enough RAM to hold the model, you just... don't. llama.cpp uses `mmap()` to load model weights, which means if the file doesn't fit in RAM, the OS quietly serves the missing pages straight off your NVMe SSD instead of crashing. It's not swap in the traditional sense (nothing gets written back to disk, it's read-only), so it won't wear out your drive — but random access to a giant GGUF file over NVMe is a lot slower than RAM.

Expect somewhere between 0.05 and 2 tokens/sec depending on how far past your RAM you're pushing it and how fast your NVMe is — this is "start it before bed, read the answer in the morning" territory for the biggest models, not interactive chat. But it means a model that's flatly impossible to run any other way on your hardware becomes merely very slow instead of impossible (see PATH_H: colibri below for a purpose-built version of this idea).

#### Steps

1. do everything from PATH_E, except pick a GGUF model bigger than your RAM
1. make sure the model lives on your fastest NVMe drive, not spinning rust
1. run llama.cpp as normal — mmap is on by default, no flag needed
1. go do something else while it generates

### PATH_G: CPU-GPU PARTIAL OFFLOADING (MoE MODELS)
Cost: whatever GPU you already have from PATH_D — this is a technique, not a separate box

Modern MoE (Mixture-of-Experts) models like Qwen3-30B-A3B only activate a small fraction of their total parameters per token (3.3B active out of 30.5B total, for Qwen3-30B-A3B). llama.cpp can exploit this: keep the small, always-used dense/attention layers on your GPU, and offload the big-but-rarely-fully-used expert layers to CPU RAM. You get most of the speed of a full GPU load while needing only a fraction of the VRAM a dense model of the same size would demand.

The key flag is `-ot ".ffn_.*_exps.=CPU"`, which forces all MoE expert tensors onto CPU regardless of `--n-gpu-layers`.

#### Steps

1. pick an MoE model — Qwen3-30B-A3B (Coder/Instruct/Thinking variants) is the well-tested option
1. run llama.cpp with something like:
   ```
   llama-server -m qwen3-30b-a3b.gguf \
     --threads -1 \
     --ctx-size 16384 \
     --n-gpu-layers 99 \
     -ot ".ffn_.*_exps.=CPU"
   ```
1. tune `--ctx-size` down if you run out of RAM, or up if you have headroom
1. plan for 45GB+ combined RAM+VRAM for a 4-bit quant, or 30GB+ if you drop to a 2-bit XL quant

Source: [r/LocalLLaMA guide on Qwen-30B CPU-GPU partial offloading](https://www.reddit.com/r/LocalLLaMA/comments/1mfs9qn/guide_running_qwen30b_coderinstructthinking_with/), and a good writeup of the general technique: [GPU-poor: offloading Qwen3-235B-A22B MoE with llama.cpp](https://medium.com/@david.sanftenberg/gpu-poor-how-to-configure-offloading-for-the-qwen-3-235b-a22b-moe-model-using-llama-cpp-13dc15287bed).

### PATH_H: colibri
Cost: $0 extra — a purpose-built version of PATH_F for MoE models

[colibri](https://github.com/JustVugg/colibri) takes the PATH_F idea (stream what doesn't fit in RAM off NVMe) and specializes it for MoE models, which are a much better fit for it than dense models: since colibri only pulls the ~40B active-per-token experts out of models like GLM-5.2 (744B total) or Kimi K3 (2.8T total), it doesn't need to touch most of the model most of the time. It keeps the small dense/attention layers resident in RAM (~10GB at int4), streams the individual expert tensors (~19MB each, 19,456 of them) from disk on demand, and uses predictive prefetching (~72% hit rate) plus a learned cache to keep frequently-used experts warm.

Performance scales with what you throw at it: ~0.05-0.1 tok/s on a bare 25GB RAM box, ~1.8 tok/s on a 128GB CPU-only desktop, up to 5.8-6.8 tok/s on a 6x RTX 5090 rig — all running the exact same 744B-parameter model.

#### Steps

1. download a release: `tar xzf colibri-v1.1.0-linux-x86_64.tar.gz`
1. download a supported model from huggingface (GLM-5.2, Inkling, Kimi K3, or the much smaller OLMoE for testing) — expect ~372GB on disk for GLM-5.2
1. point colibri at it and run: `COLI_MODEL=/nvme/glm52_i4 ./coli chat`
1. let it auto-detect your hardware and pick a placement strategy — no manual tuning required
