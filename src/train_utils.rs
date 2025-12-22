use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct TrainConfig {
    pub data_path: String,
    pub n: u8,
    pub model_output_path: String,
}

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

pub fn add_to_ngram_table(ngram_table: &mut HashMap<Vec<String>, usize>, sentence: &str, n: u8) {
    assert!(n > 0, "n must be at least 1 for n-grams");
    
    let bos_eos_appended: String = append_bos_eos(sentence, n);
    let tokenized: Vec<String> = tokenize(&bos_eos_appended);
    
    for gram in tokenized.windows(n as usize) {
        let key: Vec<String> = gram.to_vec(); // convert &[String] → Vec<String>
        *ngram_table.entry(key).or_insert(0) += 1;
    }
}

pub fn export_hashmap(ngram_table: &HashMap<Vec<String>, usize>, target_file: &String) {
    let mut exportable: HashMap<String, usize> = HashMap::new();
    for (key_vec, count) in ngram_table{
        let key_str = key_vec.join(" ");
        exportable.insert(key_str, *count);
    }
    
    let json_str = serde_json::to_string_pretty(&exportable).expect("Couldn't convert HashMap to str. Why? idk :)");
    std::fs::write(target_file, json_str).expect("Couldn't save to json. Why? idk :)");
}