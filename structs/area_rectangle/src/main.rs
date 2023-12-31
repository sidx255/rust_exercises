struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect =Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect is {:?}", rect); // {:?} is a debug print

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
