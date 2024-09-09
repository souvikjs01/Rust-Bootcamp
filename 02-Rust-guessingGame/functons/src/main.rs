const GLOBAL_CONST:u8 = 15;
fn print_fun() {
    println!("hey there");
}

fn add(num1: u8, num2: u8) -> u8 {
    return num1 + num2;
}

fn greet(st: String) -> String {
    let str2 : String = format!("Hello Mr. {}", st);
    return str2;
}

fn main() {
    println!("global value: {}", GLOBAL_CONST);
    print_fun();
    let result: u8 = add(4, 6);
    println!("Result {}", result);
    
    let ans: String = greet("Alex".to_string());
    println!("{}", ans);
}
