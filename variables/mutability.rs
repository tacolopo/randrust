fn main() {
    let x = 5;
    println!("The value of x is {x}");
    let x = x + 1;
    {
        let x = x * 2;
        println!("Inner x = {x}");
    }
    println!("outer x = {x}")
}
//you aren’t allowed to use mut with constants. Constants aren’t just 
//immutable by default—they’re always immutable.
//    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
