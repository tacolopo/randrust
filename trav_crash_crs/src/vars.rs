// Variables hold data or references to data
// Variables are immutable by default
// Rust is a blockscoped language

pub fn run() {
    let name = "Brad";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    //define const
    const ID: i32 = 001;
    println!("ID is: {}", ID);

    //Assign multiple variables at once
    let (my_name, my_age) =  ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}