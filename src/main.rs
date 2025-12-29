use anyhow::Result;
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::fs;
use broke_lm::train_utils::{TrainConfig, add_to_ngram_table, export_hashmap, export_trie};
use broke_lm::query_utils::{load_model_hashmap, load_model_trie};
use broke_lm::Trie;

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

        #[arg(short = 'm', long)]
        use_hashmap: bool, // use hashmap backend
    },
}

fn train_hashmap(train_cfg: &TrainConfig) -> Result<()> {
    // python one liners don't work because someone needs to own the thing always
    let contents = fs::read_to_string(&train_cfg.data_path)?;
    let data_raw = contents.lines().filter(|line| !line.trim().is_empty());

    let mut ngram_table: HashMap<Vec<String>, usize> = HashMap::new();

    for sentence in data_raw {
        add_to_ngram_table(&mut ngram_table, sentence, train_cfg.n);
    }

    export_hashmap(&ngram_table, &train_cfg.model_name)
}

fn train_trie(train_cfg: &TrainConfig) -> Result<()> {
    let model: HashMap<Vec<String>, usize> = load_model_hashmap()?; // get grams from hashmap
    let mut trie = Trie::new();
    for (ngram, count) in &model {
        trie.insert(ngram, *count);
    }
    trie.build_failures();
    export_trie(&trie, &train_cfg.model_name)
}

fn query(input_string: &String, backend: &bool) -> Result<f32> {
    println!("{:?}", input_string);
    println!("{:?}", backend);
    match backend {
        true => {
            let model: HashMap<Vec<String>, usize> = load_model_hashmap()?;
            println!("{:?}", model);
        }
        false => {
            let model: Trie = load_model_trie()?;
            model.debug_print();
        }
    }
    Ok(6.7)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Train {} => {
            let train_config_raw = fs::read_to_string("train_config.toml")?;
            let train_cfg: TrainConfig = toml::from_str(&train_config_raw)?;
            println!("Loaded config {:?}", train_cfg);
            println!("Training/estimating language model...");
            
            println!("Training hashmap");
            let _ = train_hashmap(&train_cfg);
            println!("Training trie");
            let _ = train_trie(&train_cfg);
        }
        Commands::Query {
            prompt,
            use_hashmap,
        } => {
            println!("Prompt input {}", prompt);
            let score = query(prompt, use_hashmap)?;
            println!("Score is {}", score);
        }
    }
    Ok(())
}
