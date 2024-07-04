use std::fs::File;
use std::io::{ErrorKind, Read};

#[allow(unused, dead_code)]
enum Result<T, E> {
    Ok(T),
    Eee(E),
}

#[allow(unused)]
fn main() {
    println!("Hello, world!");
    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    let greeting_file_result = File::open("src/hello.txt");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("src/hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("problem creating file: {:?}",e),
            },
            other => {
                panic!("problem opening file: {:?}",other )
            }
        },
    };

    // let mut greeting_file = File::open("hello.txt").unwrap_or_else(|error | {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error|{
    //             panic!("Unable to create the file {}", error);
    //         })
    //     } else {
    //         panic!("Problem opening file {}", error);
    //     }
    // });
    

    //read contents----
    let mut contents = String::new();

    let read = match greeting_file.read_to_string(&mut contents) {
        Ok(sz) => {
            println!("Contents of the file:");
            println!("{} and size {}", contents, sz);
            
        }
        Err(e) => {
            println!("Error reading file: {:?}", e);
        }
    };   



}
