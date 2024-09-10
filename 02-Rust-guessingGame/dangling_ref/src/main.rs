// Dangling reference
fn main() {
    let ref_to_nothing = create_string_ref();  // error
}

fn create_string_ref() ->&String {
    let s: String = String::from("Hello");
    return &s;
}