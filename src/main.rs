use std::io;
use std::io::Read;
use std::fs::File;

extern crate learn_error_handling;

fn main(){
    println!("{}",learn_error_handling::read_username_from_file())
}





