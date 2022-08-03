// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    //Get single value
    println!("Single value: {}", numbers[0]);

    //Reassign a value
    numbers[2] = 20;
    println!("{:?}", numbers);

    //Get array length
    println!("Array length: {}", numbers.len());

    //Arrays are stack allocated (get memory size)
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice); 
}