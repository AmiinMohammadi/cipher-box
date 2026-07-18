<img src="src/assets/banner.png">

---

# 🔐 Cipher Box

A cross-platform text encryption tool built with **Rust**, **Tauri**, and **WebAssembly (WASM)**. Cipher Box features a Neumorphic (Soft UI) design and seamlessly runs on both desktop and web environments using a single shared codebase.

---

## 🚀 Live link

[Launch Cipher Box Live](https://amiinmohammadi.github.io/cipher-box/src)

---

## ✨ Features

- **Cross-Platform:** Runs natively as a Desktop or Mobile App (Tauri) and dynamically in the Browser (WASM).
- **Cryptography:** Supports industry-standard **AES-256-GCM** and **ChaCha20Poly1305** algorithms.
- **UI:** Crafted with a responsive Neumorphic interface.
- **Feedback:** Custom interactive toast notifications for success and error handling.
- **Optimized Backend:** Core logic is written in Rust for memory safety.

---

## 🛠️ Tech Stack

- **Core Logic:** Rust (`aes-gcm`, `chacha20poly1305`, `sha2`)
- **Desktop Framework:** Tauri v2
- **Web Build:** WebAssembly (`wasm-bindgen`)
- **Frontend:** Vanilla JavaScript, HTML5, CSS3

---

## ⚙️ Architecture & Core Logic (Single Shared Codebase)

The defining feature of **Cipher Box** is its architectural efficiency. Instead of rewriting the cryptographic logic for different environments, it utilizes a **single Rust core** that serves both platforms:

1. **Native Desktop Layer (Tauri v2):** The frontend communicates directly with the Rust backend via Tauri's IPC (Inter-Process Communication) system using `invoke` commands. This ensures desktop-level memory safety and near-native performance.
2. **Web Browser Layer (WebAssembly):** The exact same Rust cryptographic logic is compiled into WebAssembly using `wasm-bindgen`. The JavaScript frontend calls these compiled WASM functions directly in the browser, eliminating the need for a backend server.

---

## 🔒 Security & Cryptographic Workflow

To ensure maximum security and efficiency, the core logic follows a strict cryptographic pipeline:

- **Key Derivation / Hashing:** Implemented via `sha2` to ensure high-entropy data integrity checks.
- **Symmetric Encryption Options:**
  - **AES-256-GCM:** A hardware-accelerated, authenticated encryption algorithm providing both confidentiality and data integrity.
  - **ChaCha20Poly1305:** A stream cipher highly optimized for environments without hardware AES acceleration, ensuring fast processing on mobile and older architectures.
- **Zero-Server / Local-First Policy:** All encryption and decryption happen entirely on the client side (locally in the Tauri webview or the user's browser). No plain text or secret keys are ever transmitted over the network.

---

## 💻 How to Run

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
---

This project is licensed under the MIT License - see the LICENSE file for details.
