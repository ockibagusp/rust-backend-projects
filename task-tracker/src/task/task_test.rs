use std::io::{Error, ErrorKind};

// use mockall::mock;
use crate::task::task::MockTaskTrait;
use crate::task::task::TaskTrait;

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
