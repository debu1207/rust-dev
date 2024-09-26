// for hiding waring for unused code
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// unit struct
struct unit;

// tuple struct
struct Pair(i32, f32);

// struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Walter");
    let age = 27;
    let walter = Person { name, age };

    // Print debug struct
    println!("{:?}", walter);

    // Instantiate a "Point"
    let p1: Point = Point { x: 10.3, y: 0.4};
    let p2: Point = Point {x: 5.3, y: 0.3};

    // Access the fields of the point
    println!("Point coordiantes: ({}, {})", p1.x, p2.y);

    // Make a new point by using struct update syntax
    let bottom_right = Point { x: 5.2, ..p2 };
    println!("bottom_right: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a 'let' binding
    let Point { x: left_edge, y: top_edge } = p2;
    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}