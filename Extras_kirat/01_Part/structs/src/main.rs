// struct

// 1)
// struct User {
//     first_name: String,
//     last_name: String,
//     age: i32,
//     signed_in: bool
// }

// fn main() {
//     let user = User {
//         first_name: String::from("Sergio"),
//         last_name: String::from("Ramos"),
//         age: 35,
//         signed_in: false
//     };
//     println!("user = {}", user.first_name);
//     println!("user = {}", user.last_name);
//     println!("user age = {}", user.age);
//     println!("user logged in = {}", user.signed_in);
// }

// 2)
struct Rect {
    weight: u32,
    height: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        self.weight * self.height
    }
    fn perimeter(&self) -> u32 {
        2 * (self.height + self.weight)
    }
    // static function:
    fn debug() -> u32 {
        return 1;
    }
}
fn main() {
    let rec1 = Rect{
        weight: 30,
        height: 50,
    };
    println!("Area of the rectangle is {}", rec1.area());
    println!("Perimeter of the rectangle is {}", rec1.perimeter());
    println!("debug is {}", Rect::debug());
}