fn main() {
    let x = 5;
    let x = x+1;  //shadowing

    {
        let x = x*2;
        println!("the {x}");
    }

    println!("the {x}");


    //data types
    let x = 2.0;

    let y: f32 = 2.0;

    let bl: bool = false;


    let emoji: char = '😻';

    println!("{emoji}  {x}  {bl}  {y}");


    //tuple type
    let tup : (i32, f32, u8) = (500, 5.6, 1);

    let (x,y,z) = tup;

    println!("Here are the values of tuple {x}, {y}, {z}");

    //array type
    let a: [i32; 5] = [1, 2, 3, 4, 5]; 
    let b: [&str; 4] = ["a";4];

    println!("array a is {:?} and b is {:?}", a, b);

    another_function(44);

    //let y = 144; //this is a statement
    
    let y = {
        let x = 3;
        x+1
    }; //this is an expression


    println!("The expression of segment makes y= {y}");

    let ans = compare_with_five(5);
    println!("Here I am comparing with 5: {ans}");

    let loop_in_let = returning_result_loop();

    println!("The result loop returned {loop_in_let}");

}

fn another_function(x: i32) {
    let z = with_return_values();
    let k = if compare_with_five(z) {6} else {5}; //using if in a let statement
    println!("The value of x is: {x} and returned value of k {k}");
}

fn with_return_values() -> i32 { // function with return using an expression
    5
}

fn compare_with_five(x: i32) -> bool { //function with control flow
    if x == 5 {
        true
    } else {
        false
    }
}


fn returning_result_loop() -> i32 {
    let mut counter = 0;
    let x = loop {
        counter += 1;

        if counter == 10 {
            break counter *2;
        }
    };
    x
}
