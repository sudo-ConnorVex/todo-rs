use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    process::exit,
};

mod cli;
pub use cli::Args;

pub mod commands;

/// File path to save the JSON file.
pub const FILE_PATH: &str = "/tmp/todo.json";
/// Depending on the type of tool being used, it displays an appropriate help message.
pub const EROR: &str = "\x1b[31mError:\x1b[0m";

/// The `Task` struct annotated with serialization and deserialization attributes for converting JSON to data and vice versa.
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: u64,
    title: String,
    done: bool,
}

/// The two methods, `new` and `switch_done`, which are used in the relevant functions.
impl Task {
    pub fn new(id: u64, title: String) -> Self {
        Self {
            id,
            title,
            done: false,
        }
    }

    pub fn switch_done(&mut self) {
        self.done = !self.done;
    }
}

/// Implement the `Display` trait for the task so that it can be printed using `println!`.
impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mark = if self.done { "\x1b[31mx\x1b[0m" } else { " " };
        write!(f, "{}:\t[{}] | {}", self.id, mark, self.title)
    }
}

/// This function opens the file at FILE_PATH with read and write access, then returns a file handle.
fn open_file() -> File {
    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(FILE_PATH)
        .unwrap_or_else(|error| {
            eprintln!("{EROR} failed to open file: {error}");
            exit(1)
        })
}

/// This function takes the file handle returned by `open_file`, converts its contents into a string, and stores it. This data is in JSON format.
/// Then, using `serde`, we attempt to parse the retrieved data into a vector of tasks. If this fails for any reason:
/// - We ask the user whether they would like us to create a copy of the original file and overwrite it:
///     - If the user enters 'y' or 'yes', we create a backup of the original file in the same directory.
///     - We return an empty vector—a vector intended to be modified throughout the program's execution and populated with valid JSON data.
/// - If the user enters anything else, it indicates they do not wish for the file to be overwritten, and we exit the program with an exit code of 0.
pub fn read_json() -> Vec<Task> {
    let mut json_data = String::new();
    open_file()
        .read_to_string(&mut json_data)
        .unwrap_or_else(|err| {
            eprintln!("{EROR} failed to open '{FILE_PATH}': {err}");
            exit(2)
        });

    match serde_json::from_str(&json_data) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{EROR} failed to read JSON data: {err}");
            print!("Do you want to backup the tasks list \nand rewrite it? [N/y]: ");

            io::stdout()
                .flush()
                .unwrap_or_else(|err| eprintln!("{EROR} failed to flush stdout: {err}"));
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap_or_else(|err| {
                eprintln!("{EROR} failed to get user input: {err}");
                exit(2)
            });

            match input.to_lowercase().trim() {
                "y" | "yes" => {
                    if let Err(err) = fs::copy(FILE_PATH, format!("{FILE_PATH}.bak")) {
                        eprintln!("{EROR} failed to create backup from '{FILE_PATH}': {err}");
                        exit(2)
                    }
                    Vec::new()
                }
                _ => {
                    println!("Ok cancelling. No changes were made");
                    exit(0);
                }
            }
        }
    }
}

/// This function takes the new Task vector, parses its data into JSON format, and saves it to the `FILE_PATH`.
pub fn write_json(tasks: &[Task]) {
    let json_data = serde_json::to_string_pretty(&tasks).unwrap_or_else(|err| {
        eprintln!("{EROR} failed to parse data to JSON: {err}");

        exit(2)
    });
    fs::write(FILE_PATH, json_data).unwrap_or_else(|err| {
        eprintln!("{EROR} failed to write into '{FILE_PATH}': {err}");
        exit(2)
    });
}
