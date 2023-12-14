// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }



// STATEMENTS AND EXPRESSIONS

// fn main() {

//     // expressions vs statements
//     // statements are instructions that perform some action and do not return a value
//     // expressions evaluate to a resulting value
//     // let x = (let y = 6); // this is not allowed
//     // let x = 5; // this is allowed
//     // expressions do not include ending semicolons

//     let y = {
//         let  x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");

//     let z = {
//         let x = 5;
//         x
//     };
//     println!("The value of z is: {z}");
// }


// FUNCTION WITH RETURN VALUE

fn five() -> i32 {
    10000000
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");
}