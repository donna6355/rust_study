fn main() {
    println!("Hello, world!");

    //WHAT PROBLEM SOVED BY SLICE TYPE
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!


    //STRING SLICE reference to a port of string
    let hello = &s[0..5]; // &s[..5];
    let world = &s[6..11]; // &s[6..];
    
    let len = s.len();

    let slice = &s[0..len]; // let slice = &s[..];
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //convert to an array of bytes using the as_bytes method

    for (i, &item) in bytes.iter().enumerate() {//enumerate returns a tuple and destructure tuple
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn slice_first_word(s: &String) -> &str {
// fn slice_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]

}
