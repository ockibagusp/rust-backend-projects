use crate::application::list_use_cases::ListUseCaseTrait;
use crate::domain::task::Task;

pub struct CmdListHandler {
    // Presentation depends directly on the Application use case
    pub use_case: Box<dyn ListUseCaseTrait>,
}

impl CmdListHandler {
    pub fn new(use_case: Box<dyn ListUseCaseTrait>) -> Self {
        Self { use_case }
    }

    pub fn handle_list_of_all_tasks(&self) -> Vec<Task> {
        return self.use_case.list_of_all();
    }

    pub fn handle_list_of_todo_tasks(&self) -> Vec<Task> {
        return self.use_case.todo();
    }

    pub fn handle_list_of_in_progress_tasks(&self) -> Vec<Task> {
        return self.use_case.in_progress();
    }

    pub fn handle_list_of_done_tasks(&self) -> Vec<Task> {
        return self.use_case.done();
    }
}
