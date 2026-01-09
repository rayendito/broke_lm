use crate::Trie;
use crate::train_utils::{TrainConfig, add_to_ngram_table, tokenize};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Vocab {
    vocab: HashMap<String, usize>,
    v_size: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NgramLM {
    vocab: Vocab,
    model_hashmap: HashMap<Vec<String>, usize>,
    pub model_trie: Trie,
}

impl NgramLM {
    pub fn new() -> Self {
        let vocabulary = Vocab {
            vocab: HashMap::new(),
            v_size: 0,
        };

        NgramLM {
            vocab: vocabulary,
            model_hashmap: HashMap::new(),
            model_trie: Trie::new(),
        }
    }

    pub fn from_pretrained(model_name: &String) -> Result<NgramLM> {
        let bytes = std::fs::read(format!("{model_name}.bin"))?;
        let model: NgramLM = bincode::deserialize(&bytes)?;
        Ok(model)
    }

    pub fn train_models(&mut self, train_cfg: &TrainConfig) -> Result<()> {
        // dataset
        let contents = fs::read_to_string(&train_cfg.data_path)?;
        let data_raw = contents.lines().filter(|line| !line.trim().is_empty());

        // hash table
        self.model_hashmap = HashMap::new();
        for sentence in data_raw {
            add_to_ngram_table(&mut self.model_hashmap, sentence, train_cfg.n);
        }

        // trie
        self.model_trie = Trie::new();
        for (ngram, count) in &self.model_hashmap {
            self.model_trie.insert(ngram, *count);
        }
        self.model_trie.build_failures();

        self.save(&train_cfg.model_name)?;

        Ok(())
    }

    pub fn query(&self, input_string: &String, hashmap_backend: &bool) -> Result<f32> {
        match hashmap_backend {
            true => {
                println!("Query using hashmap");
                self.estimate_hashmap(input_string)
            }
            false => {
                println!("Query using trie");
                Ok(6.7)
            }
        }
    }

    pub fn estimate_hashmap(&self, input_string: &String) -> Result<f32> {
        let inp_tokenized: Vec<String> = tokenize(input_string);
        println!("Tokenized string");
        println!("{:?}", inp_tokenized);
        Ok(6.7)
    }

    pub fn save(&self, path: &String) -> Result<()> {
        let encoded = bincode::serialize(self)?;
        std::fs::write(format!("{path}.bin"), encoded)?;
        Ok(())
    }
}
