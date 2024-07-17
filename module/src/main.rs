use crate::garden::vege::Asparagus;

pub mod garden;


fn main() {
    println!("Hello, world!");
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
