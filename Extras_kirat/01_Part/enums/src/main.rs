// enums:

// enum Direction {
//     North,
//     East,
//     South,
//     West,
// }
// fn main() {
//     let my_dir = Direction::North;
//     println!("{}", my_dir);
//     // move_around(my_dir);
// }
// fn move_around(direction: Direction) {
//     // implement the logic with single direction:
//     println!("My direction = {:?}", direction);
// }

enum Shape {
    Square(f32),    // length
    Rectangle(f32, f32),  // width, height
    Circle(f32),    // radius
}
fn main() {
    let rect = Shape::Rectangle(3.0, 6.0);
    let circle = Shape::Circle(7.0);
    let square = Shape::Square(4f32);

    let area1 = calculate_area(rect);
    let area2 = calculate_area(circle);
    let area3 = calculate_area(square);

    println!("rectangle = {}", area1);
    println!("circle = {}", area2);
    println!("square = {}", area3);
}
// pattern matching:
fn calculate_area(shape: Shape) ->f32 {
    let area = match shape {
        Shape::Rectangle(a, b) => a*b,
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Square(l) => l * l,
    };

    return area;
}