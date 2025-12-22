use crate::file::files;
use crate::list::list::{List, ListTrait, MockListTrait};

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
