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
        OpenTaskExtensions::list_for_title_str(&self.tasks_to_specify, "All Lists")
    }

    fn todo(&self) -> String {
        OpenTaskExtensions::list_for_title_str(&self.tasks_to_specify, "Todo Lists")
    }

    fn in_progress(&self) -> String {
        OpenTaskExtensions::list_for_title_str(&self.tasks_to_specify, "In Progress Lists")
    }

    fn done(&self) -> String {
        OpenTaskExtensions::list_for_title_str(&self.tasks_to_specify, "Done Lists")
    }
}

pub struct OpenTask;
impl OpenTask {
    pub fn add(task: &Task) -> String {
        return OpenTaskExtensions::task_title_str("Added task", task);
    }

    pub fn update(task: &Task) -> String {
        return OpenTaskExtensions::task_title_str("Updated task", task);
    }

    pub fn delete() -> String {
        return String::from("Deleted task");
    }
}

pub struct OpenMarkTask;
impl OpenMarkTask {
    pub fn in_progress(task: &Task) -> String {
        return OpenTaskExtensions::task_title_str("Mark as \"in progress\"", task);
    }

    pub fn done(task: &Task) -> String {
        return OpenTaskExtensions::task_title_str("Mark as \"done\"", task);
    }
}

pub struct OpenTaskExtensions;
impl OpenTaskExtensions {
    fn list_for_title_str(list: &Vec<Task>, title: &str) -> String {
        let tasks_str: String;
        if list.is_empty() {
            tasks_str = String::from("No lists found.");
        } else {
            let mut list_str: Vec<String> = vec![];
            for task in list {
                let task_str = OpenTaskExtensions::task_str(task);
                list_str.push(task_str);
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

    fn task_str(task: &Task) -> String {
        return format!(
            "ID: {}\n----- Description: {}\n----- Status     : {}\n----- Created At : {}\n----- Updated At : {}",
            task.id,
            task.description,
            task.status,
            OpenTaskExtensions::get_datetimes_to_string(&task.created_at),
            OpenTaskExtensions::get_datetimes_to_string(&task.updated_at)
        );
    }

    fn task_title_str(title: &str, task: &Task) -> String {
        return format!(
            "{}\n{}\n{}",
            String::from(title),
            String::from("------------------"),
            OpenTaskExtensions::task_str(&task)
        );
    }

    fn get_datetimes_to_string(date_time: &DateTime<FixedOffset>) -> String {
        return date_time.format("%d-%m-%Y %H:%M:%S").to_string();
    }
}
