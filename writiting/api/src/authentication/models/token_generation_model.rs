use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub struct TokenGenerator;

impl TokenGenerator {
    pub fn generate_alphanumeric_token(length: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect()
    }

    pub fn generate_hex_token(length: usize) -> String {
        let mut rng = thread_rng();
        (0..length)
            .map(|_| format!("{:x}", rng.gen::<u8>()))
            .collect()
    }
}