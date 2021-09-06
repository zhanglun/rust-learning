mod cli;
use structopt::StructOpt;

fn main() {
    println!("Hello, world!");
    cli::CommandLineArgs::from_args();
}
