use anyhow::Result;
use broke_lm::NgramLM;
use broke_lm::train_utils::TrainConfig;
use clap::{Parser, Subcommand};
use std::fs;

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

// fn query(input_string: &String, backend: &bool) -> Result<f32> {
//     match backend {
//         true => {
//             let inp_tokenized: Vec<String> = tokenize(input_string);
//             let model: HashMap<Vec<String>, usize> = load_model_hashmap()?;
//             println!("Tokenized string");
//             println!("{:?}", inp_tokenized);
//             println!("Query using hashmap");
//             return Ok(6.7)
//         }
//         false => {
//             let model: Trie = load_model_trie()?;
//             println!("Query using trie");
//             model.estimate(input_string)
//         }
//     }
// }

fn main() -> Result<()> {
    // training config
    let cli = Cli::parse();
    let train_config_raw = fs::read_to_string("train_config.toml")?;
    let train_cfg: TrainConfig = toml::from_str(&train_config_raw)?;

    match &cli.command {
        Commands::Train {} => {
            // instantiating model
            let mut ngram_model = NgramLM::new();
            println!("Loaded config {:?}", train_cfg);

            ngram_model.train_models(&train_cfg)?;
            // ngram_model.estimate_hashmap(&s);
        }
        Commands::Query {
            prompt,
            use_hashmap,
        } => {
            let mut ngram_model = NgramLM::from_pretrained(&train_cfg.model_name)?;
            ngram_model.model_trie.debug_print();
        }
    }
    Ok(())
}
