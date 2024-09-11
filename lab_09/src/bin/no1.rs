use std::mem::{size_of, size_of_val};

#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
enum Shape {
    Circle(Point, f32),
    Rectangle(Point, Point),
}

fn main() {
    let point = Point { x: 1, y: 2 };
    let circle = Shape::Circle(Point { x: 1, y: 2 }, 5.5);
    let rectangle = Shape::Rectangle(Point { x: 5, y: 6 }, Point { x: 2, y: 3 });

    println!("Size of 'Point':{} bytes", size_of::<Point>());
    println!("Size of 'Point':{} bytes", size_of_val(&point));

    println!("Size of 'Shape':{} bytes", size_of::<Shape>());
    println!("Size of 'Circle':{} bytes", size_of_val(&circle));
    println!("Size of 'Rectangle':{} bytes", size_of_val(&rectangle));
}
