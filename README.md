# PPM Viewer

A simple web app to view PPM (P3) images using Rust and WebAssembly.

## Live Demo

🚀 **[Try the live demo on GitHub Pages](https://[your-username].github.io/ppm-viewer/)**

## Usage

### Local Development

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

### GitHub Pages Deployment

This project is configured for automatic deployment to GitHub Pages:

1. Fork or clone this repository
2. Enable GitHub Pages in repository settings (Source: GitHub Actions)
3. Push changes to the `main` branch
4. GitHub Actions will automatically build and deploy the site

## Requirements

- Rust
- wasm-pack
- Python (for local server only)

---

All code is in `src/lib.rs` (Rust) and `static/` (frontend).
