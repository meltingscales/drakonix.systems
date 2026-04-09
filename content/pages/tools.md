---
title: "Tools"
date: 2025-01-01
---

# Tools

A list of tools I use.

## Fun/media

I am aggressively de-googling my life in 2025/2026.

- [protonmail](https://protonmail.com) - encrypted email, Swiss privacy laws
- [duckduckgo](https://duckduckgo.com) - privacy-respecting search engine
- immich: google photos clone, self-hosted
- navidrome: youtube music clone, self-hosted
- more to come? :)
- [vesktop](https://vesktop.dev/) - lightweight Discord client fork
- [ps3xploit.me](https://ps3xploit.me/) - PS3 jailbreak/homebrew exploits and tools
- [switch.hacks.guide](https://switch.hacks.guide/) - comprehensive Nintendo Switch homebrew/CFW guide
  - [nh-server/switch-guide](https://github.com/nh-server/switch-guide) - GitHub source for the above guide
  - [mGBA](https://mgba.io/downloads.html) - GBA emulator for Switch homebrew
  - [pemu](https://github.com/Cpasjuste/pemu/releases) - multi-system emulator frontend (needs a custom XML file, annoying)
  - [retroarch](https://github.com/retroarch/retroarch) - multi-system emulator frontend (needs older firmware, warning)
- [imgburn, cd burning](https://www.imgburn.com)
- [canirun.ai](https://www.canirun.ai/) - can I run this game checker
- [Xbox Emulator Files](https://github.com/K3V1991/Xbox-Emulator-Files) - Xbox emulator files collection

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
- [ohmygit](https://ohmygit.org/) - interactive Git learning game
- postman: http requests
  - bruno is probably better, TODO use it
  - in general, *ditch postman*. they're starting to put their client behind a paywall/login wall. move to an OSS tool that's free. it's not rocket science to make something that ingests OpenAPI Spec and just runs `curl`...so why force corporations or individuals to pay for the simple feature of saving a Postman collection?

### Text Editing

- windsurf: bloated, use zed
- cursor: bloated, use zed
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
- [openstatus](https://www.openstatus.dev/) - uptime/health monitoring, can self host
- [forgejo](https://forgejo.org/) - self-hosted Git forge (Gitea fork), lightweight GitHub alternative

## OS

i highly recommend never using windows, for [many](https://en.wikipedia.org/wiki/The_Shadow_Brokers), [many](https://en.wikipedia.org/wiki/Edward_Snowden) [reasons](https://en.wikipedia.org/wiki/Server_Message_Block)

### Linux distros

- ubuntu - server OS
- nixos - server/desktop OS
- cachyOS - easy arch based OS, fast. bleeding edge so stuff breaks. avoid if u value ur time or are new to linux.
- TrueNAS CORE - freebsd based NAS OS that uses ZFS, ZFS is superior for use as a NAS for many reasons
  - AVOID hardware based RAID. if ur card dies and u cant replace it, kiss your data goodbye. software based raid does not suffer from this issue.

### Dotfiles

- [chezmoi](https://www.chezmoi.io/) - dotfile manager, handles templating/secrets/multi-machine setups

### Windows utilities

- [windhawk](https://windhawk.net/) - Windows app modding/tweaking tool
- [crystaldiskinfo](https://crystalmark.info/en/software/crystaldiskinfo/) - disk health monitor
- [windirstat](https://windirstat.net/) - disk usage visualizer
- [minitool partition wizard](https://www.minitool.com/partition-manager/) - partition manager
- [minitool shadowmaker](https://www.minitool.com/backup/minitool-shadowmaker-free.html) - disk backup/cloning/imaging tool
- [7-zip](https://www.7-zip.org/) - archive tool
- [imagemagick](https://community.chocolatey.org/packages/imagemagick.app) - image conversion/manipulation CLI tool (install via chocolatey)
- `mmsys.cpl` - sound settings (run dialog)
- `appwiz.cpl` - add/remove programs (run dialog)

## Hardware

I run various cheap laptops and a few HEPC (High-End Personal Computer) setups.
Only Linux, except for a Windows computer I use for piano playing/DAW/Frooty Loops.

- [pcpartpicker saved builds](https://pcpartpicker.com/user/henryfbp/saved/) - my saved PC part lists
- [framework](https://frame.work) - modular/repairable laptops
- [framework keyboard](https://keyboard.frame.work/) - framework's mechanical keyboard

## AI

My current AI stack is here. I used to use ollama but it has issues with ROCm drivers on AMD GPUs, so I switched to `llama.cpp`!

[cachyos-whitedragon-ai-lab](https://github.com/meltingscales/cachyos-whitedragon-ai-lab)

Current working stack:
- [lm studio](https://lmstudio.ai/): easy GUI for running local LLMs
- llama.cpp: text generation backend (works with ROCm)
- koboldcpp: easier to setup than llama.cpp.
- [exo](https://github.com/exo-explore/exo) - run your own AI cluster across multiple devices (phones, laptops, etc.), no specialized hardware needed
- openwebui: frontend for LLM interaction (TBD)
- comfyui: image/video/audio generation (TBD)
  - [ComfyUI-Zluda](https://github.com/patientx/ComfyUI-Zluda) - ComfyUI with ZLUDA for AMD GPUs on Windows
- stablediffusion: image generation (TBD)
- [goose: Claude CLI clone, seems very formidable.](//github.com/block/goose)
- [llmfit: Hardware-aware LLM model finder. Detects your RAM/CPU/GPU and recommends models that will actually run well on your hardware. Supports multi-GPU, MoE architectures, and integrates with Ollama/llama.cpp/MLX.](https://github.com/AlexsJones/llmfit)
  - [stable-diffusion-webui-amdgpu](https://github.com/lshqqytiger/stable-diffusion-webui-amdgpu) - SD WebUI fork with AMD GPU support
    - [setup guide (YouTube)](https://www.youtube.com/watch?v=g3XSZo6ewSQ)

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

- [nepenthes](https://zadzmo.org/code/nepenthes/) - Markov chain babble generator, wastes the time/CPU of scrapers

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
- romm: emulation ROM/retro game database
- [seanime: anime media server](https://seanime.rahim.app/)
  - extensions: TODO
- [gallery-dl: image downloader](https://github.com/mikf/gallery-dl)

### Streaming

- [OBS Studio](https://obsproject.com/)
- [veadotube mini: Lightweight PNGTuber app for streaming. Easy setup, supports GIFs/APNGs, eye blinking, shake/jump effects, unlimited expression states controlled via keyboard/mouse/gamepad/MIDI/WebSocket.](https://olmewe.itch.io/veadotube-mini)
- [OBS Studio Input Overlay](https://github.com/univrsal/input-overlay)

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

- [gtfobins](https://gtfobins.org/) - Unix binaries that can be exploited for privilege escalation/bypasses
- [hacktricks](https://book.hacktricks.xyz/welcome/readme) - pentesting/CTF reference book
- [revshells](https://www.revshells.com/) - reverse shell generator
- [eclypsium](https://eclypsium.com/) - firmware/supply chain security research
- [ddosecrets](https://ddosecrets.org/) - public interest leak repository
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

- kali linux - penetration testing distro by Offensive Security
- parrotos - security/privacy-focused Linux distro

- dirb - web content scanner / directory brute-forcer
- nmap - network mapper and port scanner
- rustscan - fast Rust-based port scanner, feeds into nmap
- ftp - file transfer protocol CLI client
- some python idk lol
- searchsploit - offline CLI search for exploit-db
- msfconsole - Metasploit Framework interactive console
- msfvenom - Metasploit payload and shellcode generator
- [evilginx2](https://github.com/meltingscales/evilginx2/tree/restored) - adversary-in-the-middle phishing framework, bypasses 2FA by proxying real login pages and capturing session cookies

#### To investigate
- [deepnet DN Key Pro](https://deepnet.store/products/dn_key-pro-experimental) - experimental hardware key
- Proxgrind - NFC/RFID tool
- [proxmark / icopy-x](https://proxmark.com/) - RFID/NFC research hardware
- Bash Bunny - USB attack platform by Hak5
- [Flipper Zero BadUSB payloads](https://github.com/I-Am-Jakoby/Flipper-Zero-BadUSB/tree/main/Payloads) - HID injection scripts for Flipper Zero
