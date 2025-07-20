#!/usr/bin/env python3
"""
Simple HTTP server for testing the PPM viewer.
Run this from the project root directory.
"""

import http.server
import socketserver
import os
import sys

# Change to the project directory
os.chdir(os.path.dirname(os.path.abspath(__file__)))

# Port to serve on
PORT = 8000

class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add CORS headers for WASM
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        super().end_headers()

    def guess_type(self, path):
        mimetype = super().guess_type(path)
        # Ensure WASM files have correct MIME type
        if path.endswith('.wasm'):
            return 'application/wasm'
        return mimetype

if __name__ == "__main__":
    with socketserver.TCPServer(("", PORT), MyHTTPRequestHandler) as httpd:
        print(f"Serving at http://localhost:{PORT}")
        print(f"Open http://localhost:{PORT}/static/ in your browser")
        print("Press Ctrl+C to stop the server")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nServer stopped.")
