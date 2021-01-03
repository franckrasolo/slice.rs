use std::io;
use std::path::Path;
use std::process::exit;
use std::time::Instant;

use structopt::StructOpt;

use slicers::pre_push::PrePush;
use slicers::Config;

#[derive(Debug, StructOpt)]
#[structopt(about = "
A Git pre-push hook that encourages working at a sustainable pace.
As such, it rejects pushes of large change sets to a Git remote.
")]
pub struct Args {
    /// Name of the Git remote
    pub remote_name: String,
    /// Location of the Git remote
    pub remote_url: String,
}

fn main() {
    let args = Args::from_args();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!(">>> Inspecting push to {} at {}\n", args.remote_name, args.remote_url);
    let start = Instant::now();
    let result = PrePush::new(Path::new("."), Config::default()).run_hook(&input);
    let duration = start.elapsed();

    let code = match result {
        Ok(summary) => { println!("{}", summary.contents); 0 }
        Err(error)  => { eprintln!("{}", error); 1 }
    };
    println!("\n>>> Inspection completed in {:?}", duration);
    exit(code)
}
