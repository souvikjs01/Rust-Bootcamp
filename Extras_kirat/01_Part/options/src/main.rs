// options: { Some, None }
/* Option  look like:
enum Option {
    Some(i32),
    None
}
*/


fn main() {
    let index = find_first_a(String::from("preeti"));
    match index {
        Some(value) => println!("index {}", value),
        None => println!("a not found!"),
    }
}
fn find_first_a(s: String) -> Option<i32> {
    for (ind, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(ind as i32);
        }
    }
    return None;
}