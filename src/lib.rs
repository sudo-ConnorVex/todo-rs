use std::fmt::Display;

pub mod commands;

// ============ Task ============

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub done: bool,
}

impl Task {
    pub fn new(id: u32, title: String) -> Self {
        Self { id, title, done: false }
    }

    pub fn switch_done(&mut self) {
        self.done = !self.done;
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mark = if self.done { "\x1b[31mx\x1b[0m" } else { " " };
        write!(f, "{}: [{}] | {}", self.id, mark, self.title)
    }
}