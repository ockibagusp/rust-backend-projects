use crate::error::{panic_invalid_input, panic_not_found_input};
use crate::task::task::{Task, TaskTrait};

use std::fs;
use std::fs::{File as std_file, OpenOptions};
use std::io::Read;

const FILE_NAME: &str = "FILE";

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct File {
    json_str: &'static str,
}

// specifics of list function for testing purposes
// IMPORTANT: the panic!
//
// parameters methods:
// - tasks_string: the string representation of the tasks or default("" or "[]"), which is expected to be in JSON format
fn specifics_of_list(tasks_string: &str) -> Vec<Task> {
    let tasks = serde_json::from_str(tasks_string).unwrap_or_default();
    tasks
}

// specifics of add function for testing purposes
// IMPORTANT: the panic! for example: `Task` object should no longer be validated
//
// parameters methods:
// - tasks_string: the string representation of the tasks or default("" or "[]"), which is expected to be in JSON format
// - add_task: the Task object that we want to add in the list of tasks
fn specifics_of_add(tasks_string: &str, add_task: &Task) -> Vec<Task> {
    if let Err(e) = add_task.is_validation() {
        panic_invalid_input::<String>(FILE_NAME, e.to_string());
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
fn specifics_of_update(tasks_string: String, id: i32, update_task: &Task) -> Vec<Task> {
    if let Err(e) = update_task.is_validation() {
        panic_invalid_input::<String>(FILE_NAME, e.to_string());
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
        panic_not_found_input::<String>(
            FILE_NAME,
            format!("failed to update task: ID not found (id: {})", id),
        );
    }

    tasks
}

fn specifics_of_delete(tasks_string: String, id: i32) -> Vec<Task> {
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
        panic_not_found_input::<String>(
            FILE_NAME,
            format!("failed to delete task: ID not found (id: {})", id),
        );
    }
    tasks
}

// TDD
// ✅ ❔ ❌
// 1. buatlah struktur data File dengan method new, list, add, update, delete
// => 1. create the File data structure with methods new, list, add, update
// ---------------------------
// 1.1. buat method `new` untuk inisialisasi File ✅
// => 1.1. create a `new` method to initialize the File
// 1.2. buat method `list` untuk mendapatkan daftar Task ✅
// => 1.2. create a `list` method to get a list of the Tasks
// 1.3. buat method `add` untuk menambahkan Task baru ✅
// => 1.3. create an `add` method to add a new Task
// 1.4. buat method `update` untuk memperbarui Task berdasarkan ID ✅
// => 1.4. create an `update` method to update the Task by ID
// 1.5. buat method `delete` untuk menghapus Task berdasarkan ID ✅
// => 1.5. create a `delete` method to delete Tasks based on its ID
#[allow(dead_code)]
impl File {
    pub fn new(json_str: &'static str) -> Self {
        Self::new_open_options(json_str);
        Self { json_str }
    }

    fn name(&self) -> &str {
        &self.json_str
    }

    fn new_open_options(file_name: &'static str) -> () {
        if !fs::metadata(file_name).is_ok() {
            let _ = OpenOptions::new()
                .write(true)
                .create_new(true) // Will error if file already exists
                .open(file_name)
                .unwrap();

            Self::to_file_by_json(file_name, vec![]);
        }
    }

    fn get_open_options(&self) -> std_file {
        let file = OpenOptions::new().read(true).open(self.json_str);
        if file.is_err() {
            panic_not_found_input(FILE_NAME, "failed to open file");
        }

        file.unwrap()
    }

    fn tasks_str(&self) -> String {
        let mut tasks_file = &self.get_open_options();

        let mut tasks_string = String::new();
        let _ = tasks_file.read_to_string(&mut tasks_string);
        tasks_string
    }

    fn to_file_by_json(file_name: &'static str, tasks: Vec<Task>) -> () {
        let json_string = serde_json::to_string_pretty(&tasks).unwrap();
        if fs::write(file_name, json_string).is_err() {
            panic_invalid_input::<&str>(FILE_NAME, "failed to write to file");
        }
    }

    pub fn list(&self) -> Vec<Task> {
        let tasks_string = self.tasks_str();

        specifics_of_list(&tasks_string)
    }

    // IMPORTANT: not an error, for example: `Task` object should no longer be validated
    pub fn add(&self, add_task: &Task) -> Vec<Task> {
        let tasks_string = self.tasks_str();

        return specifics_of_add(&tasks_string, add_task);
    }

    // IMPORTANT: not an error, for example: `Task` object should no longer be validated
    pub fn update(&self, id: i32, update_task: &Task) -> Vec<Task> {
        let tasks_string = self.tasks_str();

        specifics_of_update(tasks_string, id, update_task)
    }

    pub fn delete(&self, id: i32) -> Vec<Task> {
        let tasks_string = self.tasks_str();

        specifics_of_delete(tasks_string, id)
    }
}

#[cfg(test)]
pub mod tests {
    // TODO: a single core test
    // // $ cargo test -- --test-threads=1
    use crate::{
        file::files::{
            specifics_of_add, specifics_of_delete, specifics_of_list, specifics_of_update,
        },
        task::task::Task,
        task::task_test::{setup_task, setup_task_status},
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
        let got = vec![setup_task_status(1, "test update fail", "todo")];
        let want = specifics_of_update(
            "[\n  {\n    \"id\": 1,\n    \"description\": \"test update fail\",\n    \"status\": \"in-progress\",\n    \"created_at\": \"2025-10-13T14:07:06.072493+07:00\",\n    \"updated_at\": \"2025-10-13T19:07:06.072493+07:00\"\n  }\n]".to_owned(),
            1,
            &setup_task_status(1, "test update fail", "done"),
        );
        assert_ne!(got, want);
    }

    #[test]
    fn test_json_update_success() {
        let got = vec![setup_task_status(1, "test update success", "in-progress")];
        let want = specifics_of_update(
            "[\n  {\n    \"id\": 1,\n    \"description\": \"test update success\",\n    \"status\": \"in-progress\",\n    \"created_at\": \"2025-10-13T14:07:06.072493+07:00\",\n    \"updated_at\": \"2025-10-13T19:07:06.072493+07:00\"\n  }\n]".to_owned(),
            1,
            &setup_task_status(1, "test update success", "in-progress"),
        );
        assert_eq!(got, want);
    }

    #[test]
    #[should_panic]
    fn test_json_delete_fail() {
        specifics_of_delete(
            "[\n  {\n    \"id\": 1,\n    \"description\": \"test delete fail\",\n    \"status\": \"in-progress\",\n    \"created_at\": \"2025-10-13T14:07:06.072493+07:00\",\n    \"updated_at\": \"2025-10-13T19:07:06.072493+07:00\"\n  }\n]".to_owned(),
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
