use crate::domain::task::Task;
use chrono::{DateTime, FixedOffset};

pub struct OpenTasks {
    pub tasks_to_specify: Vec<Task>,
}

pub trait OpenListTasksTrait {
    fn new() -> Self;
    fn set_tasks(&mut self, tasks: Vec<Task>);

    fn list(&self) -> String;
    fn todo(&self) -> String;
    fn in_progress(&self) -> String;
    fn done(&self) -> String;

    fn list_for_title_str(&self, title: &str) -> String;
}

impl OpenListTasksTrait for OpenTasks {
    fn new() -> Self {
        OpenTasks {
            tasks_to_specify: vec![],
        }
    }

    // IMPORTANT: This method is used to set the tasks to be printed
    // in the `list`, `todo`, `in_progress`, and `done` methods.
    fn set_tasks(&mut self, tasks: Vec<Task>) {
        self.tasks_to_specify = tasks;
    }

    fn list(&self) -> String {
        self.list_for_title_str("All Lists")
    }

    fn todo(&self) -> String {
        self.list_for_title_str("Todo Lists")
    }

    fn in_progress(&self) -> String {
        self.list_for_title_str("In Progress Lists")
    }

    fn done(&self) -> String {
        self.list_for_title_str("Done Lists")
    }

    fn list_for_title_str(&self, title: &str) -> String {
        let tasks_str: String;
        if self.tasks_to_specify.is_empty() {
            tasks_str = String::from("No lists found.");
        } else {
            let mut list_str: Vec<String> = vec![];
            for task in &self.tasks_to_specify {
                list_str.push(open_task_str(task));
            }
            tasks_str = list_str.join("\n");
        }

        return format!(
            "{}\n{}\n{}\n{}",
            String::from(title),
            String::from("------------------"),
            tasks_str,
            String::from("++++++++++++++++++")
        );
    }
}

pub struct OpenTask;
impl OpenTask {
    pub fn add(task: &Task) -> String {
        return OpenTask::task_title_str("Added task", task);
    }

    pub fn update(task: &Task) -> String {
        return OpenTask::task_title_str("Updated task", task);
    }

    pub fn delete() -> String {
        return String::from("Deleted task");
    }

    fn task_title_str(title: &str, task: &Task) -> String {
        return format!(
            "{}\n{}\n{}",
            String::from(title),
            String::from("------------------"),
            open_task_str(&task)
        );
    }
}

pub struct OpenMarkTask;
impl OpenMarkTask {
    pub fn in_progress(task: &Task) -> String {
        return OpenMarkTask::task_title_str("Mark as \"in progress\"", task);
    }

    pub fn done(task: &Task) -> String {
        return OpenMarkTask::task_title_str("Mark as \"done\"", task);
    }

    fn task_title_str(title: &str, task: &Task) -> String {
        return format!(
            "{}\n{}\n{}",
            String::from(title),
            String::from("------------------"),
            open_task_str(&task)
        );
    }
}

pub fn open_task_str(task: &Task) -> String {
    return format!(
        "ID: {}\n----- Description: {}\n----- Status     : {}\n----- Created At : {}\n----- Updated At : {}",
        task.id,
        task.description,
        task.status,
        get_datetimes_to_string(&task.created_at),
        get_datetimes_to_string(&task.updated_at)
    );
}

pub fn open_task_title_str(title: &str, task: Task) -> String {
    return format!(
        "{}\n{}\n{}",
        String::from(title),
        String::from("------------------"),
        open_task_str(&task)
    );
}

fn get_datetimes_to_string(date_time: &DateTime<FixedOffset>) -> String {
    return date_time.format("%d-%m-%Y %H:%M:%S").to_string();
}
