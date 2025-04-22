use chrono::prelude::FixedOffset;
use chrono::{DateTime, Local, Utc};

#[derive(PartialEq, Debug)]
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

#[cfg_attr(debug_assertions, allow(dead_code))]
fn list_cli() {
    // date in local tz
    let date_in_local: DateTime<Local> = Local::now();
    println!("(in local_tz): {date_in_local}");
    // 2025-04-10 10:10:10.000000 +07:00
    let date_str = "2025-04-10 10:10:10.000000 +07:00";
    // let date_str = "2020-04-12 22:10:57 +02:00";
    let mut datetime = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S%.6f %z").unwrap();
    println!("Parsed datetime: {}", datetime);

    let now_utc = Utc::now();
    datetime = now_utc.into();

    println!("now_fixed: {}", datetime);
}

pub fn list() -> Vec<Task> {
    // list_cli();
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
