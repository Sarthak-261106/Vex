# HOW TO RUN VEX

Development: double-click [launch_vex.bat](launch_vex.bat), or run:

```powershell
python run_vex.py
```

VEX is a local-first Windows desktop companion: the visible, privacy-respecting spirit of your laptop. This repository currently implements **Phase 0 and the Phase 1 desktop shell**. Voice, local AI, memory, screen vision, and computer control are deliberately not implemented yet.

## Prerequisites

- Windows 10/11
- Python 3.11+ on PATH
- Node.js 20+ and npm
- Rust stable with the MSVC build tools (required by Tauri)

Then install the frontend and Rust dependencies explicitly:

```powershell
cd frontend
npm install
cd ..\src-tauri
cargo build
```

No AI model is downloaded by VEX. Model installation will remain an explicit, separate action.

## Current status

The Phase 1 shell provides a draggable/resizable, always-on-top VEX window, persistent window placement, a minimal click menu, caption display, a system-tray menu, and a complete Exit action. It creates no Windows startup entry, service, scheduled task, or restart process.

## Architecture

- `frontend/`: React + TypeScript presentation layer
- `src-tauri/`: native Windows window, tray, and safe commands
- `backend/`: replaceable local-first service contracts (not started in Phase 1)
- `assets/`: visual reference and future character assets

## Hardware and local-AI recommendation

Detected: Windows 11, NVIDIA GeForce RTX 3050 Laptop GPU (6 GB VRAM, driver 581.86). RAM and storage could not be read from this restricted development session; VEX will provide a fuller in-app detector later.

For this GPU, start with Ollama or llama.cpp and a 3B–4B instruct model in 4-bit quantization (for example Qwen2.5 3B or Llama 3.2 3B). A 7B Q4 model may run but leaves little VRAM headroom and is less suitable with vision. Use a small, on-demand vision model; use Whisper `base`/`small` for local speech recognition and Piper for local TTS. None should be installed until you approve the download.

## Privacy and security

Vision is off by default. Blind mode is designed as a service-level capture block, not merely a visual state. The current shell contains no screen capture, microphone access, AI inference, network access, downloads, shell execution, or computer-control capability. Future privileged capabilities must flow through the permission layer.

## Roadmap

1. Desktop presence (current)
2. Expressions, captions, Mute/Blind UI
3. Local text brain
4. Local voice
5. SQLite memory
6. Opt-in vision
7. Permissioned computer tools
8. Download security and full-screen environment

See [PROJECT_SPEC.md](PROJECT_SPEC.md) for the scoped implementation record and [AGENTS.md](AGENTS.md) for non-negotiable engineering rules.
