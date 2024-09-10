fn main() {
    // it false age will compute there at run time, we don't know what the value of age at compile time 
    // let age: i32 = 29;

    const AGE: u32 = 29;
    println!("value of age : {}", AGE);

    const SECONDS: u32 = 3*60*60 + AGE;
    println!("{}", SECONDS);
}
