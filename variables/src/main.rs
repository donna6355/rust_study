fn main() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");

    let spaces = "   ";
    let spaces = spaces.len();

    //Floating point types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    //Numeric Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    //Boolean
    let t = true;
    let f: bool = false; // with explicit type annotation

    //Character !!Single Quote!!
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);//destructuring

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    //Array
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1,.2,3,4,5];
    let a = [3; 5]; // [3, 3, 3, 3, 3];

    let first = a[0];
    let second = a[1];

    another_function();
    print_labeled_measurement(5, 'h');

    let z = 5; //statement

    let y = {//expression
        let x = 3;
        x + 1 //Expressions do not include ending semicolorns.
    };

    //Control Flow
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
    let mut counter = 0;

    let result = loop { // returning values from loops
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //Loop Label

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //Conditional Loops with while

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //Looping through a collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
    for number in (1..4).rev() { //to reverse the range
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let sample = String::from("Hello");
    let len = calculate_length(&sample);
    println!("The length of '{sample}' is {len}.");

    let mut sample_two = String::from("Good");
    change(&mut sample_two);
    println!("this is changed string '{sample_two}'");

    let mut you = String::from("Isaac");
    let you_one = &you;
    let you_two = &you;
    println!("{you_one} is {you_two}");
    // variables you1 and you2 will not be used after this point

    let you_three = &mut you; //therefore no problem
    println!("this is {you_three}");

    let reference_to_nothing = dangle();
}   


fn another_function() {
    println!("Another function.");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
fn five() -> i32 {//return value type
    5
}

fn calculate_length(s:&String) -> usize {
    s.len()
}

fn change(some_string:&mut String) {
    some_string.push_string(", bye~!");
}

fn dangle()-> &String {// dangle returns a reference to a String
    let s = String::from("hi"); // s is a new String
    &s // we return a reference to the String, s
}// Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!