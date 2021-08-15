// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Brad";
    let mut age = 37;

    age += 1;

    println!("My name is {} and I am {}", name, age);

    //Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign Multiple Vars at Once
    let (my_name, my_age) = ("Nick", 9);
    println!("{} is {}", my_name, my_age);
}