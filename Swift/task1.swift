/* 
Albert Astrom and Sam Polyakov
CS333
Project 3
task1.swift
*
TO RUN: swift task1.swift
*/

// 1. Identifier naming

let aVariable10000 = "valid" // Can include underscores within or at the beginning of the identifier
print(aVariable10000)
let _under_score_variable = "valid" // Can start with an underscore and have one within the identifier.
print(_under_score_variable)

// let 10000variable = "not valid" // Can't start with a number.

let 👤 = "user" // Can use emoji as identifiers
print(👤)
let θ = 90 // and any unicode character
print(θ)

// 2. Variable declarations:

// Variables are mutable types using the 'var' keyword. 
var mutable = "changeable"
print("mutable: \(mutable)")

mutable = "changed"
print("mutable: \(mutable)")

// Constants are immutable types that are declared using the 'let' keyword.
let immutable = "unchangeable"
// immutableVariable = "unchanged" // This is not allowed and would cause an error.

// 3. Identifier scoping:
// The visibility of an identifier depends on where it is declared.

// Function level scope
func scopeTest() {
    let funcVar = "I am visible only within this function."
    print(funcVar)
    // Global level scope
    print(mutable) // Global variables are visible inside functions.
}

// print(localVariable)  // This would be an error, as localVariable is not visible here.

scopeTest()

// Block level scope
if true {
    let blockVar = "visible"
    print(blockVar)
}

// print(blockVar)  // This would be an error, as blockScopedVariable is not visible here.


