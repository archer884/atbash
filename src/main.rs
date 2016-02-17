extern crate grabinput;
mod reader;
use std::io::{Cursor, Read};
use reader::Atbash;

fn main() {
    let mut input = Cursor::new(grabinput::all(std::env::args().nth(1))).atbash();
    let mut output = String::new();

    input.read_to_string(&mut output).ok();

    println!("{}", output);
}
