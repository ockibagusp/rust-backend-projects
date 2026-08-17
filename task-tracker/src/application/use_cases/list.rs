use crate::application::ports::list_repository::ListRepository;
use crate::domain::task::Task;

pub struct ListUseCase {
    pub repository: Box<dyn ListRepository>,
}

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
impl ListUseCase {
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
