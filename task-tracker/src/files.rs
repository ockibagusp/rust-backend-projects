use crate::tasks;

use chrono::prelude::FixedOffset;
use chrono::{DateTime, Utc};

// https://medium.com/@aleksej.gudkov/rust-write-to-file-example-a-practical-guide-51c24695aa80

fn create(description: &str) -> tasks::Task {
    let now_utc = Utc::now();
    let now_fixed: DateTime<FixedOffset> = now_utc.into();

    tasks::Task {
        id: 1,
        description: description.to_string(),
        status: String::from("todo"),
        created_at: now_fixed,
        updated_at: now_fixed,
    }
}
