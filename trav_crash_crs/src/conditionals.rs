// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    let age: u8 = 18;
    let check_id = false;
    let knows_person_of_age = true;

    //if else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: Drink?");
    } else if age < 21 && check_id {
        println!("Too young");
    } else {
        println!("Need to see ID");
    }

    //shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("Is of age: {}", is_of_age);
}