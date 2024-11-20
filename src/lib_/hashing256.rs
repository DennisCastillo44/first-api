use sha2::{Sha256, Digest};
use rand::prelude::*;

pub struct Hash {
    pub input: String
}

impl Hash {
    
    pub fn new(input: String) -> Hash {
        Hash {input: input}
    }

    pub fn generate_hash(&self) -> String {
        let mut hash256 = Sha256::new();
        hash256.update(&self.input.as_bytes());
        let result = hash256.finalize();
        hex::encode(result)
    }
}

pub struct Salt;

impl Salt {
    
    pub fn generate_salt() -> String {
        let salt:[u8; 16] =  rand::thread_rng().gen();
        hex::encode(salt)
    }

}