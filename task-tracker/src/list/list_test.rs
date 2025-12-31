use crate::file::files;
use crate::list::list::{List, ListTrait, MockListTrait};
use mockall::predicate::*;

#[test]
// func. `index` is the same as (equals) `list`
fn test_mock_list() {
    /*
     * empty list
     */
    let mut mock = MockListTrait::default();
    mock.expect_index().times(1).return_once(|| vec![]);
    assert_eq!(mock.index(), vec![]);

    /*
     * one task in list
     */
    // test with one task
    let task = files::tests::setup_task(1, "test one");
    let task_one = task.clone();

    mock.expect_index()
        .times(1)
        .return_once(move || vec![task.clone()]);
    assert_eq!(mock.index(), vec![task_one]);
}

#[test]
fn test_mock_todo() {
    let created_at =
        chrono::DateTime::parse_from_str("1970-01-01 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
            .unwrap()
            .into();

    const TASK_DESC: &str = "test buy one";
    let mut _task = crate::task::task::Task {
        id: 2,
        description: TASK_DESC.to_string(),
        status: crate::task::task::VALID_STATUSES[0].to_string(),
        created_at: created_at,
        updated_at: created_at,
    };

    let mut mock = MockListTrait::default();
    mock.expect_todo()
        .with()
        .returning(move || vec![_task.clone()]);
    let result = mock.todo();
    assert_eq!(result[0].id, 2);
    assert_eq!(result[0].description, TASK_DESC);
    assert_eq!(result[0].status, crate::task::task::VALID_STATUSES[0]);
}
