#![allow(unused)]

use core::num;
use std::{io, net};
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

    println!("matching func called {}", match_ip(IpAddrKind::V4(String::from("102"))));

    let x = plus_one(absent_num);
    println!("plus one called on none: {}", match x {
        Some(x) => x,
        other => 0,
    });

    let x: Option<i32> = plus_two(some_num);
    println!("plus two called on somenum: {}", match x {
        Some(x) => x,
        other => 0,
    });
}

fn route(ip_kind: IpAddrKind){
    println!("{:?}", ip_kind);
}

fn match_ip(ip_kind: IpAddrKind) -> u8 {
    match ip_kind {
        IpAddrKind::V4(strs) => 4, 
        IpAddrKind::V6(strs) => 6,
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        None => None,
        Some(num) => Some(num+1),
    }
}

fn plus_two(num: Option<i32>) -> Option<i32> {
    if let Some(n) = num {
        println!("Added two {}", n);
        Some(n+2)
    } else{
        None
    }
}