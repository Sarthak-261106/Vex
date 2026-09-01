# VEX implementation record

## Scope

This repository follows the supplied master specification incrementally. The approved implementation scope is Phase 0 plus Phase 1 only; later capabilities must not be represented as working.

## Phase 0 findings

- OS: Windows 11 Home Single Language, 64-bit (build family 10.0.26200)
- GPU: NVIDIA GeForce RTX 3050 Laptop GPU, 6 GB VRAM, driver 581.86
- Runtime tools detected: Node.js 25.7.0 and npm 11.10.1
- Missing prerequisites: Python launcher and Rust/Cargo toolchain
- RAM and disk capacity: unavailable through the restricted system query; do not infer values

## Model recommendation

Use a replaceable local runtime. Prefer Ollama or llama.cpp with a 3B–4B Q4 text model for reliable performance on 6 GB VRAM. Larger 7B Q4 models are optional and should not run alongside vision without testing. Use Whisper base/small and Piper locally. Model downloads require a fresh, explicit approval and are never initiated by this project.

## Phase 1 acceptance target

- Manual launch through `launch_vex.bat` / `run_vex.py`
- Tauri desktop window with no startup registration
- Draggable, resizable, always-on-top presentation
- Placement and visual preference persistence
- Native system tray and a complete Exit action
- No hidden background process after Exit

## Known blockers

Building the native application is blocked locally until Python 3 and Rust stable/MSVC build tools are installed. No installation was attempted because downloads and system changes require user approval.
