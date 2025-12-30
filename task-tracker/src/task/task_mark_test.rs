use crate::task::task_mark::{MockTaskMarkTrait, TaskMark, TaskMarkTrait};

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
