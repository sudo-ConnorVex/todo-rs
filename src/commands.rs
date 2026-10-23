use std::{io::{self, IsTerminal, Write}, process::exit};

use crate::Task;

// ============== Helper functions ============== //
fn gen_id(list: &[Task]) -> u32 {
    list.iter()
        .map(|t| t.id)
        .max()
        .map_or(0, |m| m.saturating_add(1))
}

// ============== Commands ============== //
pub fn help_msg() {
    println!(
 "\x1b[90m
This tool is REPL style
commands:

    help, ?   - shows this message
    new       - adds a new task to tasks list
    remove    - removes a task with its ID
    list      - shows all of the tasks in list
    mark      - marks/unmarks a task with its ID
    delete    - deletes the tasks list
    clear     - clears the terminal
    quit/exit - exit the app
 \x1b[0m"
    )
}

pub fn new_item(list: &mut Vec<Task>, title: Option<&str>) {
    let title = match title {
        Some(val) => val.trim(),
        None => {
            println!("\x1b[31mNo arguments taken\x1b[0m");
            return;
        }
    };
    

    if title.is_empty() {
        println!("\x1b[31mTitle can't be empty\x1b[0m");
        return;
    }
    
    let id = gen_id(list);

    list.push(Task::new(id, title.to_string()));
    println!("\x1b[32mCreated with ID:\x1b[0m '{id}'")
}

pub fn rm_item(list: &mut Vec<Task>, args: Option<&str>) {
    let ids: Vec<u32> = match args {
        Some(val) => val
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect(),
        None => {
            println!("\x1b[31mNo arguments taken\x1b[0m");
            return;
        }
    };

    for id in ids {
        if let Some(i) = list.iter().position(|item| item.id == id) {
            list.remove(i);
            println!("\x1b[32mRemoved ID:\x1b[0m {id}");
        } else {
            println!("\x1b[31mID not found:\x1b[0m {id}");
        }
    }
}

pub fn show_list(list: &[Task]) {
    if list.is_empty() {
        println!("\x1b[33mList is empty\x1b[0m");
        return;
    }
    
    for item in list {
        println!("{item}");
    }
}

pub fn marks_task(list: &mut [Task], target: Option<&str>) {
    let ids: Vec<u32> = match target {
        Some(val) => val
            .split_whitespace()
            .filter_map(|s| s.parse::<u32>().ok())
            .collect(),
        None => {
            println!("\x1b[31mNo arguments taken\x1b[0m");
            return;
        }
    };
    
    for id in ids {
        if let Some(task) = list.iter_mut().find(|item| item.id == id) {
            if task.done {
                println!("\x1b[32mUnmarked ID: \x1b[0m{id}");
            } else {
                println!("\x1b[32mMarked ID: \x1b[0m{id}");
            }
            task.switch_done();
        } else {
            println!("\x1b[31mID not found:\x1b[0m {id}");
        }
    }
}

pub fn del_tasks(list: &mut Vec<Task>) {
    if list.is_empty() {
        println!("\x1b[33mList already is empty\x1b[0m");
        return;
    }
    list.clear();
}

pub fn clear() {
    if io::stdout().is_terminal() {
        print!("\x1b[2J\x1b[3J\x1b[H");
    	io::stdout().flush().unwrap();
    }
}

pub fn exit_success(code: i32) {
    println!("\n\x1b[0mOk Bye ;)");
    exit(code)
}