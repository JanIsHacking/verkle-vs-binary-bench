use std::sync::Mutex;
use verkle_trie::database::memory_db::MemoryDb;
use verkle_trie::{Trie, TrieTrait, VerkleConfig};
use once_cell::sync::Lazy;
use ipa_multipoint::committer::DefaultCommitter;

pub static CONFIG: Lazy<Mutex<VerkleConfig<MemoryDb>>> =
    Lazy::new(|| Mutex::new(VerkleConfig::new(MemoryDb::new())));

pub fn generate_verkle_tree(keys: &Vec<[u8; 32]>) ->  Trie<MemoryDb, DefaultCommitter> {
    let mut trie = Trie::new(CONFIG.lock().unwrap().clone());

    let key_vals = keys.clone().into_iter().map(|key_bytes| (key_bytes, key_bytes));
    trie.insert(key_vals);
    trie
}
