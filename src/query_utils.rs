use anyhow::Result;
use std::collections::HashMap;

pub fn load_model() -> Result<HashMap<Vec<String>, usize>> {
    const MODEL_PATH: &str = "model.bin";
    let bytes = std::fs::read(MODEL_PATH).expect("Failed to read bytes");
    let model: HashMap<Vec<String>, usize> =
        bincode::deserialize(&bytes).expect("Failed to read bytes to model");

    for (ngram, count) in &model {
        println!("{:?} -> {}", ngram, count)
    }

    Ok(model)
}
