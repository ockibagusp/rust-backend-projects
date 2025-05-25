use std::io::Write;
use tasks::_FILE_NAME;

use chrono::prelude::FixedOffset;
use chrono::{DateTime, Utc};

use crate::tasks;

// https://medium.com/@aleksej.gudkov/rust-write-to-file-example-a-practical-guide-51c24695aa80

#[allow(dead_code)]
fn create(description: &str) -> tasks::Task {
    let now_utc = Utc::now();
    let now_fixed: DateTime<FixedOffset> = now_utc.into();

    let add_task = tasks::Task {
        id: 1,
        description: description.to_string(),
        status: String::from("todo"),
        created_at: now_fixed,
        updated_at: now_fixed,
    };

    let add_tasklist = tasks::TaskList {
        next_count: 2,
        tasks: vec![&add_task],
    };

    let json_data =
        serde_json::to_string_pretty(&add_tasklist).expect("Unable to serialize task list");

    let mut file = std::fs::File::create(_FILE_NAME).expect("Failed to create file");
    if let Err(error) = file.write_all(json_data.as_bytes()) {
        eprintln!("Unable to write to file: {}", error);
        std::process::exit(1);
    }

    add_task
}
