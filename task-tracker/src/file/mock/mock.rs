use crate::error::error_invalid_input_str;
use crate::task::task_manager::{MockTaskManagerTrait, TaskManagerTrait};
use crate::task::task_test;
use mockall::predicate::eq;
use std::io::ErrorKind::InvalidInput;

const FILE_NAME: &str = "FILE_TEST";

#[test]
fn test_mock_add_should_fail() {
    let mut mock = MockTaskManagerTrait::default();

    // is too short description
    mock.expect_add().with(eq("f")).returning(|_| {
        Err(error_invalid_input_str(
            FILE_NAME,
            "DESCRIPTION is too short",
        ))
    });
    let result = mock.add("f");
    assert!(result.is_err());
    let err_task = result.unwrap_err().to_string();
    assert_eq!(
        err_task,
        "Error { code: \"FILE_TEST\", kind: InvalidInput, message: \"DESCRIPTION is too short\" }"
    );
}

#[test]
fn test_mock_add_should_success() {
    let mut mock = MockTaskManagerTrait::default();
    mock.expect_add()
        .with(eq("test add successfully"))
        .returning(|_| Ok(task_test::setup_task(2, "test add successfully")));
    let result = mock.add("test add successfully");
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, 2);
    assert_eq!(task.description, "test add successfully".to_string());
    assert_eq!(task.status, "todo");
}

#[test]
fn test_mock_update_description_should_fail() {
    let mut mock = MockTaskManagerTrait::default();
    mock.expect_update_description()
        .with(eq(1), eq("f"))
        .returning(|_, _| {
            Err(error_invalid_input_str(
                FILE_NAME,
                "DESCRIPTION is too short",
            ))
        });
    let result = mock.update_description(1, "f");
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), InvalidInput);
    assert_eq!(
        err.to_string(),
        "Error { code: \"FILE_TEST\", kind: InvalidInput, message: \"DESCRIPTION is too short\" }"
    );
}

#[test]
fn test_mock_update_description_should_success() {
    let mut mock = MockTaskManagerTrait::default();
    mock.expect_update_description()
        .with(eq(1), eq("test update successfully"))
        .returning(|_, _| {
            Ok(task_test::setup_task_status(
                1,
                "test update successfully",
                "todo",
            ))
        });
    let result = mock.update_description(1, "test update successfully");
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.id, 1);
    assert_eq!(task.description, "test update successfully".to_string());
    assert_eq!(task.status, "todo");
}

#[test]
fn test_mock_delete_should_fail() {
    let mut mock = MockTaskManagerTrait::default();
    mock.expect_delete()
        .with(eq(1))
        .returning(|_| Err(error_invalid_input_str(FILE_NAME, "ID not found")));
    let result = mock.delete(1);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.kind(), InvalidInput);
    assert_eq!(
        err.to_string(),
        "Error { code: \"FILE_TEST\", kind: InvalidInput, message: \"ID not found\" }"
    );
}

#[test]
fn test_mock_delete_should_success() {
    let mut mock = MockTaskManagerTrait::default();
    mock.expect_delete().with(eq(1)).returning(|_| Ok(()));
    let result = mock.delete(1);
    assert!(result.is_ok());
}
