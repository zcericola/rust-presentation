
/*
Functions
-declared using the fn keyword, you must declare the types of the parameters as well as the return type that follows the -> symbol

 */


//this is ok to do, rust will implicitly return expressions. If you put a semi-colon a the end of the line, it becomes a statement and then you need a return keyword.
//similar to using short hand with fat arrow functions in JavaScript
fn add(x: i32, y: i32) -> i32 {x + y}


//Here's an example of simple control flow, works pretty much as you would expect in other languages.
// fn is_old_enough(age: u8) -> bool {
//     const DRIVING_AGE: u8 = 16;
//     if age >= DRIVING_AGE {
//         return true;
//     } else {
//         return false;
//     }

// }

pub fn run(){
    let result: i32 = add(5, 5);
    println!("The result is: {}", result);
    // if is_old_enough(12) {
    //     println!("The person is old enough to drive.");
    // } else {
    //     println!("Uh oh, watch out!");
    // } 
}

//Order: add, is_old_enough