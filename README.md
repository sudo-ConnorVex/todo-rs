## Release v1.0.0 (Major Rewrite)

**Note:** This version is a complete rewrite of the original `todo-repl` project and is **not backward compatible** with older versions. 

Key improvements include:
- **Data Persistence**: Tasks are now saved to a JSON file (previously kept in RAM only).
- **Dual Modes**: Added a standard CLI mode alongside the existing REPL mode.
- **Corruption Recovery**: Auto-backup and recovery mechanism for corrupted JSON files.
- **Improved Error Handling**: Replaced simple error checks with robust `Result`-based error handling and user-friendly feedback.
- **Better Architecture**: Modularized the codebase into `cli`, `commands`, and `lib` for better maintainability.

# todo-rs 📝

A simple, fast, and robust command-line todo list manager written in Rust.

`todo-rs` is designed to be lightweight and user-friendly, offering both a standard CLI mode for quick tasks and an interactive REPL (Read-Eval-Print Loop) mode for continuous task management. It features automatic JSON state saving, data corruption recovery, and a clean terminal interface with ANSI colors.

## ✨ Features

- **Dual Modes**: Use it as a standard CLI tool or enter interactive REPL mode.
- **Data Persistence**: Automatically saves your tasks to a JSON file (`/tmp/todo.json`).
- **Corruption Recovery**: If the JSON file gets corrupted, the tool prompts you to create a backup and start fresh instead of crashing.
- **Colorful Output**: Uses ANSI colors for clear and readable terminal output.
- **Smart ID Generation**: Automatically generates unique, incrementing IDs for your tasks.
- **Batch Operations**: Add, remove, or mark multiple tasks at once using space-separated arguments.

## 📦 Installation

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.60 or later recommended)

### Build from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/sudo-ConnorVex/todo-rs.git
   cd todo-rs
   ```

2. Build the project in release mode:
   ```bash
   cargo build --release
   ```

3. The executable will be located at `target/release/todo-rs`. You can copy it to a directory in your `PATH` (e.g., `/usr/local/bin`):
   ```bash
   sudo cp target/release/todo-rs /usr/local/bin/todo
   ```

## 🚀 Usage

`todo-rs` supports two main modes of operation:

### 1. CLI Mode (Default)
Pass commands and arguments directly to the tool. It executes the command and exits immediately.

```bash
todo [COMMAND] [ARG 1] [ARG 2] ...
```

### 2. REPL Mode
Launch an interactive shell where you can type commands continuously. This mode is perfect for managing your tasks without typing `todo` every time.

```bash
todo --repl
```

## 📖 Commands

| Command | Alias | Arguments | Description |
| :--- | :--- | :--- | :--- |
| `new` | `n` | `<TITLE...>` | Adds one or more new tasks to the list. |
| `mark` | `m` | `<IDs...>` | Toggles the completion status of the given task IDs. |
| `remove` | `r` | `<IDs...>` | Deletes the tasks with the given IDs. |
| `list` | `l` | *(None)* | Displays all tasks in the list. |
| `clt` | `c` | *(None)* | Clears the entire task list (asks for confirmation). |
| `help` | `h` | *(None)* | Shows the help message. |
| `clear` | *(None)* | *(None)* | Clears the terminal screen (REPL mode only). |
| `exit` | `q` | *(None)* | Exits the application (REPL mode only). |

### Options
- `-V`, `--version`: Shows the current version of the tool.
- `--repl`: Starts the tool in REPL mode.

## 💡 Examples

**Add tasks:**
```bash
todo new "Buy milk" "Walk the dog"
# Output: Task created with ID: 1
#         Task created with ID: 2
```

**List tasks:**
```bash
todo list
# Output: 1: [ ] | Buy milk
#         2: [ ] | Walk the dog
```

**Mark tasks as done:**
```bash
todo mark 1 2
# Output: Marked task: 1:    [x] | Buy milk
#         Marked task: 2:    [x] | Walk the dog
```

**Remove a task:**
```bash
todo remove 1
# Output: Removed ID: 1
```

**Use REPL mode:**
```bash
$ todo --repl
Welcome to todo-rs REPL mode
type h for help
cmd> new "Finish Rust project"
Task created with ID: 1
cmd> list
1: [ ] | Finish Rust project
cmd> exit
Ok bye ;)
```

## 🗂️ Data Storage

All tasks are stored locally in a JSON file at:
```text
/tmp/todo.json
```
> **Note**: Since the file is stored in `/tmp`, it may be cleared automatically by the system upon reboot. If you want permanent storage, you can change the `FILE_PATH` constant in `src/lib.rs` to a directory like `~/.local/share/todo-rs/tasks.json`.

## 🛠️ Built With

- [Rust](https://www.rust-lang.org/) - The programming language.
- [clap](https://crates.io/crates/clap) - Command-line argument parsing.
- [serde](https://crates.io/crates/serde) & [serde_json](https://crates.io/crates/serde_json) - JSON serialization and deserialization.

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/sudo-ConnorVex/todo-rs/issues).

## 📝 License

This project is open-source and available under the [MIT License](LICENSE).

---
*Made with ❤️ and Rust.*