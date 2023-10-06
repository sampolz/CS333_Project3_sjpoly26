CS333 - Project 3: Swift - README
Albert Astrom & Sam Polyakov
10/04/2023
Google Sites URL: https://sites.google.com/colby.edu/sjpoly26-cs333/home/project-3
Extension: Additional language (Rust) AND explored whether functions are basic data types in Rust 

Directory Layout:
.
├── C tasks
│   ├── cstk.c
│   ├── cstk.h
│   ├── cstktest
│   └── cstktest.c
├── README.txt
├── Rust
│   ├── extension
│   ├── extension.rs
│   ├── task1
│   ├── task1.rs
│   ├── task2
│   ├── task2.rs
│   ├── task3
│   └── task3.rs
└── Swift
    ├── task1.swift
    ├── task2.swift
    └── task3.swift

Run and build configuration for Swift and Rust:
Apple Swift version 5.9 (swiftlang-5.9.0.128.108 clang-1500.0.40.1)
Target: arm64-apple-darwin23.0.0

rustc 1.72.1 (d5c2e9c34 2023-09-13)

C:
C portion of project 3

cstk:
TO TEST:
gcc -o cstktest cstktest.c cstk.c
./cstktest

Output:
The original list: 0 1 2 3 4 5 6 7 8 9 
The reversed list: 9 8 7 6 5 4 3 2 1 0 

Swift:
Task 1:
TO RUN: swift task1.swift

Output:
valid
valid
user
90
mutable: changeable
mutable: changed
I am visible only within this function.
changed
visible

Task 2:
TO RUN: swift task2.swift

Output:
Found at index: 2
Not found.

Task 3:
TO RUN: swift task3.swift

Output:
11.65 Float
15 5 20 5 1
("Sam", 20, true)
Toyota Prius 2012 false
Spades 10


EXTENSION 1 - RUST LANGUAGE:
Rust:

Task1:
TO COMPILE: rustc task1.rs
TO RUN: ./task1

Output:
Inner variable: 60
my_variable: 70
my_variable: 10
MyVariable: 30
my_long_variable_name: 40
my_variable: 10
my_mutable_variable: 20
my_mutable_variable: 30


Task2:
TO COMPILE: rustc task2.rs
TO RUN: ./task2

Output:
Element found at index 7


Task3:
TO COMPILE: rustc task3.rs
TO RUN: ./task3

Output:
a + b = 30
c - d = -10
a * c = 105
d / c = 1.9523809523809523
a % 3 = 1
e && true = false
e || false = false
!e = true
a == b: false
a != b: true
a < b: true
a > b: false
Character: A
String: Test String, 
Combined: Test String, A
Array element h[0]: 1
Tuple element i.0: 1
Point coordinates: (1, 2)

Extension 2 - Functions as basic data types:
TO COMPILE: rustc extension.rs
TO RUN: ./extension

Output:
five: 5
six: 6
eight: 8

Results:
In Rust can be assigned to variables, passed as arguments to other functions, and returned from functions. 
So, functions are  a basic data type, and a variable can hold an arbitrary function as long as the function's type signature matches the variable's type.
