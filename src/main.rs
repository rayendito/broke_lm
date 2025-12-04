mod train_config;

use std::fs;
use anyhow::Result;
use std::collections::HashMap;
use train_config::TrainConfig;

fn append_bos_eos(sentence: &str, n: u8) -> String {
    const BOS_TOKEN_SPACE: &str = "<s> ";
    const SPACE_EOS_TOKEN: &str = " </s>";

    let mut appended = String::from("");
    for _ in 0..(n-1){
        appended.push_str(BOS_TOKEN_SPACE);
    }
    appended.push_str(sentence);
    appended.push_str(SPACE_EOS_TOKEN);
    appended
}

fn tokenize(sentence: &String) -> Vec<String> {
    sentence.split_whitespace().map(|s| s.to_string()).collect()
}

fn add_to_ngram_table(ngram_table: &mut HashMap<Vec<String>, usize>, sentence: &str, n: u8) {
    assert!(n > 0, "n must be at least 1 for n-grams");
    
    let bos_eos_appended: String = append_bos_eos(sentence, n);
    let tokenized: Vec<String> = tokenize(&bos_eos_appended);
    
    for gram in tokenized.windows(n as usize) {
        let key: Vec<String> = gram.to_vec(); // convert &[String] → Vec<String>
        *ngram_table.entry(key).or_insert(0) += 1;
    }
}

fn export_hashmap(ngram_table: &HashMap<Vec<String>, usize>, target_file: &String) {
    let mut exportable: HashMap<String, usize> = HashMap::new();
    for (key_vec, count) in ngram_table{
        let key_str = key_vec.join(" ");
        exportable.insert(key_str, *count);
    }
    
    let json_str = serde_json::to_string_pretty(&exportable).expect("Couldn't convert HashMap to str. Why? idk :)");
    std::fs::write(target_file, json_str).expect("Couldn't save to json. Why? idk :)");
}

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
