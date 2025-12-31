use std::io::{Error, ErrorKind};

// use mockall::mock;
use crate::task::task::VALID_STATUSES;
use crate::task::task::{MockTaskTrait, Task, TaskTrait};
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
        status: String::from(VALID_STATUSES[0]),
        created_at: _created_at,
        updated_at: _updated_at,
    }
}

pub fn setup_task_status(id: i32, desciption: &str, status: &str) -> Task {
    let mut task_on_update = setup_task(id, desciption);
    task_on_update.status = status.to_string();
    task_on_update
}

#[test]
fn test_task_trait_should_fail() {
    let mut mock = MockTaskTrait::new();
    let invalid_input = ErrorKind::InvalidInput;

    mock.expect_is_validation()
        .returning(move || Err(Error::new(invalid_input, "error")));
    assert_eq!(mock.is_validation().is_err(), true);
}

#[test]
fn test_task_trait_should_success() {
    let mut mock = MockTaskTrait::default();
    mock.expect_is_validation().returning(|| Ok(()));
    assert_eq!(mock.is_validation().unwrap(), ());
}
