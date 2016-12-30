use std::env;
use std::fs::File;
use std::fmt::Display;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

const STDIN: &'static str = "-";
const BUFFER_SIZE: usize = 4096;

enum Method {
    File,
    Line,
}

fn main() {
    // skip the first element as it is the program name
    let args = env::args();
    if args.len() == 1 {
        cat(io::stdin(), "stdin", Method::Line);
    } else {
        for arg in args.skip(1) {
            if arg.eq(STDIN) {
                cat(io::stdin(), "stdin", Method::Line);
            } else {
                let path = Path::new(&arg);
                let display = path.display();
                let file = File::open(path)
                    .expect(format!("cat: failed to open: {}", display).as_str());
                cat(file, display, Method::File);
            }
        }
    }
    io::stdout().flush().unwrap();
}

// Read from `input` and print to stdout.
// `name` is used in error messages.
// `m` selects whether to read line-by-line or to read the entire input.
fn cat<R, N>(input: R, name: N, m: Method) where R: Read, N: Display {
    let mut reader = BufReader::with_capacity(BUFFER_SIZE, input);
    cat_recurse(&mut reader, name, m);
}

fn cat_recurse<R, N>(reader: &mut BufReader<R>, name: N, m: Method) where R: Read, N: Display {
    let mut s = String::new();
    let n = match m {
        Method::Line => reader.read_line(&mut s),
        Method::File => reader.read_to_string(&mut s),
    }.expect(format!("cat: failed to open {}", name).as_str());
    if n > 0 {
        print!("{}", s);
        // recurse
        cat_recurse(reader, name, m);
    }
}
