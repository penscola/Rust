
// A tuple struct
struct Pair(i32, f32);

// As we know that every corner has a point so we create a point struct
// if we didn't create a point, we will need to use two variables all the time.
struct Point {
    x: f32,
    y: f32,
}

// Creat two points of a rectangle
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn react_area(_rectangle: &Rectangle) {
    println!("Rectangle points\n");
    println!("Point1: ({},{})\nPoint2: ({},{})", _rectangle.p1.x, _rectangle.p1.y, 
    _rectangle.p2.x, _rectangle.p2.y);

    let height = _rectangle.p2.y - _rectangle.p1.y;
    let width = _rectangle.p2.x - _rectangle.p1.x;

    println!("\nArea of rectangle: {}", width * height);
}

pub fn main() {
    println!("Tuples and Structs Challenge");

    println!("\n\nProblem:\nAdd a function rect_area() which calculates the area of the \n rectangle.\n\n");
    println!("We will use the concept of tuples.\n");
    println!("We can call a named tuple as struct.\n");

    // Here is how we initiate a point
    let point1 = Point{x:3.0, y:6.0};
    let point = Point{x:12.0, y:17.0};

    /* Accessing our newly created point */
    println!("point coordinates: ({}, {})", point.x, point.y);

    let _rectangle = Rectangle {
        p1:point1,
        p2:point,
    };

    // We are passing rectangle by reference so that we can destruct it inside of method.
    react_area(&_rectangle);

    println!("\nCreating a PAIR with 1 and 0.1\n");
    let pair = Pair(1, 0.1);
    /* We are accessing values without names bur instead with indexes */
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
//     deconstruct a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}