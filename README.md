# PPM Viewer

A simple web app to view PPM (P3) images using Rust and WebAssembly.

## Usage

1. Build the WASM package:
   ```bash
   wasm-pack build --target web
   ```
2. Start a local server in the project folder (e.g. with Python):
   ```bash
   python serve.py
   ```
3. Open `static/index.html` in your browser (via the local server).
4. Upload a PPM (P3) file to view it.

## Requirements

- Rust
- wasm-pack
- Python (for local server)

---

All code is in `src/lib.rs` (Rust) and `static/` (frontend).
