use chrono::DateTime;
use chrono::prelude::FixedOffset;

pub(crate) static _FILE_NAME: &'static str = "task-cli.json";

#[derive(PartialEq, Debug, serde::Serialize)]
pub struct TaskList<'a> {
    pub next_count: i32,
    pub tasks: Vec<&'a Task>,
}

#[derive(PartialEq, Clone, Debug, serde::Serialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub status: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

pub fn add(input: &str) -> Task {
    return Task {
        id: 1,
        description: input.to_string(),
        status: String::from("todo"),
        created_at: DateTime::parse_from_str(
            "2025-04-10 10:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into(),
        updated_at: DateTime::parse_from_str(
            "2025-04-10 10:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into(),
    };
}

pub fn update(id: i32, input: &str) -> Task {
    // let now_utc = Utc::now();
    // let now_fixed: DateTime<FixedOffset> = now_utc.into();
    // println!("now_fixed: {}", now_fixed);

    return Task {
        id,
        description: input.to_string(),
        status: String::from("todo"),
        created_at: DateTime::parse_from_str(
            "2025-04-10 10:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into(),
        updated_at: DateTime::parse_from_str(
            "2025-04-10 14:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into(),
    };
}

pub fn delete(id: i32) -> bool {
    if id != 1 {
        return false;
    }
    return true;
}

pub fn list() -> Vec<Task> {
    let task1 = Task {
        id: 1,
        description: String::from("buy milk"),
        status: String::from("todo"),
        created_at: DateTime::parse_from_str(
            "2025-04-10 10:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into(),
        updated_at: DateTime::parse_from_str(
            "2025-04-10 10:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into(),
    };
    let task2 = Task {
        id: 2,
        description: "buy bread".to_string(),
        status: "in-progress".to_string(),
        created_at: DateTime::parse_from_str(
            "2025-04-12 12:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into(),
        updated_at: DateTime::parse_from_str(
            "2025-04-12 12:10:10.000000 +07:00",
            "%Y-%m-%d %H:%M:%S%.6f %z",
        )
        .unwrap()
        .into(),
    };
    vec![task1, task2]
}
