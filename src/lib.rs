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
        if args.len() < 3 {
            return Err("Not enough args, fool.");
        }
        let flag: String = args[1].clone();
        let filename: String = args[2].clone();

        Ok(Args{ flag, filename })
    }
}

pub fn run(args: Args) -> Result<(), Box<Error>> {
    let mut f: File = File::open(args.filename)?;
    //let mut contents: String = String::new();

    //f.read_to_string(&mut contents)?;
    
    //println!("\nfile contents:\n{}", contents);

    println!("wc: {}", wc(f));
    Ok(())
}

fn wc(file: File) -> u64 {
    let buff = BufReader::new(file);
    let mut counter = 0;
    let mut inword: bool = false;
    let mut wierd_shit: bool = false;
    let mut prev_char: char;
    let mut char_before_char: char = '\0';
    // for line in buff.lines() {
    //     println!("line: {}", line.unwrap());
    //     counter += 1;
    // }
    for line in buff.lines() {
        // bufreader returns Result<String, Err>, unwrap before using
        for c in line.unwrap().chars() {
            // println!("char: {}", c);
            prev_char = c;
            match c {
                ' ' | '\n' | '\r' | '\t' => {
                    inword = false;
                    wierd_shit = false;
                },
                '-' | '—' | '*' | '_' => {
                    inword = false;
                    wierd_shit = true;
                    // println!("prev_char: {}", prev_char);
                },
                _ => {
                    inword = true;
                    wierd_shit = false;
                    
                }
            }
            if char_before_char == '-' ||
                char_before_char == '—' ||
                char_before_char == '*' ||
                char_before_char == '_' {
                wierd_shit = true;
            }
            if !inword && !wierd_shit {
                // println!("========== word! ==========");
                counter += 1;
            }

            char_before_char = prev_char;
            // println!("char_before_char: {}", char_before_char);
        }
        // println!("========== word! ==========");
        counter += 1;
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_word() {
        let mut flag: String = "-w".to_string();
        let f: File = File::open("Im-Nobody-E-Dickinson.txt").expect("whoops");

        assert_eq!(44, wc(f));
    }
}
