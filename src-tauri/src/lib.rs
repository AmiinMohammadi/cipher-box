use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce as AesNonce};
use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::{ChaCha20Poly1305, Nonce as ChaNonce};
use rand::Rng;
use sha2::{Digest, Sha256};

#[cfg(not(target_arch = "wasm32"))]
use tauri::generate_handler;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Supported cryptography algorithms.
/// Uses PascalCase to ensure seamless serialization between JS and Rust across both WASM and Tauri.
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub enum Algorithms {
    Aes,
    Chacha,
}

/// Core encryption logic handling both AES-256-GCM and ChaCha20Poly1305.
/// Generates a secure random 12-byte nonce, encrypts the text, and appends the nonce to the resulting ciphertext.
/// The final output is then Base64 encoded for safe transport to the frontend.
fn internal_encrypt(text: String, key: String, algorithm: Algorithms) -> Result<String, String> {
    // 1. Hash the provided user key using SHA-256 to ensure it strictly meets the 32-byte requirement
    let key_hash = Sha256::digest(key.as_bytes());

    // 2. Generate a secure random 12-byte nonce for this specific encryption cycle
    let nonce_bytes: [u8; 12] = rand::thread_rng().gen();

    // 3. Encrypt the data based on the selected algorithm
    let ciphertext = match algorithm {
        Algorithms::Aes => {
            let cipher = Aes256Gcm::new_from_slice(&key_hash).expect("Valid 32-byte key");
            let nonce = AesNonce::from_slice(&nonce_bytes);
            cipher
                .encrypt(nonce, text.as_bytes())
                .map_err(|e| e.to_string())?
        }
        Algorithms::Chacha => {
            let cipher = ChaCha20Poly1305::new_from_slice(&key_hash).expect("Valid 32-byte key");
            let nonce = ChaNonce::from_slice(&nonce_bytes);
            cipher
                .encrypt(nonce, text.as_bytes())
                .map_err(|e| e.to_string())?
        }
    };

    // 4. Combine the nonce and the ciphertext into a single byte vector
    let mut combined = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    // 5. Encode the combined vector as a Base64 string
    Ok(general_purpose::STANDARD.encode(combined))
}

/// Core decryption logic.
/// Decodes the Base64 string, extracts the 12-byte nonce, hashes the key, and attempts decryption.
fn internal_decrypt(text: String, key: String, algorithm: Algorithms) -> Result<String, String> {
    // 1. Decode the Base64 input string back into raw bytes
    let combined = general_purpose::STANDARD
        .decode(text.trim())
        .map_err(|_| "The encrypted text format is invalid!".to_string())?;

    // Ensure the data is at least long enough to contain the 12-byte nonce
    if combined.len() < 12 {
        return Err("The encrypted text is too short!".to_string());
    }

    // 2. Split the bytes: the first 12 bytes are the nonce, the rest is the ciphertext
    let (nonce_bytes, ciphertext) = combined.split_at(12);

    // Hash the key again to match the encryption process
    let key_hash = Sha256::digest(key.as_bytes());

    // 3. Attempt to decrypt based on the selected algorithm
    let decrypted_bytes = match algorithm {
        Algorithms::Aes => {
            let cipher = Aes256Gcm::new_from_slice(&key_hash).expect("Valid 32-byte key");
            let nonce = AesNonce::from_slice(nonce_bytes);
            cipher
                .decrypt(nonce, ciphertext)
                .map_err(|_| "The key, algorithm, or data is corrupted.".to_string())?
        }
        Algorithms::Chacha => {
            let cipher = ChaCha20Poly1305::new_from_slice(&key_hash).expect("Valid 32-byte key");
            let nonce = ChaNonce::from_slice(nonce_bytes);
            cipher
                .decrypt(nonce, ciphertext)
                .map_err(|_| "The key, algorithm, or data is corrupted.".to_string())?
        }
    };

    // 4. Convert the decrypted bytes back into a valid UTF-8 String
    String::from_utf8(decrypted_bytes).map_err(|_| "Decrypted data is not valid UTF-8.".to_string())
}

// ==============================================================================
// Tauri Inter-Process Communication (IPC) Commands
// These functions act as bridges between the Rust backend and the Desktop frontend
// ==============================================================================

#[cfg(not(target_arch = "wasm32"))]
#[tauri::command]
/// Tauri command wrapper for the internal encryption logic
fn encrypt_func(text: String, key: String, algorithm: Algorithms) -> Result<String, String> {
    internal_encrypt(text, key, algorithm)
}

#[cfg(not(target_arch = "wasm32"))]
#[tauri::command]
/// Tauri command wrapper for the internal decryption logic
fn decrypt_func(text: String, key: String, algorithm: Algorithms) -> Result<String, String> {
    internal_decrypt(text, key, algorithm)
}

// ==============================================================================
// WebAssembly (WASM) Exports
// These functions are exposed directly to the browser environment via wasm-bindgen
// ==============================================================================

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
/// WASM export wrapper for the internal encryption logic
pub fn encrypt_func(text: String, key: String, algorithm: Algorithms) -> Result<String, String> {
    internal_encrypt(text, key, algorithm)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
/// WASM export wrapper for the internal decryption logic
pub fn decrypt_func(text: String, key: String, algorithm: Algorithms) -> Result<String, String> {
    internal_decrypt(text, key, algorithm)
}

// ==============================================================================
// Desktop Application Entry Point
// Initializes and runs the Tauri application window and registers the IPC handlers
// ==============================================================================

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// Initializes the Tauri builder, registers plugins, maps the commands, and starts the app.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(generate_handler![encrypt_func, decrypt_func]) // Register IPC commands
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}