```bash
cargo check
```
rebuild the WASM package to ensure everything is up to date:

```bash
wasm-pack build --target web --out-dir pkg
```
test the server:
```bash
python serve.py
```
How to Test:
The development server is already running at http://localhost:8000
Open http://localhost:8000/static/ in your browser
Use the file input to upload ppm file, or any P3 format PPM file
The image should display on the canvas