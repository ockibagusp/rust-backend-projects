use crate::task::task::{Task, VALID_STATUSES};
use crate::task::task_manager::{TaskManager, TaskManagerTrait};
use std::io::Error;

use mockall::automock;

fn error_invalid_input(message: &str) -> Error {
    return Error::new(
        std::io::ErrorKind::InvalidInput,
        format!("error: {}", message),
    );
}

fn error_not_found_input(message: &str) -> Error {
    Error::new(std::io::ErrorKind::NotFound, format!("error: {}", message))
}

// TDD
// ✅ ❔ ❌
// 3.1. buatlah struktur data List dengan objek task_manager ✅
// => 3.1. create the List data structure with an object of task_manager
// ------------------------------------------------
// 1. buat field `task_manager` bertipe objek TaskManager ✅
// => 1. make the `task_manager` field with a type of TaskManager object
pub struct List {
    pub task_manager: TaskManager,
}

// TDD
// ✅ ❔ ❌
// 3.2. buat trait ListTrait dengan method new, index, todo, in_progress, done ✅
// => 3.2. create the ListTrait trait with methods new, index, todo, in_progress, done
#[automock]
pub trait ListTrait {
    fn new(file_name: &'static str) -> Self;
    fn index(&self) -> Vec<Task>;
    fn todo(&self) -> Vec<Task>;
    fn in_progress(&self) -> Vec<Task>;
    fn done(&self) -> Vec<Task>;
}

// TDD
// ✅ ❔ ❌
// 3.3. implementasikan trait ListTrait untuk struct List ✅
// => 3.3. implement the ListTrait trait for the List struct
// ------------------------------------------------
// 1. method `new` untuk inisialisasi List ✅
// => 1. the `new` method for initialize the List
// 2. method `index` atau `list` untuk mendapatkan semua task ✅
// => 2. the `index` or `list` method to get all tasks
// 3. method `todo` untuk mendapatkan task dengan status 'todo' ✅
// => 3. the `todo` method to get tasks with 'todo' status
// 4. method `in_progress` untuk mendapatkan task dengan status 'in-progress' ✅
// => 4. the `in_progress` method to get tasks with 'in-progress' status
// 5. method `done` untuk mendapatkan task dengan status 'done' ✅
// => 5. the `done` method to get tasks with 'done' status
impl ListTrait for List {
    fn new(file_name: &'static str) -> Self {
        let _task_manager = TaskManager::new(file_name);
        List {
            task_manager: _task_manager,
        }
    }
    // Get all task list
    fn index(&self) -> Vec<Task> {
        return self.task_manager.list();
    }
    // Mark task to 'todo' status
    fn todo(&self) -> Vec<Task> {
        let task_lists = self
            .task_manager
            .list
            .iter()
            .filter(|&task| task.status == VALID_STATUSES[0])
            .cloned()
            .collect();
        task_lists
    }
    // Mark task to 'in-progress' status
    fn in_progress(&self) -> Vec<Task> {
        let task_lists = self
            .task_manager
            .list
            .iter()
            .filter(|&task| task.status == VALID_STATUSES[1])
            .cloned()
            .collect();
        task_lists
    }
    // Mark task to 'done' status
    fn done(&self) -> Vec<Task> {
        let task_lists = self
            .task_manager
            .list
            .iter()
            .filter(|&task| task.status == VALID_STATUSES[2])
            .cloned()
            .collect();
        task_lists
    }
}
