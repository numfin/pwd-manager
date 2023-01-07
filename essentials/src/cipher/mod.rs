use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use eyre::Result;

type Encoder = cbc::Encryptor<aes::Aes256>;
type Decoder = cbc::Decryptor<aes::Aes256>;
type Padding = Pkcs7;

use rand::prelude::*;

use crate::{key::Key, password::PasswordRecord};

pub struct Cipher;
impl Cipher {
    pub fn encrypt(source: &str, key: &Key) -> Result<EncodedMessage> {
        let iv: [u8; 16] = random();
        let encoder = Encoder::new_from_slices(&key.0, &iv)?;
        let encoded_msg = encoder.encrypt_padded_vec_mut::<Padding>(source.as_bytes());

        Ok(EncodedMessage {
            content: ByteCollection(encoded_msg),
            iv: ByteCollection(iv.to_vec()),
        })
    }
    pub fn decrypt(msg: &EncodedMessage, key: &Key) -> Result<String> {
        let decoder = Decoder::new_from_slices(&key.0, &msg.iv.0)?;
        let content = decoder.decrypt_padded_vec_mut::<Padding>(&msg.content.0)?;
        let content = String::from_utf8(content)?;
        Ok(content)
    }
}

pub struct ByteCollection(pub Vec<u8>);
impl ToString for ByteCollection {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

pub struct EncodedMessage {
    pub content: ByteCollection,
    pub iv: ByteCollection,
}
impl EncodedMessage {
    pub fn new(content: &[u8], iv: &[u8]) -> Self {
        Self {
            content: ByteCollection(content.to_vec()),
            iv: ByteCollection(iv.to_vec()),
        }
    }
    pub fn from_record(record: &PasswordRecord) -> Self {
        EncodedMessage::new(&record.password, &record.salt)
    }
}
