fn string(){
    //creating a new string
    let mut s = String::new();
    let data = "Isaac is cute";
    let st = data.to_string();
    let stri = "Isaac is cute".to_string();
    let strin = String::from("Isaac is cute");

    //updating string
    st.push_str(" XD"); // Isaac is cute XD
    st.push("!") //push single character

    //concatenating string
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3; // note s1 has been moved here and can no longer be used

    let st1 = String::from("tic");
    let st2 = String::from("tac");
    let st3 = String::from("toe");

    let st_all = format!("{st1}-{st2}-{st3}");



    let hello = String::from("Здравствуйте"); // it takes 24 bytes!! 2 bytes per a single character
    //this is the reason Rust not support string slice, indexing
    // let h = st[0];//ERRPR!! Rust strings don’t support indexing
    let h = &st[0..4]; // contains first 4 bytes of the string still can crash program

    //this is the best way
    for c in "Зд".chars() {
        println!("{c}"); //З, д
    }
}