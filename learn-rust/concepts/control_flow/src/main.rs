fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    zero_check();

    else_if();

    if_let();

    if_loop();

    loop_label();

    while_loop();

    for_loop();

    for_range();

    for_collection();
}

fn zero_check() {
    let number = 3;

    if number != 0 { // if number {} is not valid
        println!("number was something other than zero");
    }
}

fn else_if() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_let() {
    let condition = true;
    let number : i32 = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {number}");
}

fn if_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {  // 'counting_up: is the label
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // break the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() { // iter() returns each element in a collection
        println!("the value is: {element}");
    }
}

fn for_range() {
    for number in (1..4).rev() { // (1..4) is a range
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}

fn for_collection() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}