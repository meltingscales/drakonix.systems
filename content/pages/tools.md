# Tools

A list of tools I use.

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

- ollama: AI model serving
- openwebui: frontend for ollama
- comfyui: image/video/audio gen

### Corpo AI for coding

- claude: You can use this, but you can also self-host local AI and use CLI tools like:
  - `aider`: OSS claude cli clone
  - etc...TODO: Add more coding agents here from testing.

The basic pattern to locally host is just to run `ollama` on a powerful PC, set up a VPN with `tailscale`, and then
just connect to the ollama endpoint with your less powerful computer via a coding agent like `aider` or a frontend like `openwebui`.

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
