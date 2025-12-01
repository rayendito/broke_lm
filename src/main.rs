mod train_config;

use train_config::TrainConfig;
use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let train_config_raw = fs::read_to_string("train_config.toml")?;
    let train_cfg: TrainConfig = toml::from_str(&train_config_raw)?;
    println!("Loaded config {:?}", train_cfg);
    
    let data_raw = fs::read_to_string(&train_cfg.data_path)?;
    
    
    println!("{} gram language model", train_cfg.n);
    println!("language model will be saved in {}", train_cfg.output);
    println!("First few lines of data_raw");
    for line in data_raw.lines().take(5){
        println!("{}", line);
    }
    
    Ok(())
}
