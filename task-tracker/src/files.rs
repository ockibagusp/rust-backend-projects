use core::result::Result;
use std::fs;
use std::fs::File as std_file;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::Read;

use crate::task::Task;

// https://medium.com/@aleksej.gudkov/rust-write-to-file-example-a-practical-guide-51c24695aa80

#[allow(dead_code)]
pub struct File<'a> {
    file_name: &'a str,
}

#[allow(dead_code)]
impl<'a> File<'a> {
    pub fn new(file_name: &'a str) -> Self {
        File { file_name }
    }

    fn name(&self) -> &str {
        &self.file_name
    }

    fn list(&self) -> Result<Vec<Task>, Error> {
        let mut tasks_file = std_file::open(&self.name())?;
        let mut tasks_string = String::new();
        let err = tasks_file.read_to_string(&mut tasks_string);
        if err.is_err() {
            return Err(err.err().unwrap());
        }

        // You probably want to deserialize tasks_string into Vec<Task>
        let tasks: Vec<Task> = serde_json::from_str(&tasks_string).unwrap_or_default();
        Ok(tasks)
    }

    fn add(&self, update_task: &Task) -> Result<bool, Error> {
        let mut tasks_file = OpenOptions::new()
            .write(true)
            .create_new(true) // Will error if file already exists
            .open(&self.name())
            .or_else(|_| OpenOptions::new().read(true).open(&self.name()))
            .unwrap();

        let mut tasks_string = String::new();
        let _ = tasks_file.read_to_string(&mut tasks_string);

        // You probably want to deserialize tasks_string into Vec<Task>
        let mut tasks: Vec<Task> = serde_json::from_str(&tasks_string).unwrap_or_default();
        tasks.push(update_task.clone());

        let json_string = serde_json::to_string_pretty(&tasks).unwrap();
        if fs::write(&self.name(), json_string).is_err() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to write to file",
            ));
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::{File, Task, fs};
    use chrono::DateTime;
    use std::sync::Arc;
    use std::sync::Mutex;

    fn test_file_name() -> &'static str {
        "test-task-cli.json"
    }

    fn test_remove_file(file_name: &str) {
        println!("Removing existing test: {}...\n", file_name);

        // Remove the file
        match fs::remove_file(file_name) {
            Ok(_) => println!("File removed successfully."),
            Err(e) => eprintln!("Error removing file: {}", e),
        }
    }

    fn setup_tasks(id: i32, desciption: &str) -> Task {
        let _created_at = DateTime::parse_from_str(
            "2025-10-13 14:07:06.072493 +07:00",
            "%Y-%m-%d %H:%M:%S%.f %z",
        )
        .expect("Failed to parse created_at");

        let _updated_at = DateTime::parse_from_str(
            "2025-10-13 19:07:06.072493 +07:00",
            "%Y-%m-%d %H:%M:%S%.f %z",
        )
        .expect("Failed to parse updated_at");

        Task {
            id: id,
            description: desciption.to_string(),
            status: "todo".to_string(),
            created_at: _created_at,
            updated_at: _updated_at,
        }
    }

    #[test]
    fn test_new_file() {
        let new_file = "new-file".to_string() + "-" + test_file_name();

        let update_task = &setup_tasks(1, "Buy cook dinner");
        let json_string = serde_json::to_string_pretty(&vec![update_task.clone()]).unwrap();

        // Create File instance
        let test_file = File::new(&new_file);
        assert_eq!(test_file.name(), test_file.file_name);

        assert_eq!(
            json_string,
            "[\n  {\n    \"id\": 1,\n    \"description\": \"Buy cook dinner\",\n    \"status\": \"todo\",\n    \"created_at\": \"2025-10-13T14:07:06.072493+07:00\",\n    \"updated_at\": \"2025-10-13T19:07:06.072493+07:00\"\n  }\n]".to_string(),
        );

        test_remove_file(&new_file);
    }

    #[test]
    fn test_add_file() {
        let add_file = "add-file".to_string() + "-" + test_file_name();

        let test_file = File::new(&add_file);

        let mut update_task = setup_tasks(2, "Buy cook dinner");

        let updated = test_file.add(&update_task).unwrap();
        assert_eq!(updated, true);

        let mut tasks = test_file.list().unwrap();
        assert_eq!(tasks.len(), 1);

        update_task = setup_tasks(3, "Buy groceries");
        let updated = test_file.add(&update_task).unwrap();
        assert_eq!(updated, true);
        tasks = test_file.list().unwrap();
        assert_eq!(tasks.len(), 2);

        test_remove_file(&add_file);
    }

    #[test]
    fn test_file_list() {
        let file_list = "file-list".to_string() + "-" + test_file_name();

        let list_task = setup_tasks(1, "Buy cook dinner");
        let annother_list_task = setup_tasks(2, "Buy groceries");

        let tasks = Arc::new(Mutex::new(Vec::<Task>::new()));
        let json_string;
        {
            let mut data = tasks.lock().unwrap();
            data.extend(vec![list_task, annother_list_task]);
            json_string = serde_json::to_string_pretty(&*data).unwrap();
            fs::write(&file_list, json_string.clone()).unwrap();
        }

        let test_file = File::new(&file_list);

        let tasks = test_file.list().unwrap();
        assert_eq!(tasks.len(), 2);

        test_remove_file(&file_list);
    }
}
