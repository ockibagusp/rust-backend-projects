use crate::cmd::{domain::models::Task, infrastructure::storages::storage::StorageTrait};
// TDD
// ✅ ❔ ❌
// 3.1. implementasikan trait ListRepository untuk struct List ✅
// => 1.3. implement the ListRepository trait for the List struct
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

// TDD
// ✅ ❔ ❌
// 2.1. buat trait ListRepository dengan method all, todo, in_progress, done ✅
// => 2.2. create the ListRepository trait with methodd all, todo, in_progress, done
pub trait ListRepository {
    fn new(storage: Box<dyn StorageTrait>) -> Self;
    fn all(&self) -> Vec<Task>;
    fn todo(&self) -> Vec<Task>;
    fn in_progress(&self) -> Vec<Task>;
    fn done(&self) -> Vec<Task>;
}

pub struct ListUseCase<R: ListRepository> {
    pub repository: R,
}

impl<R: ListRepository> ListUseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    // Get all tasks list
    pub fn all(&self) -> Vec<Task> {
        return self.repository.all();
    }

    // Mark task to 'todo' status
    pub fn todo(&self) -> Vec<Task> {
        return self.repository.todo();
    }

    // Mark task to 'in-progress' status
    pub fn in_progress(&self) -> Vec<Task> {
        return self.repository.in_progress();
    }

    // Mark task to 'done' status
    pub fn done(&self) -> Vec<Task> {
        return self.repository.done();
    }
}
