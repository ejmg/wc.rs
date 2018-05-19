use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Args {
    pub flag: String,
    pub filename: String,
}

impl Args {
    pub fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments passed.")
        }
        if args.len() < 3 {
            let flag: String = String::from("-none");
            let filename = args[1].clone();
            return Ok(Args{ flag, filename });
        }
        let flag: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Args{ flag, filename })
    }
}

pub fn run(args: Args) -> Result<(), Box<Error>> {
    let f: File = File::open(&args.filename)?;

    let results: (u64, u64, u64);
    
    results = wc(f);

    if args.flag == "-c" {
        println!("text: {}, chars: {}", args.filename, results.0);
    } else if args.flag == "-w" {
        println!("text: {}, words: {}", args.filename, results.1);
    } else if args.flag == "-l" {
        println!("text: {}, lines: {}", args.filename, results.2);
    } else {
        println!("text: {}, chars: {}, words: {}, lines: {}", args.filename,
                 results.0, results.1, results.2);
    }
    Ok(())
}

fn wc(file: File) -> (u64, u64, u64) {
    let buff = BufReader::new(file);
    let mut w = 0;
    let mut chars = 0;
    let mut l = 0;
    let mut inword: bool;
    let mut symbol: bool;
    let mut prev_char: char;
    // given the logic of my program, i need to give this a value else it will
    // allow an invalid comparison on the first go.
    let mut char_before_char: char = '\0';
    
    for line in buff.lines() {
        // bufreader returns Result<String, Err>, unwrap before using
        for c in line.unwrap().chars() {
            // capture the char before using it.
            prev_char = c;
            match c {
                ' ' | '\n' | '\r' | '\t' => {
                    inword = false;
                    symbol = false;
                },
                '-' | '—' | '*' | '_' => {
                    inword = false;
                    symbol = true;
                    chars += 1;
                },
                _ => {
                    inword = true;
                    symbol = false;
                    chars += 1;
                    
                }
            }
            if char_before_char == '-' ||
                char_before_char == '—' ||
                char_before_char == '*' ||
                char_before_char == '_' {
                symbol = true;
            }
            if !inword && !symbol {
                w += 1;
            }

            char_before_char = prev_char;
        }
        w += 1;
        l += 1;
    }
    (chars, w, l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_wc() {
        let mut flag: String = "-w".to_string();
        let f: File = File::open("Im-Nobody-E-Dickinson.txt").expect("whoops");

        assert_eq!(44, wc(f));
    }
}
