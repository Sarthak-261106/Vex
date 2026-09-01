# VEX engineering rules

## Architecture

- Keep frontend, native desktop, and Python services modular.
- Services communicate through narrow interfaces; AI never calls OS APIs directly.
- Prefer local inference. Cloud is opt-in, visibly disclosed, and credential-safe.
- Do not implement later phases merely to simulate them.

## Privacy and security

- Never auto-start VEX, create a service, scheduled task, registry startup entry, or auto-restart process.
- Vision is off by default. Blind must block capture at the service boundary.
- Mute blocks voice output but never silently changes microphone input.
- No screen/microphone capture, cloud upload, download, execution, or automation without the relevant permission.
- Every internet download needs a fresh approval. Download approval never permits execution.
- Destructive actions, shell commands, installations, system changes, and external communication require confirmation.
- File access must be scoped. Never expose unrestricted filesystem or arbitrary command execution to the AI.
- Never store secrets in source, frontend bundles, or Git.

## Quality

- Keep the primary launch path obvious: `launch_vex.bat`, then `run_vex.py`.
- Build, run, test, inspect logs, and document each phase before advancing.
- Never claim a test or feature passed unless it actually did.
- Preserve captions when voice output is muted.
