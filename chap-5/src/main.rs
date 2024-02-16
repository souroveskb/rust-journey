

struct User{
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

struct Color (i32, i32, i32);
struct Point (f32, f32);

// struct AlwaysEqual;  // unit like structs used to implement traits

fn build_user (user_name: String, email: String) -> User{
    User {
        active: true,
        user_name: user_name,
        email: email,
        sign_in_count: 1,
    }
}

fn build_user_without_repetition (user_name: String, email: String) -> User {
    User {
        active: true,
        user_name, 
        email,
        sign_in_count: 1,
    }
}

fn main() {

    let mut user1 = User {
        active: true,
        user_name: String::from("souroveskb"),
        email: String::from("souroveskb@gmail.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("sourovesakib@gmail.com");  //to change this the entire user instance has to be mutable

    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    let mail = user1.user_name;
    println!("username {mail}");

    let user1 = build_user_without_repetition(
        String::from("sourooov.com"),
        String::from("emailc.om")
    );
    let mail = user1.email;
    println!("Hello there user {mail}");

    let user2 = User{
        email: String::from("user2@gmail.com"),
        ..user1
    };

    let mail = user2.sign_in_count;
    println!("Signin count user2 {mail}");

    let mail = user2.active;
    println!("Signin count user2 {mail}");

    let blue = Color(0, 0, 255);
    let origin = Point(0.0,0.0);
    let first = blue.2;
    let xpoint = origin.0;
    dbg!(xpoint);

    println!("{first} and origin {xpoint}");
    let rect1 = (2,3);
    
    println!("Area of rectangle with h*w=2*3 is {}", area_tuple(rect1));

    let rect1 = Rectangle{height: 2, width: 4};

    println!("Area of rect as struct {} of rect specs {:#?}", area_struct(&rect1), rect1);

    let rect2 = Rectangle {
        // width: dbg!(30 * scale),
        width: dbg!(20* 3),
        height: 50,
    };

    dbg!(&rect1);

    println!("Area of rect as method {} of rect specs {:#?}", rect1.area(), rect1);

    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));

    let sq = Rectangle::square(4);

    println!("Square sides {},{}", sq.height, sq.width);

}

fn area_tuple(dimension: (u32, u32)) -> u32{
    dimension.0 * dimension.1
}

fn area_struct(rect: &Rectangle) -> u32{
    rect.height * rect.width
}
