mod cli;
mod reactor;

use cli::CliArgs;
use clap::Parser;
use reactor::pipe::Pipe;

fn main() {
    let args = CliArgs::parse();

    let pipe_a = Pipe::new(33, 36, 500);

    println!("Hello, world! {}", args.name);
    println!("pipe_a = id:{} od:{} length:{} vol:{}",
             pipe_a.get_id(), pipe_a.get_od(),
             pipe_a.get_length(), pipe_a.get_volume());

    // Why does the const keyword exist? surely it's just a regular var?
    const A: i32 = 32;
    println!("{}", A)
}
