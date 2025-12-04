use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TrainConfig {
    pub data_path: String,
    pub n: u8,
    pub model_output_path: String,
}
