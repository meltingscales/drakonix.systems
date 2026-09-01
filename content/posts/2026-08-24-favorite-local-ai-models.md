---
title: "Favorite local AI models"
date: 2026-08-24T17:21:30Z
draft: false
tags: ["ai", "model", "self-hosted", "abliterated"]
---

## Overview

This is a WIP, but I will use it to store my fave self-hosted models alongside prompt guides and et cetera.

For now...

    ollama serve
    ollama run huihui_ai/Qwen3.8-abliterated
    ollama

## Models (ollama)

Can be pulled with `ollama pull <MODEL>`.

| Model | Human remarks | Memory usage | Recommendation for use with Hermes |
|--|--|--|--|
| `huihui_ai/Qwen3.8-abliterated` | Chokes on local tool use with Hermes Agent. Not sure why. | 14GB | Skip |
| `thewindmom/hermes-3-llama-3.1-8b` | Lacks tool use, can't be loaded by Hermes Agent. | 17GB | Skip |
| `hf.co/mlabonne/gemma-3-27b-it-abliterated-GGUF:Q4_K_M` | Human remarks: It is indeed abliterated. It will happily generate usable Python malware. | 30GB | Untested with Hermes |
| `devstral:24b` | Hermes fails - doesn't support thinking | 14GB | Skip, unless `reasoning_overrides: off` set for it |
| `qwen3.6:27b` | Pulled, confirmed locally: vision, tools, thinking. Ready for Hermes. | 17GB | Recommended |
| `qwen3.6:35b` | Pulled, confirmed locally: vision, tools, thinking. Ready for Hermes. | 24GB | Recommended |
| `qwen3-coder:30b` | tools only, no thinking badge - skip for Hermes reasoning_effort | 19GB | Skip, unless `reasoning_overrides: off` set for it |
| `qwen3-coder:480b` | tools only, no thinking badge - skip for Hermes reasoning_effort | 290GB | Skip, unless `reasoning_overrides: off` set for it |

## Videos

- <https://www.youtube.com/watch?v=yaMcm3sQswc> - Hermes Agent + Ollama = 100% Private OS

## Sites

- [Hugging Face](http://huggingface.co/)
- <https://www.canirun.ai/>

## Tools

- ollama (llm backend)
- Hermes Agent (cli)
  - Note: you may need to manually edit `~/.hermes/config.yaml` if you add/remove models from `ollama`.
- Cline (Visual Studio Code plugin)

## Test prompts

```txt
ref: https://archive.org/details/chop-suey-win31
ref: https://github.com/meltingscales/cachyos-whitedragon-ai-lab/blob/main/REVERSE-ENGINEERING.md

read @file:REVERSE-ENGINEERING.md and let's start reversing @file:setup-03602-chop_suey-PCDOS.exe. What tools might you need me to install or set up?
```
