mod train_config;

use train_config::TrainConfig;
use std::fs;
use anyhow::Result;

fn main() -> Result<()> {
    let train_config_raw = fs::read_to_string("train_config.toml")?;
    let cfg: TrainConfig = toml::from_str(&train_config_raw)?;
    println!("Loaded config {:?}", cfg);
    Ok(())
}
