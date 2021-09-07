mod cli;
use structopt::StructOpt;

fn main() {
    println!("Hello, world!");
    println!("{:#?}", cli::CommandLineArgs::from_args());
}
