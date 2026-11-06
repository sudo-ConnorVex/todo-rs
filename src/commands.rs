use std::io::{self, IsTerminal};

use crate::{EROR, Task, Write};

/// This function is used whenever a new ID needs to be generated.
/// Here is what happens:
/// - It creates an iterator to traverse the provided list of tasks.
/// - The `map` method extracts the ID of each task and returns a new iterator containing these IDs.
/// - The `max` method returns the largest value from that iterator.
/// - The `map_or` method takes this maximum value, increments it by 1 using the `saturating_add` method, and returns the final result. If an error occurs for any reason, it defaults to returning 0.
fn gen_id(list: &[Task]) -> u64 {
    list.iter()
        .map(|t| t.id)
        .max()
        .map_or(0, |m| m.saturating_add(1))
}

/// This function checks whether the arguments for a command are empty; if so, it returns the requested error message wrapped in a `Result`.
fn if_is_empty(args: &[String]) -> Result<(), String> {
    if args.is_empty() {
        return Err(format!("{EROR} No arguments taken"));
    }
    Ok(())
}

/// This function is used for commands that take multiple arguments and need to be parsed into a type similar to the ID type of each task.
fn parse_target(target: Vec<String>) -> Result<Vec<u64>, String> {
    target
        .into_iter()
        .map(|arg| {
            arg.parse::<u64>()
                .map_err(|err| format!("{EROR} failed to parse '{arg}' into u64: {err}"))
        })
        .collect()
}

/// Adds one or more new tasks to the task list, depending on whether the user has entered the `new` or `n` command along with several arguments.
pub fn new_task(tasks: &mut Vec<Task>, titles: Vec<String>) -> Result<(), String> {
    if_is_empty(&titles)?;

    for title in titles {
        let id = gen_id(tasks);
        tasks.push(Task::new(id, title));
        println!("\x1b[32mTask created with ID\x1b[0m: {id}");
    }
    Ok(())
}

/// It iterates through the tasks and marks or unmarks those whose IDs match the ID or IDs provided in the argument.
pub fn mark_task(tasks: &mut [Task], target: Vec<String>) -> Result<(), String> {
    if_is_empty(&target)?;

    let ids: Vec<u64> = parse_target(target)?;

    ids.iter().for_each(|id| {
        let found = tasks.iter_mut().find(|task| &task.id == id);

        if let Some(task) = found {
            task.switch_done();
            if task.done {
                println!("\x1b[32mMarked task\x1b[0m:\t {task}")
            } else {
                println!("\x1b[32mUnmarked task\x1b[0m:\t {task}")
            }
        } else {
            println!("\x1b[31mID not found:\x1b[0m {id}");
        }
    });
    Ok(())
}

/// It clears the task list completely. However, before clearing the list, it asks the user for confirmation.
pub fn clear_tasks(tasks: &mut Vec<Task>) -> Result<(), String> {
    if tasks.is_empty() {
        return Err("\x1b[33mTasks list is already empty\x1b[0m".to_string());
    }

    print!("\x1b[31mYou sure?\x1b[0m [N/y]: ");
    io::stdout().flush().expect("failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to get user input");

    match input.to_lowercase().trim() {
        "y" | "yes" => {
            tasks.clear();
            println!("Tasks list cleared!");
        }
        _ => println!("Tasks list clear cancelled!"),
    }
    Ok(())
}

/// Used for deleting a task or tasks, depending on the number of arguments provided.
pub fn remove_task(tasks: &mut Vec<Task>, target: Vec<String>) -> Result<(), String> {
    if_is_empty(&target)?;

    let ids: Vec<u64> = parse_target(target)?;

    ids.iter().for_each(|id| {
        if let Some(index) = tasks.iter().position(|task| &task.id == id) {
            tasks.remove(index);
            println!("\x1b[32mRemoved ID:\x1b[0m {id}");
        } else {
            println!("\x1b[31mID not found:\x1b[0m {id}");
        }
    });
    Ok(())
}

/// It checks if the list is non-empty, then iterates over the tasks and prints them all. The `Display` trait has been implemented for the `Task` struct.
pub fn list(tasks: &[Task]) {
    if !tasks.is_empty() {
        tasks.iter().for_each(|task| println!("{task}"))
    } else {
        println!("\x1b[33mTask list is already empty\x1b[0m");
    }
}

/// It is used in REPL mode to clear the terminal.
pub fn clear() {
    if !io::stdout().is_terminal() {
        return;
    }

    print!("\x1b[2J\x1b[3J\x1b[H");
    io::stdout().flush().unwrap();
}

/// Depending on the type of tool being used, it displays an appropriate help message.
pub fn show_help(is_repl: bool) {
    let commands = "
Usage: todo [COMMAND] [ARG 1] [ARG 2] ...

Commands:
    new,     n  <TITLE>  -  Adds new task to tasks list
    mark,    m  <IDs>    -  Marks the task as completed, or vice versa
    remove,  r  <IDs>    -  Deletes the tasks with the given IDs
    list,    l           -  Shows the tasks in the list
    clt,     c           -  Clears the tasks list
    help,    h           -  Shows this help message
    clear (REPL mode)    -  Clears the terminal
    exit, q (REPL mode)  -  Exits the app
 ";

    let description = "
Description: A simple todo list CLI tool written in Rust.
The JSON file containing the list of tasks is located at /tmp/todo.json.
For now, only a Linux version of this tool exists (I don't think it will be of any use to Windows users)
GitHub: https://github.com/sudo-ConnorVex/todo-rs
 ";

    let options = "
Options:
    -V, --version        -  Shows the current version of the tool
    --repl               -  Open tool in REPL mode
 ";

    if is_repl {
        println!("{commands}")
    } else {
        println!("{commands} {options} {description}")
    }
}
