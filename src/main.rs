use adventofcode2018_reattempt::{run, CliArgs};
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: CliArgs = CliArgs::from_args();
    run(&args, &mut std::io::stdout())
}
