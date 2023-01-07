use sha2::{Digest, Sha256};

pub struct Key(pub [u8; 32]);
impl Key {
    pub fn new(content: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        let hash: [u8; 32] = *hasher.finalize().as_ref();

        Self(hash)
    }
}
