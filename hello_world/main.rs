struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_method: u64,
}
#[derive(Debug)] // add this for debug
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { //short for self: &Self
        self.width * self.height
    }

    fn can_hold(&self, other:&Rectangle)->bool{ //with more parameters 
        self.width > other.width && self.height > other.height
    }

    fn square(size:u32)->self{//constructor
        Self{
            width:size,
            height:size,
        }
    }//let square = Rectangle::square(10);
}
fn main() {
    println!("Hello, world!");
    let square = Rectangle::square(10);

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

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    //refatoring with tuple
    let rect1 = (30,50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    //refactoring with struct
    let rect2 = Rectangle { 
        width:20,
        height:15,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(rect2),
    )

    println!("rect2 is {}", rect2);//error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`

    println!("rect2 is {rect2:?}");
    dbg!(&rect2);
    println!(
        "The METHOD area of the rectangle is {} square pixels.",
        rect2.area(),
    )

}

fn build_user(email:String, username:String) -> User {
    User {
        active: true,
        username, //username: username,
        email, //email: email,
        sign_in_count: 1,
    }
}

fn area(width:u32, height:u32)->u32{
    width * height
}

fn area_tuple(dimesions:(u32,u32))->u32 {
    dimesions.0 * dimesions.1
}

fn area_struct(rectangle:&Rectangle)->u32 {
    rectangle.width * rectangle*height
}