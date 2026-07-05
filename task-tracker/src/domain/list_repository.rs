use crate::domain::task::Task;

// TDD
// ✅ ❔ ❌
// 3.2. buat trait ListRepository dengan method new, index, todo, in_progress, done ✅
// => 3.2. create the ListRepository trait with methods new, index, todo, in_progress, done
pub trait ListRepository {
    // fn new() -> Self
    // where
    //     Self: Sized;
    fn all(&self) -> Vec<Task>;
    fn todo(&self) -> Vec<Task>;
    fn in_progress(&self) -> Vec<Task>;
    fn done(&self) -> Vec<Task>;
}
