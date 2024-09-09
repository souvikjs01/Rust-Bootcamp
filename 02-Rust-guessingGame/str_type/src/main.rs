fn main() {
    // str: static string read only memory
    let str_literal: &str = "Hi there, how are you!!!";
    println!("The str literal is {}", str_literal);

    //String: dynamic string heap allocation memory
    let mut string_literal: String = String::from("Hey, bro!");
    string_literal.push_str(" what's up");
    println!("the string literal {}", string_literal);
}
