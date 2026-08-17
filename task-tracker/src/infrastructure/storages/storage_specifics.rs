use crate::application::error_impl::{panic_invalid_input, panic_not_found_input};
use crate::domain::task::{Task, TaskTrait};
use crate::infrastructure::storages::storage::FILE_NAME;

// specifics of list function for testing purposes
// IMPORTANT: the panic!
//
// parameters methods:
// - tasks_string: the string representation of the tasks or default("" or "[]"), which is expected to be in JSON format
pub fn specifics_of_list(tasks_string: &str) -> Vec<Task> {
    let tasks = serde_json::from_str(tasks_string).unwrap_or_default();
    tasks
}

// specifics of add function for testing purposes
// IMPORTANT: the panic! for example: `Task` object should no longer be validated
//
// parameters methods:
// - tasks_string: the string representation of the tasks or default("" or "[]"), which is expected to be in JSON format
// - add_task: the Task object that we want to add in the list of tasks
pub fn specifics_of_add(tasks_string: &str, add_task: &Task) -> Vec<Task> {
    if let Err(e) = add_task.is_validation() {
        panic_invalid_input(FILE_NAME, e);
    }

    // You probably want to deserialize tasks_string or default into Vec<Task>
    let mut tasks: Vec<Task> = serde_json::from_str(tasks_string).unwrap_or_default();
    tasks.push(add_task.clone());
    tasks
}

// specifics of update function for testing purposes
// IMPORTANT: the panic! for example: `Task` object should no longer be validated
//
// parameters methods:
// - tasks_string: the string representation of the tasks or default("" or "[]"), which is expected to be in JSON format
// - id: the ID of the Task object that we want to update in the list of tasks
// - update_task: the Task object that we want to update in the list of tasks
pub fn specifics_of_update(tasks_string: String, id: i32, update_task: &Task) -> Vec<Task> {
    // // TODO: very important!
    // update_task.updated_at = Local::now().into();

    if let Err(e) = update_task.is_validation() {
        panic_invalid_input(FILE_NAME, e);
    }

    // You probably want to deserialize tasks_string into Vec<Task>
    let mut tasks: Vec<Task> = serde_json::from_str(&tasks_string).unwrap_or_default();
    let mut index_to_update = false;
    for task in tasks.iter_mut() {
        if task.id == id {
            *task = update_task.clone();
            index_to_update = true;
            break;
        }
    }

    if !index_to_update {
        panic_not_found_input(
            FILE_NAME,
            &format!("failed to update task: ID not found (id: {})", id),
        );
    }

    tasks
}

pub fn specifics_of_delete(tasks_string: String, id: i32) -> Vec<Task> {
    // You probably want to deserialize tasks_string into Vec<Task>
    let mut tasks: Vec<Task> = serde_json::from_str(&tasks_string).unwrap_or_default();
    let mut index_to_remove = false;
    for (i, task) in tasks.iter().enumerate() {
        if task.id == id {
            tasks.remove(i);
            index_to_remove = true;
            break;
        }
    }

    if !index_to_remove {
        panic_not_found_input(
            FILE_NAME,
            &format!("failed to delete task: ID not found (id: {})", id),
        );
    }
    tasks
}

#[cfg(test)]
pub mod tests {
    // TODO: a single core test
    // // $ cargo test -- --test-threads=1
    use crate::{
        domain::{
            task::{Task, TaskStatus},
            task_test::{setup_task, setup_task_status},
        },
        infrastructure::storages::storage_specifics::{
            specifics_of_add, specifics_of_delete, specifics_of_list, specifics_of_update,
        },
    };

    #[test]
    fn test_json_add_fail() {
        let got: Vec<super::Task> = vec![];
        let want = specifics_of_add("[]", &setup_task(1, "test add fail"));
        assert_ne!(got, want);
    }

    #[test]
    fn test_json_add_success() {
        let got = vec![setup_task(1, "test add success")];
        let want = specifics_of_add("[]", &setup_task(1, "test add success"));
        assert_eq!(got, want);
    }

    #[test]
    fn test_json_list() {
        let got = vec![setup_task(1, "test list")];
        let want = specifics_of_list(
            "[\n  {\n    \"id\": 1,\n    \"description\": \"test list\",\n    \"status\": \"todo\",\n    \"created_at\": \"2025-10-13T14:07:06.072493+07:00\",\n    \"updated_at\": \"2025-10-13T19:07:06.072493+07:00\"\n  }\n]",
        );
        assert_eq!(got, want);
    }

    #[test]
    fn test_json_update_fail() {
        let got = vec![setup_task_status(1, "test update fail", TaskStatus::Todo)];
        let want = specifics_of_update(
            "[\n  {\n    \"id\": 1,\n    \"description\": \"test update fail\",\n    \"status\": \"in-progress\",\n    \"created_at\": \"2025-10-13T14:07:06.072493+07:00\",\n    \"updated_at\": \"2025-10-13T19:07:06.072493+07:00\"\n  }\n]".to_owned(),
            1,
            &mut setup_task_status(1, "test update fail", TaskStatus::Done),
        );
        assert_ne!(got, want);
    }

    #[test]
    fn test_json_update_success() {
        let got = vec![setup_task_status(
            1,
            "test update success",
            TaskStatus::InProgress,
        )];
        let want = specifics_of_update(
            "[\n  {\n    \"id\": 1,\n    \"description\": \"test update success\",\n    \"status\": \"in-progress\",\n    \"created_at\": \"2025-10-13T14:07:06.072493+07:00\",\n    \"updated_at\": \"2025-10-13T19:07:06.072493+07:00\"\n  }\n]".to_owned(),
            1,
            &setup_task_status(1, "test update success", TaskStatus::InProgress),
        );
        assert_eq!(got, want);
    }

    #[test]
    #[should_panic]
    fn test_json_delete_fail() {
        specifics_of_delete(
            "Error\n------------------\ncode   : FILE\nkind   : NotFound\nmessage: \"failed to delete task: ID not found (id: 2)\"\n++++++++++++++++++".to_owned(),
            2,
        );
    }

    #[test]
    fn test_json_delete_success() {
        let got: Vec<Task> = vec![];
        let want = specifics_of_delete(
            "[\n  {\n    \"id\": 1,\n    \"description\": \"test delete success\",\n    \"status\": \"in-progress\",\n    \"created_at\": \"2025-10-13T14:07:06.072493+07:00\",\n    \"updated_at\": \"2025-10-13T19:07:06.072493+07:00\"\n  }\n]".to_owned(),
            1,
        );
        assert_eq!(got, want);
    }
}
