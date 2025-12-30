use crate::task::task::Task;
use crate::task::task_manager::{TaskManager, TaskManagerTrait};

use mockall::automock;

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
// 3.2. buat trait ListTrait dengan method new dan index ✅
#[automock]
pub trait ListTrait {
    fn new(file_name: &'static str) -> Self;
    fn index(&self) -> Vec<Task>;
}

// TDD
// ✅ ❔ ❌
// 3.3. implementasikan trait ListTrait untuk struct List ✅
// => 3.3. implement the ListTrait trait for the List struct
// ------------------------------------------------
// 1. fungsi `new` untuk inisialisasi List ✅
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
}
