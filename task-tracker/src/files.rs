use crate::task::{Task, TaskTrait};
use chrono::DateTime;
use core::result::Result;
use std::fs;
use std::fs::{File as std_file, OpenOptions};
use std::io::{Error, Read};

fn panic_invalid_input(message: &str) -> ! {
    panic!(
        "Error: {}",
        Error::new(std::io::ErrorKind::InvalidInput, message,)
    )
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct File {
    pub file_name: &'static str,
}

#[allow(dead_code)]
impl File {
    pub fn new(file_name: &'static str) -> Self {
        if !fs::metadata(file_name).is_ok() {
            let _ = OpenOptions::new()
                .write(true)
                .create_new(true) // Will error if file already exists
                .open(file_name)
                .unwrap();

            Self::json_string(file_name, vec![]);
        }
        Self { file_name }
    }

    fn name(&self) -> &str {
        &self.file_name
    }

    fn open_options(&self) -> std_file {
        OpenOptions::new().read(true).open(self.name()).unwrap()
    }

    fn tasks_str(&self) -> String {
        let mut tasks_file = &self.open_options();

        let mut tasks_string = String::new();
        let _ = tasks_file.read_to_string(&mut tasks_string);
        tasks_string
    }

    fn json_string(file_name: &'static str, tasks: Vec<Task>) -> () {
        let json_string = serde_json::to_string_pretty(&tasks).unwrap();
        if fs::write(file_name, json_string).is_err() {
            panic_invalid_input("Failed to write to file");
        }
    }

    pub fn list(&self) -> Vec<Task> {
        let tasks_string = self.tasks_str();

        // You probably want to deserialize tasks_string into Vec<Task>
        let tasks: Vec<Task> = serde_json::from_str(&tasks_string).unwrap();
        tasks
    }

    pub fn add(&self, add_task: Task) -> Result<(), Error> {
        let is_valid = add_task.is_validation();
        if is_valid.is_err() {
            return Err(is_valid.unwrap_err());
        }

        let tasks_string = self.tasks_str();

        // You probably want to deserialize tasks_string into Vec<Task>
        let mut tasks: Vec<Task> = serde_json::from_str(&tasks_string).unwrap();
        tasks.push(add_task.clone());

        Self::json_string(&self.file_name, tasks);

        Ok(())
    }

    pub fn update(&self, id: i32, update_task: Task) -> Result<(), Error> {
        if id != update_task.id {
            panic_invalid_input("Failed to update task: ID mismatch");
        }

        let tasks_string = self.tasks_str();

        // You probably want to deserialize tasks_string into Vec<Task>
        let mut tasks: Vec<Task> = serde_json::from_str(&tasks_string).unwrap_or_default();
        for task in tasks.iter_mut() {
            if task.id == id {
                *task = update_task.clone();
                task.updated_at =
                    DateTime::parse_from_str("1970-01-01 00:00:00 +00:00", "%Y-%m-%d %H:%M:%S %z")
                        .unwrap()
                        .into();
                break;
            }
        }

        Self::json_string(&self.file_name, tasks);

        Ok(())
    }

    pub fn delete(&self, id: i32) -> Result<(), Error> {
        let tasks_string = self.tasks_str();

        // You probably want to deserialize tasks_string into Vec<Task>
        let mut tasks: Vec<Task> = serde_json::from_str(&tasks_string).unwrap_or_default();
        if tasks.get(id as usize - 1) == None {
            return Err(Error::new(std::io::ErrorKind::NotFound, "Task not found"));
        }

        tasks.remove(id as usize - 1);

        Self::json_string(&self.file_name, tasks);

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::{File, Task, fs};
    use crate::task::VALID_STATUSES;
    use chrono::DateTime;
    use std::sync::{Arc, Mutex};

    fn test_start_file(file_name: Option<&str>) -> &str {
        let name = test_file_name(file_name);
        if fs::metadata(name).is_ok() {
            test_remove_file(name);
        }
        name
    }

    fn test_file_name(name: Option<&str>) -> &str {
        match name {
            None => "test-task-cli.json",
            Some(n) => {
                let new_name = format!("{}-test-task-cli.json", n);
                return new_name.leak();
            }
        }
    }

    fn test_remove_file(file_name: &str) {
        match fs::remove_file(file_name) {
            Ok(_) => println!("Removing existing test: {}...\n", file_name),
            Err(e) => eprintln!("Error removing file: {}", e),
        }
    }

    pub fn setup_task(id: i32, desciption: &str) -> Task {
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
            status: String::from(VALID_STATUSES[0]),
            created_at: _created_at,
            updated_at: _updated_at,
        }
    }

    #[test]
    fn test_new_file() {
        let new_file = test_start_file(Some("new-file"));

        let update_task = &setup_task(1, "Buy cook dinner");
        let json_string = serde_json::to_string_pretty(&vec![update_task.clone()]).unwrap();

        // Create File instance
        let test_file = File::new(new_file);
        assert_eq!(test_file.name(), test_file.file_name);

        assert_eq!(
            json_string,
            "[\n  {\n    \"id\": 1,\n    \"description\": \"Buy cook dinner\",\n    \"status\": \"todo\",\n    \"created_at\": \"2025-10-13T14:07:06.072493+07:00\",\n    \"updated_at\": \"2025-10-13T19:07:06.072493+07:00\"\n  }\n]".to_string(),
        );

        test_remove_file(test_file.name());
    }

    #[test]
    fn test_file_list() {
        let new_file = test_start_file(Some("file-list"));

        let list_task = setup_task(1, "Buy cook dinner");
        let annother_list_task = setup_task(2, "Buy groceries");

        let tasks = Arc::new(Mutex::new(Vec::<Task>::new()));
        let json_string;
        {
            let mut data = tasks.lock().unwrap();
            data.extend(vec![list_task, annother_list_task]);
            json_string = serde_json::to_string_pretty(&*data).unwrap();
            fs::write(new_file, json_string.clone()).unwrap();
        }

        // Create File instance
        let test_file = File::new(new_file);

        {
            let tasks = test_file.list();
            assert_eq!(tasks.len(), 2);
        }

        test_remove_file(test_file.name());
    }

    #[test]
    fn test_add_file_not_found() {
        let new_file = test_start_file(Some("add-file-not-found"));

        // Create File instance
        let test_file = File::new(new_file);

        let added = test_file.add(setup_task(-1, "fail"));
        assert!(added.is_err_and(|e| e.kind() == std::io::ErrorKind::InvalidInput));

        test_remove_file(test_file.name());
    }

    #[test]
    fn test_add_file() {
        let new_file = test_start_file(Some("add-file"));

        // Create File instance
        let test_file = File::new(new_file);

        let mut add_task = setup_task(2, "Buy cook dinner");
        let added = test_file.add(add_task).unwrap();
        assert_eq!(added, ());

        let mut tasks = test_file.list();
        assert_eq!(tasks.len(), 1);

        add_task = setup_task(3, "Buy groceries");
        let updated = test_file.add(add_task).unwrap();
        assert_eq!(updated, ());

        tasks = test_file.list();
        assert_eq!(tasks.len(), 2);

        test_remove_file(test_file.name());
    }

    #[test]
    #[should_panic(expected = "Error: Failed to update task: ID mismatch")]
    fn test_update_fail() {
        let update_file = test_start_file(Some("update-fail"));

        let list_task = setup_task(1, "Buy cook dinner");

        let tasks = Arc::new(Mutex::new(Vec::<Task>::new()));
        let json_string;
        {
            let mut data = tasks.lock().unwrap();
            data.extend(vec![list_task]);
            json_string = serde_json::to_string_pretty(&*data).unwrap();
            fs::write(update_file, json_string.clone()).unwrap();
        }

        // Create File instance
        let test_file = File::new(update_file);

        let one_task = test_file.list();
        assert_eq!(one_task.len(), 1);
        assert_eq!(one_task[0].id, 1);
        assert_eq!(one_task[0].description, "Buy cook dinner");

        // Not the bottom, clean up
        test_remove_file(test_file.name());

        // task id -1 == 1 should fail update
        let update_fail = test_file.update(-1, setup_task(1, "fail update"));
        // !!! This should panic
        assert!(update_fail.is_err());
    }

    #[test]
    fn test_update() {
        let update_file = test_start_file(Some("update"));

        let list_task = setup_task(1, "Buy cook dinner");

        let tasks = Arc::new(Mutex::new(Vec::<Task>::new()));
        let json_string;
        {
            let mut data = tasks.lock().unwrap();
            data.extend(vec![list_task]);
            json_string = serde_json::to_string_pretty(&*data).unwrap();
            fs::write(update_file, json_string.clone()).unwrap();
        }

        // Create File instance
        let test_file = File::new(update_file);

        let one_task = test_file.list();
        assert_eq!(one_task.len(), 1);
        assert_eq!(one_task[0].id, 1);
        assert_eq!(one_task[0].description, "Buy cook dinner");

        let update = test_file.update(1, setup_task(1, "Buy a pizza"));
        assert!(update.is_ok());
        let one_task_updated = test_file.list();
        assert_eq!(one_task_updated.len(), 1);
        assert_eq!(one_task_updated[0].id, 1);
        assert_eq!(one_task_updated[0].description, "Buy a pizza");

        test_remove_file(test_file.name());
    }

    #[test]
    fn test_delete_file_not_found() {
        let new_file = test_start_file(Some("delete-file-not-found"));

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
            fs::write(new_file, json_string.clone()).unwrap();
        }

        // Create File instance
        let test_file = File::new(new_file);

        // Attempt to delete a non-existing task: 4
        let deleted = test_file.delete(4);
        assert!(deleted.is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound));

        let tasks = test_file.list();
        assert_eq!(tasks.len(), 3);

        test_remove_file(test_file.name());
    }

    #[test]
    fn test_delete_file() {
        // ? bukan test_file_name(Some("..."));
        let delete_file = "delete-file-test-task-cli.json";

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
            fs::write(&delete_file, json_string.clone()).unwrap();
        }

        // Create File instance
        let test_file = File::new(&delete_file);

        let deleted = test_file.delete(2).unwrap();
        assert_eq!(deleted, ());

        let mut tasks = test_file.list();
        assert_eq!(tasks.len(), 2);

        tasks = test_file.list();
        assert_eq!(tasks.len(), 2);

        test_remove_file(test_file.name());
    }
}
