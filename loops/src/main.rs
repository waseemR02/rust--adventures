fn loop_type_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn loop_type_while() {
    let mut counter = 3;

    while counter != 0 {
        println!("count: {counter}");
        counter -= 1;
    }

    println!("LIFTOFF!!!");
}

fn disambiguating_loop() {
    let mut count = 0;

    'counting_up: loop {
        println!("counting up: {}", count);

        let mut remaining = 2;
        loop {
            println!("remaining: {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 5 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

fn loop_type_loop() {
    let mut counter = 1;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {result}");
}

fn main() {
    // loop_type_loop();
    // disambiguating_loop();
    // loop_type_while();
    loop_type_for();
}
