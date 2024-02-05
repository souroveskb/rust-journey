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

}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
