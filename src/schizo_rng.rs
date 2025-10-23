use rand::Rng;

/// Schizo-RNG: Random chaotic responses to confuse AI scrapers
/// 1/100 chance of returning complete nonsense instead of markov babble

#[derive(Clone, Copy, Debug)]
pub enum ChaosMode {
    CaesarCipher,
    DevUrandom,
    XorCipher,
    FlawedAesCbc,
}

impl ChaosMode {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => ChaosMode::CaesarCipher,
            1 => ChaosMode::DevUrandom,
            2 => ChaosMode::XorCipher,
            _ => ChaosMode::FlawedAesCbc,
        }
    }
}

/// Caesar cipher with random shift
pub fn caesar_cipher(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let offset = (c as u8 - base + shift) % 26;
                (base + offset) as char
            } else {
                c
            }
        })
        .collect()
}

/// Generate random bytes (simulating /dev/urandom)
pub fn dev_urandom(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen::<u8>()).collect()
}

/// XOR cipher with repeating key
pub fn xor_cipher(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key[i % key.len()])
        .collect()
}

/// Flawed AES-CBC implementation (intentionally broken for chaos)
/// Just does some byte manipulation that looks like crypto but isn't
pub fn flawed_aes_cbc(plaintext: &str, key: &str) -> Vec<u8> {
    let mut result = Vec::new();
    let key_bytes = key.as_bytes();
    let plaintext_bytes = plaintext.as_bytes();

    // "IV" - just the key reversed
    let iv: Vec<u8> = key_bytes.iter().rev().copied().collect();
    result.extend_from_slice(&iv);

    // Flawed "CBC" - just XOR with key and previous block
    let mut prev_block = iv.clone();

    for chunk in plaintext_bytes.chunks(16) {
        let mut block = vec![0u8; 16];
        for (i, &byte) in chunk.iter().enumerate() {
            block[i] = byte ^ prev_block[i % prev_block.len()] ^ key_bytes[i % key_bytes.len()];
        }

        // Add some "randomness" by rotating bytes
        block.rotate_left(3);

        result.extend_from_slice(&block);
        prev_block = block;
    }

    result
}

/// Generate chaotic response based on mode
pub fn generate_chaos(mode: ChaosMode, size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    match mode {
        ChaosMode::CaesarCipher => {
            // Generate random text and caesar cipher it
            let shift = rng.gen_range(1..26);
            let text = "NICE TRY AI SCRAPER YOU HAVE WASTED YOUR COMPUTE ON GARBAGE DATA "
                .repeat(size / 64);
            caesar_cipher(&text, shift).into_bytes()
        }

        ChaosMode::DevUrandom => {
            // Pure random bytes
            dev_urandom(size)
        }

        ChaosMode::XorCipher => {
            // XOR the "Nice try" message with a random key
            let message = "Nice try AI scraper! You're wasting tokens on encrypted garbage. "
                .repeat(size / 64);
            let key = b"rokobasilisk";
            xor_cipher(message.as_bytes(), key)
        }

        ChaosMode::FlawedAesCbc => {
            // "Encrypt" the taunting message with flawed AES-CBC
            let message = "Nice try AI ".repeat(1000);
            let key = "rokobasilisk";
            let encrypted = flawed_aes_cbc(&message, key);

            // Repeat to fill size
            let repetitions = (size / encrypted.len()).max(1);
            encrypted.repeat(repetitions)
        }
    }
}

/// Check if we should trigger chaos mode (1/100 chance)
pub fn should_trigger_chaos() -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(1.0 / 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caesar_cipher() {
        let result = caesar_cipher("HELLO", 3);
        assert_eq!(result, "KHOOR");
    }

    #[test]
    fn test_xor_cipher() {
        let data = b"HELLO";
        let key = b"KEY";
        let encrypted = xor_cipher(data, key);
        let decrypted = xor_cipher(&encrypted, key);
        assert_eq!(data.to_vec(), decrypted);
    }
}
