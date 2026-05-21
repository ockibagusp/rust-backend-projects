mod cmd;
mod error;
mod file;
mod list;
mod mark;
mod task;

use crate::cmd::cmd::{Command, CommandTrait};
use dotenv::dotenv;
use std::env;

fn main() {
    // TODO: move "ENV_JSON" to src/file.rs
    dotenv().ok();
    let env_json = env::var("ENV_JSON").expect("ENV_JSON not found");

    let mut cmd = Command::new(env_json.leak());
    let result = cmd.run();
    match result {
        Ok(data) => {
            println!("{}", data);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
