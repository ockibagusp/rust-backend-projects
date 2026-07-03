use crate::cmd::cmd::{Command, CommandTrait};
use dotenv::dotenv;
use task_tracker::{cmd, infrastructure::config::Config};

fn main() {
    dotenv().ok();
    let config = Config::from_env();

    let mut cmd = Command::new();
    let result = cmd.run(&config);
    match result {
        Ok(data) => {
            println!("{}", data);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
