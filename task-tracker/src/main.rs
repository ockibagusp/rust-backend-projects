mod cmd;
mod error;
mod file;
mod list;
mod mark;
mod task;

use crate::cmd::cmd::{Command, CommandTrait};

fn main() {
    let mut cmd = Command::new();
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
