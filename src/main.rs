extern crate wcrs;

use std::env;
use std::process;

use wcrs::Args;

fn main() {
    let args: Vec<String> = env::args().collect();


    let args = Args::new(&args).unwrap_or_else(|err| {
        println!("Error with args, args not parsed! {}", err);
        process::exit(1);
    });

    if let Err(e) = wcrs::run(args) {
        println!("wc error: {}", e);
        process::exit(1);
    }
}


