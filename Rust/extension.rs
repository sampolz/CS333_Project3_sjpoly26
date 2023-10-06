/* 
Albert Astrom and Sam Polyakov
CS333
Project 3
extension.rs
*
TO COMPILE: rustc extension.rs
TO RUN: ./extension
*
Exploring whether functions are a basic data type in Rust
*/

fn add_one(x: i32) -> i32 {
    // adds one to an int
    x + 1
}

fn multiply_by_two(x: i32) -> i32 {
    // multiplies an int by 2
    x * 2
}

fn apply(func: fn(i32) -> i32, x: i32) -> i32 {
    // takes in a function and an int, applies that function to the int
    func(x)
}

fn main() {
    let my_function: fn(i32) -> i32 = add_one;

    let five = my_function(4);
    println!("five: {}", five); 

    let six = apply(add_one, 5);
    println!("six: {}", six); 

    let eight = apply(multiply_by_two, 4);
    println!("eight: {}", eight);
}
