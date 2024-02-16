#![allow(unused)]

use std::io;
#[derive(Debug)]
enum IpAddrKind{
    V4(String), 
    V6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
#[derive(Debug)]
enum Message{
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:#?}", self);
        println!("Calling the call func");
    }
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home = IpAddr{kind:four, address: String::from("127.0.0.1")};
    let mut addr = String::new();
    // io::stdin()
    //     .read_line(&mut addr)
    //     .expect("Failed to read line");

    let addr =  addr.trim();
    // addr.pop();

    // let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let home = IpAddrKind::V4(addr.to_string());
    let loopback = IpAddrKind::V6(String::from("::1"));

    route(home);
    route(loopback);

    // Enum Message
    let m = Message::Write(String::from("Hello there"));
    m.call();

    //null references to enum
    let some_num = Some(5);
    let some_char = Some("c");

    let absent_num: Option<i32> = None;
}

fn route(ip_kind: IpAddrKind){
    println!("{:?}", ip_kind);
}