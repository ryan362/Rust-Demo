// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello ");

    //Get Length
    println!("Length: {}", hello.len());

    //Push Char
    hello.push('W');

    //Push String
    hello.push_str("orld!");

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Check if empty string
    println!("Is empty: {}", hello.is_empty());

    //Contains
    println!("Contains 'World': {}", hello.contains("World!"));

    //Replace
    println!("Replace 'World': {}", hello.replace("World", "There"));

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //Create String with Capacity
    let mut strng = String::with_capacity(10);
    strng.push('a');
    strng.push('b');

    println!("{}", strng);

    //Assertion testing
    assert_eq!(2, strng.len());
    assert_eq!(10, strng.capacity());

    println!("{}", hello);
}