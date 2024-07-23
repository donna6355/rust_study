use std::collections::HashMap;


fn hash_map(){
    //create hash map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue",10));
    scores.insert(String::from("Yellow"),50);

    //overwriting
    scores.insert(String::from("Blue"),30);
    //adding only if a key isn't present
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    //updateing over old value
    let text = "hello world snow world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}"); //{"world": 2, "hello": 1, "snow": 1}

    //access hash map value
    for (key, value) in &scores {
        println!("{key} : {value}");
        // Yellow: 50
        // Blue: 10
    }


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}