use crate::task::task_mark::{MockTaskMarkTrait, TaskMark, TaskMarkTrait};

use chrono::DateTime;
use mockall::predicate::*;
use std::io::Error;

#[test]
fn test_mock_in_progress_should_fail() {
    let mut mock = MockTaskMarkTrait::default();
    /*
     * Task not found
     */
    mock.expect_mark_in_progress().with(eq(1)).returning(|_| {
        Err(Error::new(
            std::io::ErrorKind::NotFound,
            "error: `id` is not found",
        ))
    });
    let result = mock.mark_in_progress(1);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
    assert_eq!(err.to_string(), "error: `id` is not found");

    /*
     * Task is already in 'in-progress' status
     */
    mock.expect_mark_in_progress().with(eq(2)).returning(|_| {
        Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "error: Task is already in 'in-progress' status",
        ))
    });
    let result = mock.mark_in_progress(2);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(
        err.to_string(),
        "error: Task is already in 'in-progress' status"
    );
}

#[test]
fn test_mock_in_progress_should_success() {
    let created_at = DateTime::parse_from_str("1970-01-01 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
        .unwrap()
        .into();

    const TASK_DESC: &str = "test buy one";
    let task_to_update = crate::task::task::Task {
        id: 1,
        description: TASK_DESC.to_string(),
        status: crate::task::task::VALID_STATUSES[0].to_string(),
        created_at: created_at,
        updated_at: created_at,
    };

    let mut mock = MockTaskMarkTrait::default();

    mock.expect_mark_in_progress()
        .with(eq(1))
        .returning(move |_| Ok(task_to_update.clone()));
    let result = mock.mark_in_progress(1);
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, 1);
    assert_eq!(task.description, TASK_DESC.to_string());
}
