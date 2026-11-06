use clap::Parser;

#[derive(Parser)]
#[command(version, long_about = None, disable_help_flag = true)]
pub struct Args {
    /// REPL mode
    #[arg(long)]
    pub repl: bool,

    /// Command
    pub command: Option<String>,

    /// the command argument
    #[arg(num_args = 1..)]
    pub cmd_args: Vec<String>,
}
