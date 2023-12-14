// fn main() {
//     let number = 3;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }


// It’s also worth noting that the condition in this code must be a bool. If the condition isn’t a bool, we’ll get an error. 

// fn main() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }



// IF IS AN EXPRESSION AND CAN BE USED ON THE RIGHT SIDE OF A LET STATEMENT

// fn main() {
//     let condition = false;
//     let number = if condition { 5 } else { 6 };

//     println!("The value of number is: {number}");
// }


// LOOP




// fn main() {
//     loop {
//         println!("again!");
//     }
// }


// RETURNING VALUES FROM LOOPS

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 100000000 {
//             break counter * 2;
//         }
//     };

//     println!("The result is: {result}");
// }


// IF WE HAVE NESTING IN LOOPS , CONTINUE AND SKIP APPLY ON INNNERMOST ONE 
// You can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop. Loop labels must begin with a single quote. Here’s an example with two nested loops:

fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    // CONDITIONAL LOOPS WITH WHILE
    let mut number = 3;
    while number != 0 { 
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");


    // LOOPING THROUGH A COLLECTION WITH FOR 
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    
}