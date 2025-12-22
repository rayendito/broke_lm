use std::fs;
use anyhow::Result;
use std::collections::HashMap;

mod train_utils;
use train_utils::TrainConfig;
use train_utils::add_to_ngram_table;
use train_utils::export_hashmap;


fn main() -> Result<()> {
    let train_config_raw = fs::read_to_string("train_config.toml")?;
    let train_cfg: TrainConfig = toml::from_str(&train_config_raw)?;
    println!("Loaded config {:?}", train_cfg);

    // python one liners don't work because someone needs to own the thing always
    let contents = fs::read_to_string(&train_cfg.data_path)?;
    let data_raw = contents.lines().filter(|line| !line.trim().is_empty());
    
    let mut ngram_table: HashMap<Vec<String>, usize> = HashMap::new();
    
    for sentence in data_raw {
        add_to_ngram_table(&mut ngram_table, sentence, train_cfg.n);
    }
    
    export_hashmap(&ngram_table, &train_cfg.model_output_path);

    Ok(())
}
