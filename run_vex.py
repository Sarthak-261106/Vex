"""The single development entry point for VEX. It never registers startup."""
from __future__ import annotations

import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent
FRONTEND = ROOT / "frontend"


def main() -> int:
    npm = shutil.which("npm.cmd") or shutil.which("npm")
    if npm is None:
        print("VEX requires Node.js and npm. Install them, then run this launcher again.")
        return 1
    if not (FRONTEND / "node_modules").exists():
        print("Frontend dependencies are missing. Run: cd frontend && npm install")
        print("This launcher will never download dependencies automatically.")
        return 1
    print("Starting VEX desktop development mode…")
    return subprocess.call([npm, "run", "tauri", "dev"], cwd=FRONTEND)


if __name__ == "__main__":
    raise SystemExit(main())
