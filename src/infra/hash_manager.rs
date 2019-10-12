use crate::domain::interface;

#[derive(Clone)]
pub struct HashManager;

impl HashManager {
    pub fn new() -> HashManager {
        HashManager
    }
}

impl interface::IHashManager for HashManager {
    fn hash(&self, raw: String) -> interface::Hash {
        interface::Hash::from_string(bcrypt::hash(raw, 10).unwrap())
    }

    fn verify(&self, hash: interface::Hash, raw: String) -> bool {
        bcrypt::verify(raw, &hash.to_string()).is_ok()
    }
}
