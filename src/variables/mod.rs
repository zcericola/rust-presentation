/*
Variables and Immutability

Variables are immutable by default in Rust meaning they can't be altered once they are assigned a value unless you explicitly do it with the mut keyword.
This is a central part of Rust's architecture. When you create a variable that you don't want to change, you can be confident that it won't change.


*/

//Must use the pub keyword (public) in order to use a function outside of its scope
pub fn run(){
    let x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    const MEANING_OF_LIFE: u8 = 42;
    println!("This is a constant, {}", MEANING_OF_LIFE);
    }

