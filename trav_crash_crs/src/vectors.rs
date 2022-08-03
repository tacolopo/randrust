// Vectors are resizable arrays
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    //Get single value
    println!("Single value: {}", numbers[0]);

    //Reassign a value
    numbers[2] = 20;
    println!("{:?}", numbers);

    //add on to vector
    numbers.push(5);
    numbers.push(6);

    //Get Vector length
    println!("Vector length: {}", numbers.len());

    //pop off last value
    numbers.pop();

    //Vectors are stack allocated (get memory size)
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //get slice
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice); 

    //loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}