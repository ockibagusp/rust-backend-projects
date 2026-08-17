use crate::domain::task::Task;

// TDD
// ✅ ❔ ❌
// 2.1. buat trait ListRepository dengan method all, todo, in_progress, done ✅
// => 2.2. create the ListRepository trait with methodd all, todo, in_progress, done
pub trait ListRepository {
    // fn new() -> Self
    // where
    //     Self: Sized;
    fn all(&self) -> Vec<Task>;
    fn todo(&self) -> Vec<Task>;
    fn in_progress(&self) -> Vec<Task>;
    fn done(&self) -> Vec<Task>;
}
