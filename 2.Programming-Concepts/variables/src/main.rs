// variable are immutable by default in rust 
// to make them mutable we need to use mut keyword
// we can't change the type of variable once it is declared
// we can shadow the variable by redeclaring it with let keyword
// shadowing is different from mut keyword

// when using const keyword we need to specify the type of variable
// we can't use mut keyword with const keyword
// naming convention for const is to use all uppercase with underscore between words


// constants are valid for the entire time a program runs, within the scope they were declared in
// constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about

const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x= 7;
    println!("The value of x is: {x}");

    println!("The value of MAX_POINTS from global scope  is: {MAX_POINTS}");


    let y  = 2;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in inner scope is: {y}");
    }
    println!("The value of y in outer scope is: {y}");


}

// shadowing - we can declare a new variable with the same name as a previous variable,
// and the new variable shadows the previous variable
// we can shadow a variable by using the same variable’s name 
//and repeating the use of the let keyword 

//The other difference between mut and shadowing is that because we’re effectively
// creating a new variable when we use the let keyword again,
// we can change the type of the value but reuse the same name.