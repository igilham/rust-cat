use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

const STDIN: &'static str = "-";
const BUFFER_SIZE: usize = 4096;

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
    let file = File::open(path)
       .expect(format!("cat: failed to open: {}", display).as_str());
    
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);
    loop {
        let mut s = String::new();
        let n = reader.read_to_string(&mut s)
            .expect(format!("cat: failed to open {}", display).as_str());
        if n > 0 {
            print!("{}", s);
        } else {
            break;
        }
    }
}
