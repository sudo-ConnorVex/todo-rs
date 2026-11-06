use std::{
    io::{self, Write},
    process::exit,
};

use clap::Parser;
use todo_rs::{
    Args, EROR,
    commands::{clear, clear_tasks, list, mark_task, new_task, remove_task, show_help},
    read_json, write_json,
};

/// This function utilizes the tool via a CLI.
/// Specifically, the user must provide the program name, the desired command, and the required arguments.
fn cli_mode(args: Args) {
    let mut tasks = read_json();

    match args.command {
        Some(cmd) => match cmd.to_lowercase().as_str() {
            "new" | "n" => new_task(&mut tasks, args.cmd_args).unwrap_or_else(|err| {
                eprintln!("{err}");
                exit(1)
            }),
            "remove" | "r" => remove_task(&mut tasks, args.cmd_args).unwrap_or_else(|err| {
                eprintln!("{err}");
                exit(1)
            }),
            "mark" | "m" => mark_task(&mut tasks, args.cmd_args).unwrap_or_else(|err| {
                eprintln!("{err}");
                exit(1)
            }),
            "clt" | "c" => {
                if let Err(err) = clear_tasks(&mut tasks) {
                    eprintln!("{err}");
                    exit(1)
                }
            }
            "list" | "l" => {
                list(&tasks);
                exit(0)
            }
            "help" | "h" => {
                show_help(args.repl);
                exit(0)
            }
            _ => {
                eprintln!("{EROR} Unknown command: {cmd}");
                exit(1)
            }
        },
        None => show_help(args.repl),
    }

    write_json(&tasks);
}

/// This function creates a REPL loop, which serves as the core environment where the application's main events take place.
/// The commands and arguments entered by the user are processed within this loop.
fn repl_mode(args: Args) {
    let mut tasks = read_json();

    println!("Welcome to todo-rs REPL mode");
    println!("type h for help");

    loop {
        print!("cmd> ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        let bytes = io::stdin().read_line(&mut input).unwrap_or_default();

        if bytes == 0 {
            println!("Ok Bye ;)");
            exit(0)
        }

        let mut input = input.split_whitespace();
        let cmd = input.next().unwrap_or("");
        let other: Vec<String> = input.map(String::from).collect();

        match cmd.to_lowercase().as_str() {
            "new" | "n" => {
                if let Err(err) = new_task(&mut tasks, other) {
                    eprintln!("{err}");
                    continue;
                }
            }
            "remove" | "r" => {
                if let Err(err) = remove_task(&mut tasks, other) {
                    eprintln!("{err}");
                    continue;
                }
            }
            "mark" | "m" => {
                if let Err(err) = mark_task(&mut tasks, other) {
                    eprintln!("{err}");
                    continue;
                }
            }
            "clt" | "c" => {
                if let Err(err) = clear_tasks(&mut tasks) {
                    eprintln!("{err}");
                    continue;
                }
            }
            "list" | "l" => {
                list(&tasks);
                continue;
            }
            "help" | "h" => {
                show_help(args.repl);
                continue;
            }
            "clear" => {
                clear();
                continue;
            }
            "exit" | "q" => {
                println!("Ok bye ;)\n");
                exit(0)
            }
            _ => {
                eprintln!("{EROR} Unknown command: {cmd}");
                continue;
            }
        }

        write_json(&tasks);
    }
}

/// Here, we parse the arguments passed to the program.
/// If the `repl` state is `true` (meaning the user has included the `--repl` flag), the program calls the `repl_mode` function. Otherwise, it runs in CLI mode.
fn main() {
    let args = Args::parse();

    if args.repl {
        repl_mode(args);
    } else {
        cli_mode(args);
    }
}
