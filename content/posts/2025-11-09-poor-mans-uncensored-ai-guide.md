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
- stable diffusion: 
- tensorflow: data science library
- LM Studio: easy, beginner friendly version of openwebui.
- gguf: file format for LLM weights (the actual data that makes up an LLM) as well as metadata.
- huggingface: place people store model files

## AMD VS Nvidia

**AMD drivers for Linux are dog shit**. AMD should just open source ROCm. TL;DR use NVIDIA for ease of use.

I personally run an AMD GPU, but it took a week+ to get it working. I had to compile `llama.cpp` with a bunch of custom flags hidden in some GitHub discussion.

If you want an easy experience, use NVidia GPUs. You don't need a 24GB+ VRAM card; certainly not for text gen, and you can likely get good image gen working with lower VRAM cards.

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
1. ???
1. profit!

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
