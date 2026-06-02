use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit, Nonce as AesNonce};
use base64::{engine::general_purpose, Engine as _};
use chacha20poly1305::{ChaCha20Poly1305, Nonce as ChaNonce};
use rand::RngCore;
use sha2::{Digest, Sha256};
use tauri::generate_handler;
#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Algorithms {
    Aes,
    Chacha,
}

#[tauri::command]
fn encrypt_func(text: String, key: String, algorithm: Algorithms) -> Result<String, String> {
    let key_hash = Sha256::digest(key.as_bytes());

    // ۲. تولید ۱۲ بایت تصادفی برای Nonce
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);

    // ۳. انجام عملیات رمزنگاری بر اساس الگوریتم انتخابی
    let ciphertext = match algorithm {
        Algorithms::Aes => {
            let cipher = Aes256Gcm::new_from_slice(&key_hash).map_err(|e| e.to_string())?;
            let nonce = AesNonce::from_slice(&nonce_bytes);
            cipher
                .encrypt(nonce, text.as_bytes())
                .map_err(|e| e.to_string())?
        }
        Algorithms::Chacha => {
            let cipher = ChaCha20Poly1305::new_from_slice(&key_hash).map_err(|e| e.to_string())?;
            let nonce = ChaNonce::from_slice(&nonce_bytes);
            cipher
                .encrypt(nonce, text.as_bytes())
                .map_err(|e| e.to_string())?
        }
    };

    // ۴. چسباندن Nonce به ابتدای متن رمز شده برای استفاده در زمان رمزگشایی
    let mut combined = nonce_bytes.to_vec();
    combined.extend_from_slice(&ciphertext);

    // ۵. تبدیل خروجی باینری به یک رشته متنی خوانیِ Base64 برای فرانت‌اَند
    Ok(general_purpose::STANDARD.encode(combined))
}
#[tauri::command]
fn decrypt_func(text: String, key: String, algorithm: Algorithms) -> Result<String, String> {
    // ۱. تبدیل دوباره رشته Base64 به باینری
    let combined = general_purpose::STANDARD
        .decode(text)
        .map_err(|_| "فرمت متن رمزگذاری شده معتبر نیست!".to_string())?;

    if combined.len() < 12 {
        return Err("طول متن رمز شده کوتاه است!".to_string());
    }

    // ۲. تفکیک ۱۲ بایت اول (Nonce) از بقیه متن (Ciphertext)
    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let key_hash = Sha256::digest(key.as_bytes());

    // ۳. انجام عملیات رمزگشایی
    let decrypted_bytes = match algorithm {
        Algorithms::Aes => {
            let cipher = Aes256Gcm::new_from_slice(&key_hash).map_err(|e| e.to_string())?;
            let nonce = AesNonce::from_slice(nonce_bytes);
            cipher
                .decrypt(nonce, ciphertext)
                .map_err(|_| "رمزگشایی شکست خورد! احتمالا کلید اشتباه است.".to_string())?
        }
        Algorithms::Chacha => {
            let cipher = ChaCha20Poly1305::new_from_slice(&key_hash).map_err(|e| e.to_string())?;
            let nonce = ChaNonce::from_slice(nonce_bytes);
            cipher
                .decrypt(nonce, ciphertext)
                .map_err(|_| "رمزگشایی شکست خورد! احتمالا کلید اشتباه است.".to_string())?
        }
    };

    // ۴. تبدیل بایت‌های رمزگشایی شده به متن رشته‌ای قابل فهم
    String::from_utf8(decrypted_bytes).map_err(|_| "خطا در تبدیل متن به UTF-8".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(generate_handler![encrypt_func, decrypt_func])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
