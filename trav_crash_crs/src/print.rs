pub fn run() {
     //print to console
     println!("Hello from the print.rs file"); 

     //basic formatting
     println!("{} is from {}", "Brad", "Mass");

     //positional arguments
     println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

     //named arguments
     println!("{name} likes to play {activity}", name = "John", activity = "baseball");

     //Placeholder Traits
     println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

     //Placeholder for debug trait
     println!("{:?}", (12, true, "hello"));

     //Basic math
     println!("10+10 = {}", 10+10);
}