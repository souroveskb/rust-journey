use chap_7::eat_at_restaurant;
use std::collections::*;
use crate::garden::vegetables::Asparagus;
pub mod garden;

fn main() {
    let plant = Asparagus{};
    println!("plant--> {:?}", plant);
    eat_at_restaurant();

    let mut map = HashMap::new();

    map.insert("v", 1);

    println!("Map {:?}", map);
}
