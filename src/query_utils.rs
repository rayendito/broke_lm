use crate::trie::Trie;
use anyhow::Result;
use std::collections::HashMap;

pub fn load_model_hashmap() -> Result<HashMap<Vec<String>, usize>> {
    const MODEL_PATH: &str = "model_hashmap.bin";
    let bytes = std::fs::read(MODEL_PATH).expect("Failed to read bytes");
    let model: HashMap<Vec<String>, usize> =
        bincode::deserialize(&bytes).expect("Failed to read bytes to model");
    Ok(model)
}

pub fn load_model_trie() -> Result<Trie> {
    const MODEL_PATH: &str = "model_trie.bin";
    let bytes = std::fs::read(MODEL_PATH).expect("Failed to read bytes");
    let model: Trie = bincode::deserialize(&bytes).expect("Failed to read bytes to model");
    Ok(model)
}
