# PPM Viewer

Web-based PPM (P3 format) image viewer using Rust and WebAssembly. Client-side processing for fast, privacy-preserving image rendering.

**[Live Demo](https://marufhossain14.github.io/ppm-viewer/)**

## Why WebAssembly?

Traditional JavaScript struggles with image parsing performance. WebAssembly provides near-native speed for compute-intensive operations while keeping all processing client-side.

## Tech Stack

- **Rust** - Systems language that compiles the parsing logic into WebAssembly
- **WebAssembly (WASM)** - Binary format that runs at near-native speed in browsers
- **wasm-bindgen** - Bridge between Rust code and JavaScript
- **HTML5 Canvas** - Web API for drawing the image pixels on screen

## Features

- Parse and render PPM P3 format images
- Comment filtering in headers
- Dimension and pixel validation
- Client-side processing (no uploads)


## Quick Start

### Local Development

1. Build the WASM package:
   ```bash
   wasm-pack build --target web
   ```
2. Start a local server:
   ```bash
   python serve.py
   ```
3. Open `http://localhost:8000/static/` and upload a PPM file

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://drager.github.io/wasm-pack/)
- Python 3 (for local server)
