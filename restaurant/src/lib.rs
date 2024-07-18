pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


//cargo new restaurant --lib; create new library
mod front_of_house{
    //Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant(){
    //absolute path starting crate keyword
    crate::front_of_house::hosting::add_to_waitlist();

    //relative path starting with a module name
    //the name of the module defined at the same level
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order(){}
mod back_of_office{
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();
        //super :: relative paths that begin in the parent module //like starting a filesystem path with the .. syntax
    }
    fn cook_order(){}
    //use pub before a struct definition, make the struct public, but the struct’s fields will still be private
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast{
        pub fn summer(toast: &str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
    //make an enum public, all of its variants are then public
    pub enum Appetizer{
        Soup, 
        Salad,
    }
}

pub fn eat_at_summer_restaurant({
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_office::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please",meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_office::Appetizer::Soup;
    let order2 = back_of_office::Appetizer::Salad;
})