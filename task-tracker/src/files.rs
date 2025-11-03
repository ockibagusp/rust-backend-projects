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

    fn open_options(&self) -> String {
        let mut tasks_file = OpenOptions::new()
            .write(true)
            .create_new(true) // Will error if file already exists
            .open(self.name())
            .or_else(|_| OpenOptions::new().read(true).open(self.name()))
            .unwrap();

        let mut tasks_string = String::new();
        let _ = tasks_file.read_to_string(&mut tasks_string);
        tasks_string
    }

    pub fn list(&self) -> Result<Vec<Task>, Error> {
        let mut tasks_file = std_file::open(self.name())?;
        let mut tasks_string = String::new();
        let err = tasks_file.read_to_string(&mut tasks_string);
        if err.is_err() {
            return Err(err.err().unwrap());
        }

        // You probably want to deserialize tasks_string into Vec<Task>
        let tasks: Vec<Task> = serde_json::from_str(&tasks_string).unwrap_or_default();
        Ok(tasks)
    }

    pub fn add(&self, add_task: Task) -> Result<(), Error> {
        if !add_task.is_validation() {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "All task cannot be empty",
            ));
        }

        let tasks_string = self.open_options();

        // You probably want to deserialize tasks_string into Vec<Task>
        let mut tasks: Vec<Task> = serde_json::from_str(&tasks_string).unwrap_or_default();
        tasks.push(add_task.clone());

        let json_string = serde_json::to_string_pretty(&tasks).unwrap();
        if fs::write(self.name(), json_string).is_err() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to write to file",
            ));
        }

        Ok(())
    }

    pub fn delete(&self, id: i32) -> Result<(), Error> {
        let tasks_string = self.open_options();

        // You probably want to deserialize tasks_string into Vec<Task>
        let mut tasks: Vec<Task> = serde_json::from_str(&tasks_string).unwrap_or_default();
        if tasks.get(id as usize - 1) == None {
            return Err(Error::new(std::io::ErrorKind::NotFound, "Task not found"));
        }

        tasks.remove(id as usize - 1);

        let json_string = serde_json::to_string_pretty(&tasks).unwrap();
        if fs::write(self.name(), json_string).is_err() {
            return Err(Error::new(
                std::io::ErrorKind::Other,
                "Failed to write to file",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{File, Task, fs};
    use chrono::DateTime;
    use std::sync::Arc;
    use std::sync::Mutex;

    fn test_file_name(name: Option<&str>) -> String {
        match name {
            None => "test-task-cli.json".to_string(),
            Some(n) => format!("{}-test-task-cli.json", n),
        }
    }

    fn test_remove_file(file_name: &str) {
        println!("Removing existing test: {}...\n", file_name);

        // Remove the file
        match fs::remove_file(file_name) {
            Ok(_) => println!("File removed successfully."),
            Err(e) => eprintln!("Error removing file: {}", e),
        }
    }

    fn setup_task(id: i32, desciption: &str) -> Task {
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
        let new_file = Some("new-file");
        let binding = test_file_name(new_file);

        let update_task = &setup_task(1, "Buy cook dinner");
        let json_string = serde_json::to_string_pretty(&vec![update_task.clone()]).unwrap();

        // Create File instance
        let test_file = File::new(&binding);
        assert_eq!(test_file.name(), test_file.file_name);

        assert_eq!(
            json_string,
            "[\n  {\n    \"id\": 1,\n    \"description\": \"Buy cook dinner\",\n    \"status\": \"todo\",\n    \"created_at\": \"2025-10-13T14:07:06.072493+07:00\",\n    \"updated_at\": \"2025-10-13T19:07:06.072493+07:00\"\n  }\n]".to_string(),
        );

        test_remove_file(test_file.name());
    }

    #[test]
    fn test_add_file_not_found() {
        let add_file_not_found = Some("add-file_not_found");
        let binding = test_file_name(add_file_not_found);

        // Create File instance
        let test_file = File::new(&binding);

        let added = test_file.add(setup_task(-1, "fail"));
        assert!(added.is_err_and(|e| e.kind() == std::io::ErrorKind::InvalidInput));

        test_remove_file(test_file.name());
    }

    #[test]
    fn test_file_list_no_such_file() {
        let file_list_n = test_file_name(Some("file-list-no-such"));

        let tasks = Arc::new(Mutex::new(Vec::<Task>::new()));
        let json_string;
        {
            let mut data = tasks.lock().unwrap();
            data.extend(vec![]);
            json_string = serde_json::to_string_pretty(&*data).unwrap();
            fs::write(&file_list_n, json_string.clone()).unwrap();
        }

        // Create File fail instance
        let file_list_fail = &file_list_n[0..12];

        // Create File instance
        let test_file = File::new(&file_list_fail);

        let tasks = test_file.list();
        assert!(tasks.is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound));

        test_remove_file(&file_list_n);
    }

    #[test]
    fn test_file_list() {
        let file_list = Some("file-list");
        let binding = test_file_name(file_list);

        let list_task = setup_task(1, "Buy cook dinner");
        let annother_list_task = setup_task(2, "Buy groceries");

        let tasks = Arc::new(Mutex::new(Vec::<Task>::new()));
        let json_string;
        {
            let mut data = tasks.lock().unwrap();
            data.extend(vec![list_task, annother_list_task]);
            json_string = serde_json::to_string_pretty(&*data).unwrap();
            fs::write(&binding, json_string.clone()).unwrap();
        }

        // Create File instance
        let test_file = File::new(&binding);

        {
            let tasks = test_file.list().unwrap();
            assert_eq!(tasks.len(), 2);
        }

        test_remove_file(test_file.name());
    }

    #[test]
    fn test_add_file() {
        let add_file = Some("add-file");
        let binding = test_file_name(add_file);

        // Create File instance
        let test_file = File::new(&binding);

        let mut add_task = setup_task(2, "Buy cook dinner");
        let added = test_file.add(add_task).unwrap();
        assert_eq!(added, ());

        let mut tasks = test_file.list().unwrap();
        assert_eq!(tasks.len(), 1);

        add_task = setup_task(3, "Buy groceries");
        let updated = test_file.add(add_task).unwrap();
        assert_eq!(updated, ());

        tasks = test_file.list().unwrap();
        assert_eq!(tasks.len(), 2);

        test_remove_file(test_file.name());
    }

    #[test]
    fn test_delete_file_not_found() {
        let delete_file_not_found = Some("delete-file-not-found");
        let binding = test_file_name(delete_file_not_found);

        let lists_task = vec![
            setup_task(1, "test 1"),
            setup_task(2, "test 2"),
            setup_task(3, "test 3"),
        ];

        let tasks = Arc::new(Mutex::new(Vec::<Task>::new()));
        let json_string;
        {
            let mut data = tasks.lock().unwrap();
            data.extend(lists_task);
            json_string = serde_json::to_string_pretty(&*data).unwrap();
            fs::write(&binding, json_string.clone()).unwrap();
        }

        // Create File instance
        let test_file = File::new(&binding);

        // Attempt to delete a non-existing task: 4
        let deleted = test_file.delete(4);
        assert!(deleted.is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound));

        let tasks = test_file.list().unwrap();
        assert_eq!(tasks.len(), 3);

        test_remove_file(test_file.name());
    }

    #[test]
    fn test_delete_file() {
        let delete_file = Some("delete-file");
        let binding = test_file_name(delete_file);

        let lists_task = vec![
            setup_task(1, "test 1"),
            setup_task(2, "test 2"),
            setup_task(3, "test 3"),
        ];

        let tasks = Arc::new(Mutex::new(Vec::<Task>::new()));
        let json_string;
        {
            let mut data = tasks.lock().unwrap();
            data.extend(lists_task);
            json_string = serde_json::to_string_pretty(&*data).unwrap();
            fs::write(&binding, json_string.clone()).unwrap();
        }

        // Create File instance
        let test_file = File::new(&binding);

        let deleted = test_file.delete(2).unwrap();
        assert_eq!(deleted, ());

        let mut tasks = test_file.list().unwrap();
        assert_eq!(tasks.len(), 2);

        tasks = test_file.list().unwrap();
        assert_eq!(tasks.len(), 2);

        test_remove_file(test_file.name());
    }
}
