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
- <https://vesktop.dev/>
- <https://switch.hacks.guide/>
  - <https://github.com/nh-server/switch-guide>
  - <https://mgba.io/downloads.html>
  - <https://github.com/Cpasjuste/pemu/releases> (Needs a custom XML file, annoying.)
  - <https://github.com/retroarch/retroarch> (Needs older firmware, warning.)
- [imgburn, cd burning](https://www.imgburn.com)

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

- [zed](https://zed.dev/) - Fast and simple text editor. Rust backend.
- nano - command line editor

## CICD

- jenkins
  - AVOID. legacy and lots of design bugs
  - pipeline is useful but...its 2025. try to use github cicd or similar. AVOID jenkins.
- get good at bash
- understand yaml
- github runners
  - (literally just docker containers w/ some yaml command runner. its all the same pattern.)
  - can also do self hosted
- <https://www.openstatus.dev/> (uptime/health, can self host)

## OS

i highly recommend never using windows, for [many](https://en.wikipedia.org/wiki/The_Shadow_Brokers), [many](https://en.wikipedia.org/wiki/Edward_Snowden) [reasons](https://en.wikipedia.org/wiki/Server_Message_Block)

### Windows utilities

- [crystaldiskinfo](https://crystalmark.info/en/software/crystaldiskinfo/) - disk health monitor
- [windirstat](https://windirstat.net/) - disk usage visualizer
- [minitool partition wizard](https://www.minitool.com/partition-manager/) - partition manager
- [7-zip](https://www.7-zip.org/) - archive tool
- `mmsys.cpl` - sound settings (run dialog)
- `appwiz.cpl` - add/remove programs (run dialog)

- ubuntu - server OS
- nixos - server/desktop OS
- cachyOS - easy arch based OS, fast. bleeding edge so stuff breaks. avoid if u value ur time or are new to linux.
- TrueNAS CORE - freebsd based NAS OS that uses ZFS, ZFS is superior for use as a NAS for many reasons
  - AVOID hardware based RAID. if ur card dies and u cant replace it, kiss your data goodbye. software based raid does not suffer from this issue.

## Hardware

I run various cheap laptops and a few HEPC (High-End Personal Computer) setups.
Only Linux, except for a Windows computer I use for piano playing/DAW/Frooty Loops.

- https://pcpartpicker.com/user/henryfbp/saved/

## AI

My current AI stack is here. I used to use ollama but it has issues with ROCm drivers on AMD GPUs, so I switched to `llama.cpp`!

<https://github.com/meltingscales/cachyos-whitedragon-ai-lab>

Current working stack:
- [lm studio](https://lmstudio.ai/): easy GUI for running local LLMs
- llama.cpp: text generation backend (works with ROCm)
- koboldcpp: easier to setup than llama.cpp.
- openwebui: frontend for LLM interaction (TBD)
- comfyui: image/video/audio generation (TBD)
- stablediffusion: image generation (TBD)
- [goose: Claude CLI clone, seems very formidable.](//github.com/block/goose)

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
- <https://pomofocus.io/> - pomodoro timer
- [obsidian](https://obsidian.md/) - markdown note-taking / personal knowledge base

## Hacking links

### Dark web / anonymity

- [tor](https://www.torproject.org/) - onion routing anonymity network
- [i2p](https://geti2p.net/) - garlic routing anonymity network
- [ahmia](https://ahmia.fi/) - clearnet Tor search engine
- torch - Tor search engine (`.onion`)
- dread - Reddit-like forum on Tor (`.onion`)
- exploit.in - Russian-language cybercrime/exploit forum
- raidforums - data leak forum (seized by law enforcement)
- leaks - data leak aggregator

- [hacktricks](https://book.hacktricks.xyz/welcome/readme) - pentesting/CTF reference book
- [revshells](https://www.revshells.com/) - reverse shell generator
- [eclypsium](https://eclypsium.com/) - firmware/supply chain security research
- [ddosecrets](https://ddosecrets.com/wiki/Distributed_Denial_of_Secrets) - public interest leak repository
- [hacker news](https://news.ycombinator.com/) - tech/security news aggregator
- [dark reading](https://www.darkreading.com/application-security) - cybersecurity news
- [sans ouch](https://www.sans.org/newsletters/ouch/) - SANS security awareness newsletter
- [databreachtoday](https://www.databreachtoday.com/) - breach/incident news
- [terraform plan visualizer](https://medium.com/vmacwrites/tools-to-visualize-your-terraform-plan-d421c6255f9f) - visualize terraform plans
- [SAFe implementation roadmap](https://www.scaledagileframework.com/implementation-roadmap/) - scaled agile framework
- [simple sabotage field manual](https://www.cia.gov/static/5c875f3ec660e092cf893f60b4a288df/SimpleSabotage.pdf) - declassified CIA doc
- [privacytools](https://www.privacytools.io/) - privacy-focused software recommendations
- [algorithmic trading roadmap](https://blog.openalgo.in/algorithmic-trading-roadmap-2025-from-curious-coder-to-confident-execution-0662572a7838) - algo trading guide
- [lesswrong](https://www.lesswrong.com/) - rationalism/AI safety community

### Hacking tools

#### Used

- kali linux
- parrotos

- dirb
- nmap
- rustscan
- ftp
- some python idk lol
- searchsploit
- msfconsole
- msfvenom

#### To investigate

- [deepnet DN Key Pro](https://deepnet.store/products/dn_key-pro-experimental) - experimental hardware key
- Proxgrind
- [proxmark / icopy-x](https://proxmark.com/) - RFID/NFC research hardware
- Bash Bunny - USB attack platform by Hak5
- [Flipper Zero BadUSB payloads](https://github.com/I-Am-Jakoby/Flipper-Zero-BadUSB/tree/main/Payloads)
