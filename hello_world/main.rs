struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_method: u64,
}
fn main() {
    println!("Hello, world!");

    let isaac = User {
        active : true,
        username : String::from("Isaac"),
        email : String::from("isaac@meow.com"),
        sign_in_method: 1,
    }

    // let moomin = User {
    //     active : isaac.active,
    //     username : String::from("Moomin"),
    //     email : isaac.email,
    //     sign_in_method : isaac.sign_in_method,
    // }

    let moomin = User {
        username : String::from("Moomin"),
        ..isaac //remaining fields not explicitly set should have the same value as the fields in the given instance
        //no longer use isaac as a whole after creating moomin because the String in the username field of isaac was moved into moomin
    }

    println!(isaac.email);

}

fn build_user(email:String, username:String) -> User {
    User {
        active: true,
        username, //username: username,
        email, //email: email,
        sign_in_count: 1,
    }
}