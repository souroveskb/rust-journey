fn main() {
    let x = 5;
    let x = x+1;  //shadowing

    {
        let x = x*2;
        println!("the {x}");
    }

    println!("the {x}");

}
