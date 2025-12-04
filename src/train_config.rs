use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TrainConfig {
    pub data_path: String,
    pub n: usize,
    pub output: String,
}
