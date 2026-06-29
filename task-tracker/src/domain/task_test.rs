use crate::domain::task::{MockTaskTrait, Task, TaskStatus, TaskTrait};
use chrono::DateTime;

pub fn setup_task(id: i32, desciption: &str) -> Task {
    let _created_at = DateTime::parse_from_str(
        "2025-10-13 14:07:06.072493 +07:00",
        "%Y-%m-%d %H:%M:%S%.f %z",
    )
    .expect("Failed to parse created_at");

    let _updated_at = DateTime::parse_from_str(
        "2025-10-13 19:07:06.072493 +07:00",
        "%Y-%m-%d %H:%M:%S%.f %z",
    )
    .expect("Failed to parse updated_at");

    Task {
        id: id,
        description: desciption.to_string(),
        status: TaskStatus::Todo,
        created_at: _created_at,
        updated_at: _updated_at,
    }
}

pub fn setup_task_status(id: i32, desciption: &str, status: TaskStatus) -> Task {
    let mut task_on_update = setup_task(id, desciption);
    task_on_update.status = status;
    task_on_update
}

#[test]
fn test_task_trait_should_fail() {
    let mut mock = MockTaskTrait::new();
    mock.expect_is_validation().returning(move || Err("error"));
    assert_eq!(mock.is_validation(), Err("error"));
}

#[test]
fn test_task_trait_should_success() {
    let mut mock = MockTaskTrait::default();
    mock.expect_is_validation().returning(|| Ok(()));
    assert_eq!(mock.is_validation().unwrap(), ());
}
