mod train_config;

use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use train_config::TrainConfig;

fn tokenize(text: &str, n: usize) -> Vec<String> {
    text.split_whitespace().map(|s| s.to_string()).collect()
}

// fn count_ngrams(tokens: &Vec<String>, n: usize) -> HashMap<Vec<String>, usize> {

// }

fn main() -> Result<()> {
    let train_config_raw = fs::read_to_string("train_config.toml")?;
    let train_cfg: TrainConfig = toml::from_str(&train_config_raw)?;
    println!("Loaded config {:?}", train_cfg);
    println!("Printing {} so it silences the warning", train_cfg.n);
    println!("Printing {} so it silences the warning", train_cfg.output);

    let data_raw = fs::read_to_string(&train_cfg.data_path)?
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<_>>();

    println!("First few lines of data_raw");
    for line in data_raw.lines().take(5) {
        println!("{}", line);
    }

    let tokens = tokenize(&data_raw);
    for tok in tokens.iter().take(5) {
        println!("{}", tok);
    }

    Ok(())
}
