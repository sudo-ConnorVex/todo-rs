## ToDo-rs
A simple Rust REPL todo manager.
This tool does not save tasks and only keeps them in RAM. Because I don't know

## Run
build and run
```bash
cargo build --release

./target/release/todo_rs
```

## Commands
|Command|Discreption|
|-------|------------
|help, ? | Shows all of the commands|
|new `<TITLE>`| Adds a new task to list|
|remove `ID` `ID` ... | Removes the tasks with their IDs|
|list | Lists all of the tasks|
|mark `ID` `ID` ... | marks/unmarks the tasks with their IDs|
|delete | Clears the task list |
|clear | Clears the terminal |
|exit, quit | exits the tool |

## Examples 
```
██╗    ██╗███████╗██╗      ██████╗ ██████╗ ███╗   ███╗███████╗
██║    ██║██╔════╝██║     ██╔════╝██╔═══██╗████╗ ████║██╔════╝
██║ █╗ ██║█████╗  ██║     ██║     ██║   ██║██╔████╔██║█████╗
██║███╗██║██╔══╝  ██║     ██║     ██║   ██║██║╚██╔╝██║██╔══╝
╚███╔███╔╝███████╗███████╗╚██████╗╚██████╔╝██║ ╚═╝ ██║███████╗
 ╚══╝╚══╝ ╚══════╝╚══════╝ ╚═════╝ ╚═════╝ ╚═╝     ╚═╝╚══════╝

type 'help' for see commands

>> new 5 bananas
Created with ID: '0'
>> new 1 full fat milk
Created with ID: '1'
>> list
0: [ ] | 5 bananas
1: [ ] | 1 full fat milk
>> mark 0
Marked ID: 0
>> mark 0 1 2
Unmarked ID: 0
Marked ID: 1
ID not found: 2
>> remove 0
Removed ID: 0
>> list
1: [x] | 1 full fat milk
>> exit

Ok Bye ;)
```
Of course, the ANSI colors aren't visible here.