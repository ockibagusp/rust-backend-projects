use crate::domain::task::Task;

// TDD
// ✅ ❔ ❌
// 3.2. buat trait ListRepositoryTrait dengan method new, index, todo, in_progress, done ✅
// => 3.2. create the ListRepositoryTrait trait with methods new, index, todo, in_progress, done
pub trait ListRepositoryTrait {
    // fn new() -> Self
    // where
    //     Self: Sized;
    fn all(&self) -> Vec<Task>;
    fn todo(&self) -> Vec<Task>;
    fn in_progress(&self) -> Vec<Task>;
    fn done(&self) -> Vec<Task>;
}

pub trait ListUseCaseTrait {
    fn list_of_all(&self) -> Vec<Task>;
    fn todo(&self) -> Vec<Task>;
    fn in_progress(&self) -> Vec<Task>;
    fn done(&self) -> Vec<Task>;
}

pub struct ListUseCase {
    pub repository: Box<dyn ListRepositoryTrait>,
}

// TDD
// ✅ ❔ ❌
// 3.3. implementasikan trait ListRepositoryTrait untuk struct List ✅
// => 3.3. implement the ListRepositoryTrait trait for the List struct
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
impl ListUseCaseTrait for ListUseCase {
    // Get all tasks list
    fn list_of_all(&self) -> Vec<Task> {
        return self.repository.all();
    }

    // Mark task to 'todo' status
    fn todo(&self) -> Vec<Task> {
        return self.repository.todo();
    }

    // Mark task to 'in-progress' status
    fn in_progress(&self) -> Vec<Task> {
        return self.repository.in_progress();
    }

    // Mark task to 'done' status
    fn done(&self) -> Vec<Task> {
        return self.repository.done();
    }
}
