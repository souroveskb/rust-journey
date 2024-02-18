
mod back_of_the_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            // println!()
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("Mangoes"),
            }
        }
        pub fn get_seasonal_fruit(&self) -> String{
            self.seasonal_fruit.to_string()
            // &self.seasonal_fruit //for &str val
        }
    }
}


pub fn eat_at_restaurant() {
    
    let mut meal = back_of_the_house::Breakfast::summer("Moyda");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let fruit = meal.get_seasonal_fruit();
    println!("Fruit is {}", fruit);
}