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