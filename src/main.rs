use std::env;
use std::fs::File;
use std::fmt::Display;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

const BUFFER_SIZE: usize = 4096;

fn main() {
    let args = env::args();
    if args.len() == 1 {
        cat(io::stdin(), "stdin");
    } else {
        // skip the first element as it is the program name
        for arg in args.skip(1) {
            if arg == "-" {
                cat(io::stdin(), "stdin");
            } else {
                let file = File::open(&arg)
                    .unwrap_or_else(|e| panic!("cat: failed to open{} : {}", &arg, e));
                cat(file, &arg);
            }
        }
    }
}

/// Read from `input` and print to stdout.
fn cat<R, N>(input: R, name: N)
        where R: Read, N: Display {
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, input);
    let mut s = String::new();
    loop {
        s.clear();
        let n = reader.read_line(&mut s);
        let n = n.unwrap_or_else(|e| panic!("cat: failed to read {}: {}", name, e));
        if n > 0 {
            print!("{}", s);
        } else {
            break;
        }
    }
    io::stdout().flush().unwrap();
}
