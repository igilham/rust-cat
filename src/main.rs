use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // skip the first element as it is the program name
    for arg in env::args().skip(1) {
        let path = Path::new(&arg);
        cat(&path);
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
