use dotenv::dotenv;
use task_tracker::{
    adapters::cmd::cmd::{Command, CommandTrait},
    infrastructure::config::Config,
};

fn main() {
    dotenv().ok();
    let config = Config::from_env();

    let result = Command::run(&config);
    match result {
        Ok(data) => {
            println!("{}", data);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
