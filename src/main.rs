use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const STDIN: &'static str = "-";

fn main() {
    // skip the first element as it is the program name
    let args = env::args();
    if args.len() == 1 {
        cat_stdin();
    } else {
        for arg in args.skip(1) {
            if arg.eq(STDIN) {
                cat_stdin();
            } else {
                let path = Path::new(&arg);
                cat(&path);
            }
        }
    }
}

fn cat_stdin() {
    let stdin = io::stdin();
    let mut s = String::new();
    loop {
        match stdin.read_line(&mut s) {
            Err(why) => panic!("cat: failed to read from stdin: {}", why),
            Ok(n) => if n == 0 {
                break;
            } else {
                print!("{}", s);
                s.clear();
            },
        };
    }
}

fn cat(path: &Path) {
    let display = path.display();
    let mut file = match File::open(path) {
        Err(why) => panic!("cat: failed to open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("cat: failed to read {}: {}", display, why.description()),
        Ok(_) => print!("{}", s),
    };
}
