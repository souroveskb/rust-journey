use std::fs::File;
use std::io::{ErrorKind, Read};

#[allow(unused)]
fn main() {
    println!("Hello, world!");
    let demo_file = File::open("../data/demo.txt");

    let mut contents = String::new();

    let mut greeting_file = match demo_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("../data/demo.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating file: {:?}",e),
            },
            other => {
                panic!("problem opening file: {:?}",other )
            }
        },
    };


    let read = match greeting_file.read_to_string(&mut contents) {
        Ok(sz) => {
            println!("Contents of the file:");
            contents
        }
        Err(e) => {
            println!("Error reading file: {:?}", e);
            String::from("hagu")
        }
    };

    println!("{:?}", read);
}
