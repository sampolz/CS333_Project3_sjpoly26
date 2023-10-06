/* 
Albert Astrom and Sam Polyakov
CS333
Project 3
task2.rs
*
TO COMPILE: rustc task3.rs
TO RUN: ./task3
*/

// Define a struct
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // Integers
    let intA: i32 = 10;
    let intB: i64 = 20;

    // Floats
    let floatC:f32 = 10.5;
    let floatD:f64 = 20.5;

    // Boolean
    let boolE = false;

    // Char
    let charF = 'A';

    // String 
    let strG = String::from("Test String, ");

    // Array 
    let arrH = [1, 2, 3];

    // Tuple 
    let tupleI = (1, 1.0, 'a');

    // struct
    let pointJ = Point { x: 1, y: 2 };

    println!("a + b = {}", intA + intB);
    println!("c - d = {}", floatC - floatD);
    println!("a * c = {}", intA as f64 * floatC); // need to convert int to float
    println!("d / c = {}", floatD / floatC); 
    println!("a % 3 = {}", intA % 3);

    println!("e && true = {}", boolE && true);
    println!("e || false = {}", boolE || false);
    println!("!e = {}", !boolE); 

    println!("a == b: {}", intA == intB);
    println!("a != b: {}", intA != intB); 
    println!("a < b: {}", intA < intB);
    println!("a > b: {}", intA >= intB);

    println!("Character: {}", charF);
    println!("String: {}", strG);

    let combined = strG + &charF.to_string(); // combining string and c
    println!("Combined: {}", combined);

    println!("Array element h[0]: {}", arrH[0]);
    println!("Tuple element i.0: {}", tupleI.0);
    println!("Point coordinates: ({}, {})", pointJ.x, pointJ.y);
}
