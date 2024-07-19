let v: Vec<i32> = Vec::new(); //type annotation
let v2 = vec![1, 2, 3]; // vec! macro

v.push(4); // add elements

//two ways to read element of vectors
let third: &i32 = &v[2];
let second: Option<&i32> = v.get(1); //better to handle invalid index
match second {
	Some(second) => println!("The second element is {second}"),
	None => println!("NO second element"),
}
let last: &i32 = v.pop(); //remove and return the last element

//iterate over the values in a vector
let v3 = vec![100, 32, 35];
for i in &v3 {
	println!("{i}");
}

let mut v4 = vec![100, 3, 24];
for i in &mut v4 {
	*i += 40;
}

//trick to store multiple types
enum SpreadsheetCell {
	Int(i32),
	Float(f64),
	Test(String),
}

let row = vec![
	SpreadsheetCell::Int(3),
	SpreadsheetCell::Float(10.12),
	SpreadsheetCell::Text(String::from("Isaac"),
];