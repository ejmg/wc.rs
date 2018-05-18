extern crate wcrs;

use std::env;
use std::process;

use wcrs::Args;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);


    let args = Args::new(&args).unwrap_or_else(|err| {
        println!("shit is fucked, args not parsed! {}", err);
        process::exit(1);
    });

    println!("flag: {:?}", args.flag);
    println!("file: {:?}", args.filename);

    if let Err(e) = wcrs::run(args) {
        println!("wc error: {}", e);
        process::exit(1);
    }
}


