// Loops - Used to iterate until a condition is met

pub fn run() {
    let mut count = 0;

    //Infinite Loop
    loop {
        count += 1;
        println!("Number: {}", count);

        if count == 20 {
            break;
        }
    }

    //While Loop (FizzBuzz)
    while count <= 100 {
        let mut x = String::from("");
        if count % 3 == 0 {
            x.push_str("Fizz");
        }
        if count % 5 == 0 {
            x.push_str("Buzz")
        }
        count += 1;
        if x.is_empty() == false {
            println!("{}", x);
        } else {
            println!("{}", count);
        }
    }

    //For Range
    for count in 0..100 {
        let mut x = String::from("");
        if count % 3 == 0 {
            x.push_str("Fizz");
        }
        if count % 5 == 0 {
            x.push_str("Buzz")
        }
        if x.is_empty() == false {
            println!("{}", x);
        } else {
            println!("{}", count);
        }
    }
}