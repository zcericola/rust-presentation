/*
Data Types
Rust is a statically typed language. It has to know the type of every variable at compile time, in contrast to JavaScript for example which is dynamically typed.


Integers 
i8 -i128/ u8 - u128 signed vs. unsigned
-default to i32 which is fastest

Floating Point Numbers 
f32 - f64
let fp_num = 2.0; //f64 type by default
let fp_num: f32 = 3.0;

Boolean
let isBool: bool = true;

Character
4 bytes in size, this will only hold a single character
let c = 'c';

String
string literal and String type
let s = 'string'; //stored on the stack, fixed size, immutable
let s = String::from("Hello"); //stored on the heap

Compound Types
Tuples
let my_tuple = (i32, f64) = (44, 3.0);
-tuples have a fixed length and can't grow or shrink like an array would

Arrays
let fruit = ['banana', 'apple'];
-arrays operate like tuples in Rust, they must have a fixed length
-arrays are allocated on the stack instead of the heap
-Most of the time if you need an array as you're used to in another language, you probably should use a vector which behaves more like you would assume.

Vectors
let v = vec![10, 20, 30, 40]; //uses the vec macro for easier writing
let v = Vec<i32> = Vec::new(); // creates a new empty vec


*/

fn array_ex(){
    let my_nums = [3, 4, 5, 6, 7];
    //Here's an example of rust's safety features
    //index is greater than the length of the array, so the value can't be read
    //other languages like C++ would allow you to do this and instead return a value from another part of memory which creates hard to debug errors.
    let index = 5;
    // let element = my_nums[index];
}

pub fn run(){
    //This is ok, Rust has type inference and can tell that name is a string.
    let name = "Zac";
    println!("My name is {}", name);

    //The compiler will throw an error here because it doesn't know what to do
    // let age = "29".parse();
    // println!("I am {:?} years old.", age);

    array_ex();
}