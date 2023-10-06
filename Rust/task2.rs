/* 
Albert Astrom and Sam Polyakov
CS333
Project 3
task2.rs
*
TO COMPILE: rustc task2.rs
TO RUN: ./task2
*/

fn bst(arr: &[i32], target: i32) ->  Option<usize> {
    // returns index of a value in a list using binary search
    let mut lowIndex = 0;
    let mut highIndex = arr.len()-1;

    while(lowIndex <= highIndex){
        let mut middleIndex = (lowIndex+highIndex)/2;
        if(arr[middleIndex] == target){
            return Some(middleIndex);
        }
        else if(arr[middleIndex] < target){
            lowIndex = middleIndex+1;
        }
        else{
            highIndex = middleIndex-1;
        }
    }
    None
}

fn main() {
    let arr = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    let target = 15;

    match bst(&arr, target) {
        Some(index) => println!("Found at index {}", index),
        None => println!("Not found"),
    }
}
