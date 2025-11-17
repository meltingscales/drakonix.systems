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

- CPU
- RAM
- GPU
  - AMD GPU
  - NVIDIA GPU
  - VRAM
- TPU

### Software

- ollama
- llama.cpp
- openwebui
- comfyui
- stable diffusion
- tensorflow
- LM Studio
- gguf
- huggingface

## AMD VS Nvidia

**AMD drivers for Linux are dog shit**. AMD should just open source ROCm. TL;DR use NVIDIA for ease of use.

I personally run an AMD GPU, but it took a week+ to get it working. I had to compile `llama.cpp` with a bunch of custom flags hidden in some GitHub discussion.

If you want an easy experience, use NVidia GPUs. You don't need a 24GB+ VRAM card. You can likely get good imagegen working with lower VRAM cards.

## Actual Content

This is where the guide actually starts ![](https://media3.giphy.com/media/v1.Y2lkPTc5MGI3NjExMjlrYW1jbXZ4czVhemR6bTNrcnR4MThuYzBzbXFubTdubjQzaDdpeSZlcD12MV9naWZzX3NlYXJjaCZjdD1n/Dps6uX4XPOKeA/200.webp)

### PATH_A: GCP_VM_WITH_RENTED_GPU

Cost: ??? $ / mo

### PATH B: RUNPOD.IO
Cost: ??? $ / mo

### PATH_C: DIY_CLOUD_HOSTING
Cost: ??? $ / mo

### PATH_D: GAMING_PC
Cost: ??? $ / mo

### PATH_E: CPU_ONLY_HIGH_RAM
Cost: ??? $ / mo

### PATH_F: CPU_ONLY_HIGH_RAM_SWAP_TO_DISK___EVIL_CHOICE_>;3
Cost: ??? $ / mo
