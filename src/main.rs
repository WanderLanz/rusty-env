use clap::Parser;
use std::env;

#[derive(Parser, Debug, Clone)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    #[arg(short, help = "Only print keys")]
    keys: bool,
    #[arg(short, help = "Truncate values to given length")]
    value_length: Option<u16>,
    #[arg(
        short,
        help = "Suffix to indicate truncation",
        requires = "value_length",
        default_value = "..."
    )]
    suffix: String,
}

fn main() {
    let opts = Opts::parse();
    if opts.keys {
        for (k, _) in env::vars() {
            println!("{}", k);
        }
    } else if let Some(l) = opts.value_length {
        for (k, mut v) in env::vars() {
            if v.len() > l as usize {
                v.truncate(l as usize);
                v.push_str(&opts.suffix);
            }
            println!("{}=\"{}\"", k, v);
        }
    } else {
        for (k, v) in env::vars() {
            println!("{}=\"{}\"", k, v);
        }
    }
}
