use std::fs;
use std::collections::HashMap;
use anyhow::Result;
use clap::{Parser, Subcommand};

mod train_utils;
use train_utils::{TrainConfig, add_to_ngram_table, export_hashmap};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command : Commands,
}

#[derive(Subcommand)]
enum Commands {
    Estimate {},
    Query {
        #[arg(long)]
        prompt : String,
    },
}


fn estimate() -> Result<()> {
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

// fn query(input_string : String) -> f32 {

// }

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Estimate {} => {
            println!("Estimating language model...");
            estimate()
        }
        Commands::Query { prompt } => {
            println!("prompt inputted {}", prompt);
            Ok(())
        }
    }
}
