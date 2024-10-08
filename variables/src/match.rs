enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin:Coin)->u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => { //use curly braces if to run multiple lines of code in a match arm
            println!("Lucky Nickel");
            5
        },
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }

}

fn catch_all_patterns(){
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other)
        _ => reroll(), //_ is a special pattern that matches any value and does not bind to that value
    }
} 
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn if_let(){
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        (_) => (),
    }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }
    if let Coin::Quarter(state) => coin {
        println!("State quarter from {state:?}!");
    }else{
        count += 1;
    }
}