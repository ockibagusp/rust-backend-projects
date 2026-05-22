use crate::file::files::File;
use crate::task::task::{Task, VALID_STATUSES};

use dotenv::dotenv;
use mockall::automock;
use std::env;

// TDD
// ✅ ❔ ❌
// 3.1. buatlah struktur data ListManager dengan objek file ✅
// => 3.1. create the ListManager data structure with an object of file
// ------------------------------------------------
// 1. buat field `list` bertipe objek File ✅
// => 1. make the `list` field with a type of File object
pub struct ListManager {
    pub list: Vec<Task>,
}

// TDD
// ✅ ❔ ❌
// 3.2. buat trait ListManagerTrait dengan method new, index, todo, in_progress, done ✅
// => 3.2. create the ListManagerTrait trait with methods new, index, todo, in_progress, done
#[automock]
pub trait ListManagerTrait {
    fn new() -> Self;
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
impl ListManagerTrait for ListManager {
    fn new() -> Self {
        let file = File::new();
        ListManager { list: file.list() }
    }
    // Get all tasks list
    fn index(&self) -> Vec<Task> {
        return self.list.clone();
    }
    // Mark task to 'todo' status
    fn todo(&self) -> Vec<Task> {
        return get_status_tasks(&self.list, VALID_STATUSES[0]);
    }
    // Mark task to 'in-progress' status
    fn in_progress(&self) -> Vec<Task> {
        return get_status_tasks(&self.list, VALID_STATUSES[1]);
    }
    // Mark task to 'done' status
    fn done(&self) -> Vec<Task> {
        return get_status_tasks(&self.list, VALID_STATUSES[2]);
    }
}

fn get_status_tasks(list: &Vec<Task>, status: &str) -> Vec<Task> {
    return list
        .iter()
        .filter(|&task| task.status == status)
        .cloned()
        .collect();
}
