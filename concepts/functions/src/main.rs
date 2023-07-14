fn main() {
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1 // No semicolon here, this is an expression
    };

    println!("The value of y is: {y}"); // y is 4

    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // not allowed: x + 1;
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
