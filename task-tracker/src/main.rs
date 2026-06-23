use crate::cmd::cmd::{Command, CommandTrait};
use task_tracker::cmd;

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
