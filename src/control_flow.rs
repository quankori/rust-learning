pub fn if_else() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let condition = true;
    // If else 2
    let number2 = if condition { 5 } else { 6 };
    println!("{}", number2);
}

pub fn for_loop() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

pub fn while_loop(mut index: usize) {
    let a = [1, 2, 3, 4, 5];
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    // Loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}
