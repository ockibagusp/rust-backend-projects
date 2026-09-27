use std::env;

#[derive(Clone)]
pub struct Config {
    pub env_json: String,
}

impl Config {
    pub fn from_env() -> Self {
        let env_json = env::var("ENV_JSON").expect("ENV_JSON not found");

        Self { env_json }
    }
}
