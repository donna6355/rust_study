fn unrecoverable_error() {
    //unrecoverable errors with panic!
	let v = vec![1,2,3];
    v[99];
    //explicitly call panic! macro
	panic!("crash and burn");
}

// enum Result<T, E> {
//     Ok(T), // type of the value
//     Err(E). // type of the error
// }

use std::fs::File;
use std::io::ErrorKind;

fn recoverable_error () {
    //recoverable error with Result<T, E>
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
            Ok(file) => file,
            // Err(error) => panic!("Problem opening the file: {error:?}"),
            Err(error) => match error.kind(){
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {e:!}"),
                },
                other_error => {
                    panic!("Problem opening the file: {other_error:?}");
                }
            }

    }

    //expect is more chosen in production-quality code
    let hello_file = File::open("hello.txt").unwrap(); // for Ok return the value, for Err call the panic!
    let hello_file = File::open("hello.txt").expect("hello.txt should be included in this project"); // include error message with expect
}

//to avoid nested match expressions
fn alternative() {
    let greeting_file_result =
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {e:!}");
            })
        } else {
            panic!("Problem opening the file: {other_error:?}");
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    }
    
    let mut username = String::new();
    
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
fn shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; //to perform an early return of a value out of the function
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

}
fn even_shorter_read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

use std::fs;
use std::io;
fn shortest_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}