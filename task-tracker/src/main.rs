mod cmd;
mod error;
mod file;
mod help;
mod list;
mod mark;
mod task;

use crate::cmd::cmd::{Command, CommandTrait};

fn main() {
    let mut main = Command::new("tasks.json");
    let args: Vec<String> = std::env::args().collect();
    let result = main.run(&args);
    match result {
        Ok(data) => {
            println!("{}", data);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
