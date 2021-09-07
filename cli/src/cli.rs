use std::path::PathBuf;
use structopt:: StructOpt;

//来源： https://docs.microsoft.com/zh-cn/learn/modules/rust-create-command-line-program/3-cli-module

#[derive(Debug, StructOpt)]
pub enum Action {
  Add { 
    #[structopt()]
    task: String,
   },
  Done {
    #[structopt()]
    position: usize,
  },
  List,
}

#[derive(Debug, StructOpt)]
#[structopt(
  name = "Rusty Journal",
  about = "A command line to-do app written in Rust"
)]

pub struct CommandLineArgs {
  #[structopt(subcommand)]
  pub action: Action,

  /// use a different journal file.
  #[structopt(parse(from_os_str), short, long)]
  pub journal_file: Option<PathBuf>,
}