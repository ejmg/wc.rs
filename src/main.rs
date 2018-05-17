use std::env;
use std::io::prelude::*;
use std::fs::File;


fn wc() -> u64 {
    unimplemented!()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let flag: &String = &args[1];
    let file: &String = &args[2];

    println!("flag: {:?}", flag);
    println!("file: {:?}", file);

    let mut f: File = File::open(file).expect("file not found");

    let mut contents: String = String::new();

    f.read_to_string(&mut contents).expect("file contents whack");

    println!("file contents:\n {}", contents);
    
}
