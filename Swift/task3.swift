/* 
Albert Astrom and Sam Polyakov
CS333
Project 3
task3.swift
*
TO RUN: swift task3.swift
*/

// Types:

// 1. Integers
let integer: Int = 10

// 2. Float
let flo: Float = 1.65

// 3. Double
let dub: Double = 1.658943123

// 4. Booleans 
let bool: Bool = true

// 5. Strings
let str: String = "String!"

// 6. Characters 
let char: Character = "C"

// Operators:
let sum = integer + 5           // Addition
let difference = integer - 5    // Subtraction
let product = integer * 2       // Multiplication
let quotient = integer / 2      // Division
let remainder = integer % 3     // Mod

// Mixing types
let mixed = Float(integer) + flo
print(mixed, type(of: mixed))

// Compound Assignment Operators:
var compound = 10
compound += 5
compound -= 5
compound *= 2
compound /= 2
compound %= 3


// Aggregate Types:

// 1. Tuples
let tuple: (String, Int, Bool) = ("Sam", 20, true)

// 2. Structures
struct Car {
    var make: String
    var model: String
    var year: Int
    var isSUV: Bool
}

let myCar = Car(make: "Toyota", model: "Prius", year: 2012, isSUV: false)

// 3. Classes
class Card {
    var suit: String
    var value: Int

    init(suit: String, value: Int) {
        self.suit = suit
        self.value = value
    }
}

let myCard = Card(suit: "Spades", value: 10)

print(sum, difference, product, quotient, remainder)
print(tuple)
print(myCar.make, myCar.model, myCar.year, myCar.isSUV)
print(myCard.suit, myCard.value)
