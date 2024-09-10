// fn main() {
    // let apple: i32 = 10;
    // println!("apple = {}", apple);

    // // let apple: i32 = 12; 
    // let apple: i32 = apple + 20;   
    // println!("apple = {}", apple);

    // let apple: bool = true;    
    // println!("apple = {}", apple);
// }

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
