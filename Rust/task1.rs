/* 
Albert Astrom and Sam Polyakov
CS333
Project 3
task1.rs
*
TO COMPILE: rustc task1.rs
TO RUN: ./task1
*/


fn main() {
    // comparing mutable and immutable variables
    let my_variable = 10; 
    let mut my_mutable_variable = 20; 
    
    // identifiers are case sensitive, variables should start with a lowercase but can start with uppercase
    let MyVariable = 30;
    
    // Identifier names can have underscores
    let my_long_variable_name = 40;
    
    // Identifier names cannot start with a number
    // let 1variable = 50; 
    
    {
        let inner_variable = 60; // Only accessible within this block
        println!("Inner variable: {}", inner_variable);
        
        let my_variable = 70; // Shadows my_variable
        println!("my_variable: {}", my_variable);
    }
    
    // Cannot do this as inner_variable is out of the blocks scope
    // println!("Inner variable: {}", inner_variable);
    
    println!("my_variable: {}", my_variable);
    println!("MyVariable: {}", MyVariable);
    println!("my_long_variable_name: {}", my_long_variable_name);
    
    // cannot modify nonmutable variables
    //my_variable += 10;
    println!("my_variable: {}", my_variable);

    println!("my_mutable_variable: {}", my_mutable_variable);
    // CAN modify mutable variables
    my_mutable_variable += 10;
    println!("my_mutable_variable: {}", my_mutable_variable);
}
