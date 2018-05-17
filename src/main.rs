use std::env;
use std::io::prelude::*;
use std::fs::File;


struct Args {
    flag: String,
    filename: String,
}

fn wc() -> u64 {
    unimplemented!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // TODO: unwrap this arg
    let args = Args::new(&args).unwrap_or_else(|err| {
        println!("shit is fucked, args not parsed!", );
    });

    println!("flag: {:?}", args.flag);
    println!("file: {:?}", args.filename);

    let mut f: File = File::open(args.filename).expect("file not found");
    let mut contents: String = String::new();

    f.read_to_string(&mut contents).expect("file contents whack");

    println!("file contents:\n{}", contents);
    
}

impl Args {
    fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("Not enough args, fool.");
        }
        let flag: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Args{ flag, filename })
    }
}
