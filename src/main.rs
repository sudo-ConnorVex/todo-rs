use std::io::{self, Write};

use todo_rs::{Task, commands::{clear, del_tasks, exit_success, help_msg, marks_task, new_item, rm_item, show_list}};

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    clear();
    println!(
"\x1b[90m
██╗    ██╗███████╗██╗      ██████╗ ██████╗ ███╗   ███╗███████╗
██║    ██║██╔════╝██║     ██╔════╝██╔═══██╗████╗ ████║██╔════╝
██║ █╗ ██║█████╗  ██║     ██║     ██║   ██║██╔████╔██║█████╗  
██║███╗██║██╔══╝  ██║     ██║     ██║   ██║██║╚██╔╝██║██╔══╝  
╚███╔███╔╝███████╗███████╗╚██████╗╚██████╔╝██║ ╚═╝ ██║███████╗
 ╚══╝╚══╝ ╚══════╝╚══════╝ ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚══════╝
\x1b[0m"
    );
    println!("type 'help' for see commands\n");
    
    loop {
        print!("\x1b[90m>>\x1b[0m \x1b[36m");
        io::stdout().flush().expect("Error: failed to flush stdout. from main");
        
        let mut input = String::new();
        let bytes = io::stdin()
            .read_line(&mut input)
            .expect("Error: failed to get user input. from main");

        if bytes == 0 {
            exit_success(0);
        }
        
        print!("\x1b[0m");
        io::stdout().flush().expect("Error: failed to flush stdout. from main");


        let mut input = input.trim().splitn(2, ' ');
        let cmd = input.next().unwrap();
        let other = input.next();
        
        match cmd.to_lowercase().trim() {
            "help" | "?"    => help_msg(),
            "list"          => show_list(&tasks),
            "new"           => new_item(&mut tasks, other),
            "mark"          => marks_task(&mut tasks, other),
            "remove"        => rm_item(&mut tasks, other),
            "delete"        => del_tasks(&mut tasks),
            "clear"         => clear(),
            "exit" | "quit" => exit_success(0),
            cmd             => {
                if cmd.is_empty() {
                    continue;
                }
                println!("\x1b[31mUnknown command:\x1b[0m {cmd}");
                continue
            }
        }
    }
}