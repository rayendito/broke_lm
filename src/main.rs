use anyhow::Result;
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs;
use trie_rs::TrieBuilder;

mod train_utils;
use train_utils::{TrainConfig, add_to_ngram_table, export_hashmap};

mod query_utils;
use query_utils::load_model;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Train {},
    Query {
        #[arg(long)]
        prompt: String,
    },
}

fn train() -> Result<()> {
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

    export_hashmap(&ngram_table, &train_cfg.model_name)
}

fn query(input_string: String) -> Result<f32> {
    let model: HashMap<Vec<String>, usize> = load_model()?;
    // let mut builder = TrieBuilder::new();
    // builder.push("すし");
    Ok(6.7)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Train {} => {
            println!("Training/estimating language model...");
            train()
        }
        Commands::Query { prompt } => {
            println!("Prompt input {}", prompt);
            let score = query(prompt)?;
            println!("Score is {}", score);
            Ok(())
        }
    }
}
