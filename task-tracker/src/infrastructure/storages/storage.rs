use crate::domain::task::Task;
use crate::error::{panic_invalid_input, panic_not_found_input};
use crate::infrastructure::{
    config,
    storages::storage_specifics::{
        specifics_of_add, specifics_of_delete, specifics_of_list, specifics_of_update,
    },
};

use chrono::Local;

use std::fs::{File as std_file, OpenOptions};
use std::{fs, io::Read};

pub const FILE_NAME: &str = "FILE";

// Store in JSON file

pub trait StorageTrait {
    fn new(config: &config::Config) -> Self
    where
        Self: Sized;
    fn list(&self) -> Vec<Task>;
    fn add(&self, add_task: &Task) -> Vec<Task>;
    fn update(&self, id: i32, update_task: &mut Task) -> Vec<Task>;
    fn delete(&self, id: i32) -> Vec<Task>;
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct Storage {
    json_str: String,
}

// TDD
// ✅ ❔ ❌
// 1. buatlah struktur data File dengan method new, list, add, update, delete
// => 1. create the File data structure with methods new, list, add, update
// ---------------------------
// 1.1. buat method `new` untuk inisialisasi File ✅
// => 1.1. create a `new` method to initialize the File
// 1.2. buat method `list` untuk mendapatkan daftar Task ✅
// => 1.2. create a `list` method to get a list of the Tasks
// 1.3. buat method `add` untuk menambahkan Task baru ✅
// => 1.3. create an `add` method to add a new Task
// 1.4. buat method `update` untuk memperbarui Task berdasarkan ID ✅
// => 1.4. create an `update` method to update the Task by ID
// 1.5. buat method `delete` untuk menghapus Task berdasarkan ID ✅
// => 1.5. create a `delete` method to delete Tasks based on its ID
#[allow(dead_code)]
impl StorageTrait for Storage {
    fn new(config: &config::Config) -> Self {
        let env_json = config.env_json.clone();
        new_open_options(&env_json);
        Self { json_str: env_json }
    }

    fn list(&self) -> Vec<Task> {
        let tasks_string = tasks_str(&self.json_str);

        specifics_of_list(&tasks_string)
    }

    // IMPORTANT: not an error, for example: `Task` object should no longer be validated
    fn add(&self, add_task: &Task) -> Vec<Task> {
        let tasks_string = tasks_str(&self.json_str);

        let task = specifics_of_add(&tasks_string, add_task);
        to_file_by_json(&self.json_str, &task);
        task
    }

    // IMPORTANT: not an error, for example: `Task` object should no longer be validated
    fn update(&self, id: i32, update_task: &mut Task) -> Vec<Task> {
        let tasks_string = tasks_str(&self.json_str);
        // ?
        update_task.updated_at = Local::now().into();
        let tasks = specifics_of_update(tasks_string, id, update_task);
        to_file_by_json(&self.json_str, &tasks);
        tasks
    }

    // IMPORTANT: not an error, for example: `Task` object should no longer be validated
    fn delete(&self, id: i32) -> Vec<Task> {
        let tasks_string = tasks_str(&self.json_str);

        let tasks = specifics_of_delete(tasks_string, id);
        to_file_by_json(&self.json_str, &tasks);
        tasks
    }
}

fn new_open_options(file_name: &String) -> () {
    if !fs::metadata(file_name).is_ok() {
        let _ = OpenOptions::new()
            .write(true)
            .create_new(true) // Will error if file already exists
            .open(file_name)
            .unwrap();

        to_file_by_json(file_name, &vec![]);
    }
}

fn get_open_options(file_name: &String) -> std_file {
    let file = OpenOptions::new().read(true).open(file_name);
    if file.is_err() {
        panic_not_found_input(FILE_NAME, "failed to open file");
    }

    file.unwrap()
}

fn tasks_str(file_name: &String) -> String {
    let mut tasks_file = get_open_options(file_name);

    let mut tasks_string = String::new();
    let _ = tasks_file.read_to_string(&mut tasks_string);
    tasks_string
}

fn to_file_by_json(file_name: &String, tasks: &Vec<Task>) -> () {
    let json_string = serde_json::to_string_pretty(tasks).unwrap();
    if fs::write(file_name, json_string).is_err() {
        panic_invalid_input(FILE_NAME, "failed to write to file");
    }
}
