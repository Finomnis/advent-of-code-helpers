pub use clap::Parser;

/// This is a solver for Advent of Code 2021 tasks.
#[derive(Parser)]
#[clap()]
pub struct Options {
    /// The task on the day, can be 1 or 2
    #[clap()]
    pub task: u8,

    /// The path to the challenge input data
    #[clap()]
    pub data: ::std::path::PathBuf,

    /// Run the reworked solution, if available
    #[clap(long)]
    pub reworked: bool,

    /// Render the task visually, if available
    #[clap(long)]
    pub render: bool,
}
