# 🔐 Cipher Box

A cross-platform text encryption tool built with **Rust**, **Tauri**, and **WebAssembly (WASM)**. Cipher Box features a Neumorphic (Soft UI) design and seamlessly runs on both desktop and web environments using a single shared codebase.

## ✨ Features

- **Cross-Platform:** Runs natively as a Desktop or Mobile App (Tauri) and dynamically in the Browser (WASM).
- **Cryptography:** Supports industry-standard **AES-256-GCM** and **ChaCha20Poly1305** algorithms.
- **UI:** Crafted with a responsive Neumorphic interface.
- **Feedback:** Custom interactive toast notifications for success and error handling.
- **Optimized Backend:** Core logic is written in Rust for memory safety.

## 🛠️ Tech Stack

- **Core Logic:** Rust (`aes-gcm`, `chacha20poly1305`, `sha2`)
- **Desktop Framework:** Tauri v2
- **Web Build:** WebAssembly (`wasm-bindgen`)
- **Frontend:** Vanilla JavaScript, HTML5, CSS3

## 🚀 How to Run

### 1. Run as Web App (WASM)

Ensure you have compiled the Rust logic to WebAssembly inside the `pkg` directory. Then, spin up a local server in the project root such as http.server module of python:

#### Using Python 3 built-in HTTP server

```bash
python3 -m http.server 8080
```
Open http://localhost:8080 in your browser.

### 2. Run as Desktop App (Tauri)

Ensure you have the Tauri CLI and Rust environment configured. Run the following command in the project root:

```bash
npm run tauri dev
# or
cargo tauri dev
```
## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.
