// fn largest_i32(list: &[i32]) -> &i32 {
//     let mut largest = &list[0];
//     for item in list {

//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn largest_char(list: &[char]) -> &char {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main(){
    let number_list = vec![34, 50, 25, 100, 65];
    // let mut largest = &number_list[0];
    // for number in number_list{
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    // println!("The largest number is {largest}");
    // let result = largest_i32(&number_list);
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    // let result = largest_char(&char_list);
    let result = largest(&char_list);
    println!("The largest char is {result}");

}