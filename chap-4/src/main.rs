fn main() {
    let mut s = String::from("Hello ");
    s.push_str("world!");

    let st = "hello world";
    {
        let  st = "nice";
        println!("{st}");
    }
    println!("{}", st);

    {
        let s = String::from("heap String"); // s is valid from this point forward
        println!("{}", s);

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;    // Rust considers s1 as no longer valid. Therefore, Rust doesn’t 
                            // need to free anything when s1 goes out of scope

    println!("{}", s2);


    let s = String::from("ownership");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);   


    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1
    println!("{}", s1);
    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2); 
    println!("{}", s3);

    let (s2, len) = calculate_len(s3);

    println!("The length of '{}' is {}.", s2, len);

    let length = calculate_length(&s2);

    println!("The length is {length}");

    let mut s2 = String::from("hello");     // s2 comes into scope

    change(&mut s2);
    println!("The length of '{}' is", s2);

    let r3 = &mut s2; // no problem
    println!("{}", r3);

    let s = no_dangle();
    println!("{s}");

    let s = String::from("hello world");

    // let hello = &s[0..5]; //string slices
    let world = &s[6..];
    println!("the world {world}");
    // let hello =  &s[0..first_word(&s)];
    let hello = first_word(&s);

    println!("first word {hello}");

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("partial str literal: {word}");

    let word = first_word(&my_string_literal[..]);
    println!("full str literal: {word}");

    let word = first_word(my_string_literal);
    println!("without slices str literal: {word}");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3], "These are not equal!! ");



}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("ownership given"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}


fn calculate_len(s: String) -> (String, usize) {
    let lens = s.len();
    (s, lens)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str("world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            // return i;
            return &s[0..i];
        }
    }

    &s[..]
    // s.len()
}


