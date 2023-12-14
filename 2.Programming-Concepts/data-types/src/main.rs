fn main() {

    let guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("The value of guess is: {guess}");
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");


    // NUMERIC OPERATIONS 
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

    println!("The value of sum is: {sum}");
    println!("The value of difference is: {difference}");
    println!("The value of product is: {product}");
    println!("The value of quotient is: {quotient}");
    println!("The value of truncated is: {truncated}");
    println!("The value of remainder is: {remainder}");

    // CHARACTER TYPE
    // char literals are specified with single quotes, as opposed to string literals, which use double quotes
    // char type is four bytes in size and represents a Unicode Scalar Value
    // Unicode Scalar Value ranges from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive
    // Unicode Scalar Value doesn’t include surrogate pair values (U+D800 to U+DFFF)
    // which are used to encode values greater than U+FFFF
    // Rust’s char type is therefore a Unicode Scalar Value, which means you can
    //  create a char literal from a single Unicode Scalar Value by specifying a
    //  Unicode escape sequence
    // Some characters need to be escaped when specified in char literals
    // '\n' - newline
    // '\r' - carriage return
    // '\t' - tab
    // '\\' - backslash
    // '\'' - single quote
    // '\"' - double quote
    // '\xNN' - byte with hexadecimal value NN
    // '\u{NNNNNNNN}' - Unicode scalar value
    // let c = 'z';
    // let z = 'ℤ';
    // let heart_eyed_cat = '😻';

    // compound types can group multiple values into one type
    // Rust has two primitive compound types: tuples and arrays
    let tup : (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // destructuring
    println!("The value of x , y and z are:{x} {y} {z}");

    // we can access a tuple element directly by using a period (.) followed by the index of the value we want to access
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred , six_point_four and one are:{five_hundred} {six_point_four} {one}");


    // ARRAY TYPE  : 
    // unlike a tuple, every element of an array must have the same type
    // arrays in Rust have a fixed length, like tuples
    // arrays are useful when you want your data allocated on the stack rather than the heap
    // or when you want to ensure you always have a fixed number of elements
    let a : [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3;5];
    // println!("The value of a is: {a}"); // a is not a string literal so it cannot be printed directly
    
    
}
