
use std::vec;
use std::collections::HashMap;


#[derive(Debug)]
enum Spreadsheet {
    Int(i32),
    Float(f32),
    Text(String),
}


#[allow(dead_code, unused_assignments)]
fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.pop();

    let mut v = vec![1, 2, 3, 4, 5];

    v.push(8);

    let mut third: Option<&i32> = v.get(2);

    third = Some(&8);

    println!("Third num: {}", match third{
        Some(t) => t,
        None => &0,
    });

    let fourth = &v[3];
    println!("Fourth: {}", fourth);


    for i in &v {
        print!("{i} ");
    }

    println!("Hello, world!");


    let row = vec![Spreadsheet::Int(4), Spreadsheet::Float(3.0), Spreadsheet::Text(String::from("Hello"))];

    println!("row: {:?}", row);


    // Strings
    let mut s = String::new();

    let data = "string data";

    s = data.to_string().to_uppercase();

    s.push_str(" Hello");

    s.push('1');

    println!("{:?}", s);

    for c in "hell".bytes() {
        println!("{c}");
    }


    let mut map = HashMap::new();

    map.insert("key1", 1);
    map.insert("key2", 2);

    let key = "key1";

    let found  = map.get(&key).copied().unwrap_or(-1);

    println!("Found key value {}", found);

    for (k, v) in &map {
        println!("{k}: {v}");

    }

    map.entry("key3").or_insert(3);


    let text = "the catch of the double double";

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("After Entry: ");
    for (k, v) in &map {
        println!("{k}: {v}");

    }


    assert_eq!(Some("car").unwrap_or("bike"), "car");  //use of unwrap on options
    // gives the value of Some("--") or if None is passed then gives the default value passed
    // in the unwrap function .unwrap_or("--");
    assert_eq!(None.unwrap_or("bike"), "bike");

}
