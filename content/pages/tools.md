---
title: "Tools"
date: 2025-01-01
---

# Tools

A list of tools I use.

## Fun/media

I am aggressively de-googling my life in 2025/2026.

- immich: google photos clone, self-hosted
- navidrome: youtube music clone, self-hosted
- more to come? :)

## Programming

- python: general purpose language
  - conda/poetry/uv/pipenv/pip - package managers. prefer uv for speed/simplicity.
  - pil/pillow: image processing library
  - numpy: numerical computing library
  - pandas: xlsx/csv manipulation library
  - matplotlib: data visualization library
- rust: faster than python.
- bash: scripting language
- just: very good build tool
- postman: http requests
  - bruno is probably better, TODO use it
  - in general, *ditch postman*. they're starting to put their client behind a paywall/login wall. move to an OSS tool that's free. it's not rocket science to make something that ingests OpenAPI Spec and just runs `curl`...so why force corporations or individuals to pay for the simple feature of saving a Postman collection?

### Text Editing

- windsurf: bloated, use zeditor
- cursor: bloated, use zeditor
- vscode - bloated, only use if you absolutely need to. It's slow and used to be my favorite, but...

- zeditor - Fast and simple text editor. Rust backend.
- nano - command line editor

## OS

i highly recommend never using windows, for many, many reasons

- ubuntu - server OS
- nixos - server/desktop OS
- cachyOS - easy arch based OS, fast
- TrueNAS CORE - freebsd based NAS OS that uses ZFS, ZFS is superior for use as a NAS for many reasons

## Hardware

I run various cheap laptops and a few HEPC (High-End Personal Computer) setups.
Only Linux, except for a Windows computer I use for piano playing.

- https://pcpartpicker.com/user/henryfbp/saved/

## AI

My current AI stack is here. I used to use ollama but it has issues with ROCm drivers on AMD GPUs, so I switched to `llama.cpp`!

<https://github.com/meltingscales/cachyos-whitedragon-ai-lab>

Current working stack:
- llama.cpp: text generation backend (works with ROCm)
- openwebui: frontend for LLM interaction (TBD)
- comfyui: image/video/audio generation (TBD)
- stablediffusion: image generation (TBD)

### Corpo AI for coding

- claude: You can use this, but you can also self-host local AI and use CLI tools like:
  - `aider`: OSS claude cli clone
  - etc...TODO: Add more coding agents here from testing.

The basic pattern to locally host is just to run `llama.cpp` (or `ollama` if you have NVIDIA GPUs) on a powerful PC, set up a VPN with `tailscale`, and then connect to the LLM endpoint with your less powerful computer via a coding agent like `aider` or a frontend like `openwebui`.

### Local AI models

These models were intended for ollama but can be converted for llama.cpp. I plan on running my own benchmarks on each.

- llama3.2:3b
  - TODO: Test.
- yuiseki/devstral-small-2507:24b
  - TODO: Test.
- hf.co/bartowski/Qwen2.5-Coder-14B-Instruct-abliterated-GGUF:Q4_K_S
  - TODO: Test.
- hf.co/mlabonne/gemma-3-27b-it-abliterated-GGUF:Q4_K_M
  - Human remarks: It is indeed abliterated. It will happily generate usable Python malware.
  - TODO: Test.

### Anti AI/Anti Slop

- https://zadzmo.org/code/nepenthes/
  - Markov chain babble generator. Waste the time/CPU of scrapers.

## Infra

- docker: containerization platform
- kubernetes: container orchestration
- grafana: monitoring visualization
- prometheus: monitoring metrics/backend

## Backup

- `dd` - data duplicator/data destroyer

<!-- TODO: Check out protondrive -->

## Media

- yt-dlp: youtube downloader, before youtube killed it with session cookie enforcement.
- transmission: torrent client
- jellyfin: media server
- immich: self hosted google photos clone
- romm: emulation ROM/retro game database
- [seanime: anime media server](https://seanime.rahim.app/)
  - extensions: TODO
- [gallery-dl: image downloader](https://github.com/mikf/gallery-dl)

## Other

- handy.computer - offline text transcription
- sshx.io - share terminal (dangerous)

## Hacking links

- https://book.hacktricks.xyz/welcome/readme
- https://www.revshells.com/
- https://eclypsium.com/
- https://ddosecrets.com/wiki/Distributed_Denial_of_Secrets
- https://news.ycombinator.com/
- https://www.darkreading.com/application-security
- https://www.sans.org/newsletters/ouch/
- https://www.databreachtoday.com/
- https://medium.com/vmacwrites/tools-to-visualize-your-terraform-plan-d421c6255f9f
- https://www.scaledagileframework.com/implementation-roadmap/
- https://www.cia.gov/static/5c875f3ec660e092cf893f60b4a288df/SimpleSabotage.pdf
- https://www.privacytools.io/
- https://blog.openalgo.in/algorithmic-trading-roadmap-2025-from-curious-coder-to-confident-execution-0662572a7838
- https://www.lesswrong.com/
