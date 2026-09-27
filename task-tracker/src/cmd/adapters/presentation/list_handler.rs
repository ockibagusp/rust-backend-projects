use crate::cmd::{application::use_cases::list::*, domain::models::Task};

pub struct CmdListHandler<R: ListRepository> {
    // Presentation depends directly on the Application use case
    pub use_case: ListUseCase<R>,
}

impl<R: ListRepository> CmdListHandler<R> {
    pub fn new(use_case: ListUseCase<R>) -> Self {
        Self { use_case }
    }

    pub fn all(&self) -> Vec<Task> {
        return self.use_case.all();
    }

    pub fn todo(&self) -> Vec<Task> {
        return self.use_case.todo();
    }

    pub fn in_progress(&self) -> Vec<Task> {
        return self.use_case.in_progress();
    }

    pub fn done(&self) -> Vec<Task> {
        return self.use_case.done();
    }
}
