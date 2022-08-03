pub fn run() {
    greeting("hello", "Jane");

    //bind function values to variable
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    //Closure (can use outside variables)
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("CSum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    //no semicolon = return
    n1 + n2
}