use std::env;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

const STDIN: &'static str = "-";
const BUFFER_SIZE: usize = 4096;

enum Method<'a> {
    File(&'a str),
    Line,
}

impl<'a> Method<'a> {
    fn name(&self) -> &str {
        match *self {
            Method::File(name) => name,
            Method::Line => "stdin",
        }
    }
}

fn main() {
    // skip the first element as it is the program name
    let args = env::args();
    if args.len() == 1 {
        cat(io::stdin(), Method::Line);
    } else {
        for arg in args.skip(1) {
            if arg.eq(STDIN) {
                cat(io::stdin(), Method::Line);
            } else {
                let file = File::open(&arg)
                    .unwrap_or_else(|e| panic!("cat: failed to open {}: {}", &arg, e));
                cat(file, Method::File(&arg));
            }
        }
    }
    io::stdout().flush().unwrap();
}

/// Read from `input` and print to stdout.
fn cat<R>(input: R, m: Method) where R: Read {
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, input);
    cat_recurse(&mut reader, m);
}

fn cat_recurse<R>(reader: &mut BufReader<R>, m: Method) where R: Read {
    let mut s = String::new();
    let n = match m {
        Method::Line => reader.read_line(&mut s),
        Method::File(..) => reader.read_to_string(&mut s),
    };
    let n = n.unwrap_or_else(|e|panic!("cat: failed to open {}: {}", m.name(), e));
    if n > 0 {
        print!("{}", s);
        cat_recurse(reader, m);
    }
}
