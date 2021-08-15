pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    //Basic Format
    println!("{} is from {}", "Brad", "Mass");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "Code");

    //Named Arguments
    println!("{name} likes to play {activity}", name = "John", activity = "Basketball");

    //Placeholder Traits
    println!("Binary {:b} Hex {:x} Octal: {:o}", 10, 10, 10);

    //Placeholder for Debug Trait
    println!("{:?}", (12, true, "hello"));

    //Basic Math
    println!("10 + 10 = {}", 10 + 10);
}