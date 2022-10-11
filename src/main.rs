use clap::Parser;
use std::env;

#[derive(Parser, Debug, Clone, Copy)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    #[arg(short, help = "Only print keys")]
    keys: bool,
}

fn main() {
    let opts = Opts::parse();
    if opts.keys {
        for (k, _) in env::vars() {
            println!("{}", k);
        }
    } else {
        for (k, v) in env::vars() {
            println!("{}={}", k, v);
        }
    }
}
