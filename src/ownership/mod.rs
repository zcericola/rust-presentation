/*
Ownership and the Borrow Checker
The pièce de résistance of Rust
-Every program needs a way to manage memory while running.
- Languages like JS, Java, and Python are Garbage Collected. 
    -Pros: user doesn't have to deal with freeing up memory when a variable is no longer in use
    -Cons: The language requires more memory at runtime to keep track of all the objects behind the scenes

- Lower level languages like C, C++require the programmer to explicitly allocate memory and then later on free it up
    -Pros: Fine grain control that makes it possible to achieve very quick programs that can run on embedded systems on a fixed amount of memory (Good for games, smart devices, etc)
    -Cons: Easy to manage memory poorly and cause crazy bugs or unexpected behavior

-- Rust takes a third approach. It has a strict set of rules around memory management that are checked at compile time. This way, they ensure memory safety like in applications languages but they also allow for the speed and control of system languages.
    -Pros: Best of both worlds
    -Cons: The borrowing system is hard to wrap your head around at first.

Basic Ownership Rules
- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
 */

 fn ex_one() -> () {
     let x = 5;
     let y = x;
     println!("The values of x and y are: {} and {}", x, y);
 }

// fn ex_two() -> () {
//     //borrow checker in action
//     //This is a problem, they are both pointed to the same address on the heap
//     //Would cause a double free error in C++ where both variables try to de-allocate the same memory space which can lead to memory corruption and security issues
//     //Rust recognizes this and instead dereferences str1 to prevent any issues
//     let str1 = String::from("string one");
//     let str2 = str1;
//     println!("The values of str1 and str2 are: {:?} and {:?}", str1, str2);
// }

// fn references() -> Vec<i32> {
//     //remember, must use the mut keyword here since we are changing the values in the array
//     let mut nums = vec![10, 20, 30, 40];
//     //& operator is how you indicate you need a reference
//     for i in &mut nums {
//         // * is the dereference operator, does the opposite of &.
//         *i *= 2;

//     }
//     return nums;
// }


pub fn run(){
   
    let fav_car = String::from("Porsche"); //fav_car enters scope here
    println!("My favorite car is a {:?}", fav_car);
    ex_one(); //behaves as you would expect
    // ex_two();
    // let result_arr: Vec<i32> = references();
    // println!("The return value of references: {:?}",  result_arr);

}
//fav_car is no longer in scope


//Order: ex_1, ex_2, references