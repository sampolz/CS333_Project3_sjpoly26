/* 
Albert Astrom and Sam Polyakov
CS333
Project 3
task2.swift
*
TO RUN: swift task2.swift
*/
func binarySearch(_ arr: [Int],_ target: Int) -> Int? {
    var low = 0
    var high = arr.count - 1
    
    while low <= high {
        let mid = (low + high) / 2
        
        if arr[mid] == target {
            return mid
        } else if arr[mid] < target {
            low = mid + 1
        } else {
            high = mid - 1
        }
    }
    
    return nil
}

func find(arr: [Int], target: Int) {
    if let result = binarySearch(arr, target) {
    print("Found at index: \(result)")
    return
    } else {
    print("Not found.")
    return
}

}

let present = [1, 2, 3, 4, 5]
let notPresent = [100, 200, 300, 400, 500, 600]

find(arr: present, target: 3)
find(arr: notPresent, target: 3)
