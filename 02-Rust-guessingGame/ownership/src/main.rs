
// 1)

// fn main() {
//     let str1 = String::from("Hello");
//     let str2 = str1;

//     println!("str2 = {}", str2);
//     // println!("str1 = {}", str1);
// }

// 2)
// fn main() {
//     let x:u8 = 6;
//     print_fun(x);
//     println!("x = {}", x);
// }
// fn print_fun(x:u8){
//     println!("x= {}", x);
// }

// 3)

// fn main() {
//     let x: String = String::from("hello");
//     print_fun(x);
//     // println!("x = {}", x);
// }
// fn print_fun(x:String){
//     println!("x= {}", x);
// }

// 4)
// fn main() {
//     let s1: String = get_string();
//     println!("s1 = {}", s1);

//     let s2:String = String::from("world");
//     let s3:String = send_get_string(s2);
//     println!("s3 = {}", s3);
// }

// fn get_string() -> String {
//     let new_string : String = String::from("hello");
//     return new_string;
// }

// fn send_get_string(received : String) -> String {
//     return received;
// }

// 5)
// fn main(){
//     let s1: String = String::from("hello");
//     let (s2, len) = calculate_len(s1);
//     println!("the length of the string: {} is: {}", s2, len);
// }
// fn calculate_len(s: String) -> (String, usize) {
//     let length = s.len();
//     return (s, length);
// }
// or we can write this way:

// fn main(){
//     let s1: String = String::from("hello");
//     let len = calculate_len(s1.clone());
//     println!("the length of the string: {} is: {}", s1, len);
// }
// fn calculate_len(s: String) -> usize {
//     let length = s.len();
//     return length;
// }

// the best way is:
// fn main(){
//     let s1: String = String::from("hello");
//     let len = calculate_len(&s1);  // borrow operation
//     println!("the length of the string: {} is: {}", s1, len);
// }
// fn calculate_len(s: &String) -> usize {
//     let length = s.len();
//     return length;
// }

// 6)
// fn main() {
//     let mut s1: String = String::from("hello");
//     append_string(&mut s1);
//     println!("The new string is {}", s1);
// }

// fn append_string(s2: &mut String) {
//     s2.push_str(" world");
// }

// 7)
// fn main() {
//     let mut s1: String = String::from("hi there");
//     println!("s1 = {}", s1);

//     let w1 = &mut s1;
//     w1.push_str(" how are you");
//     println!("w1 = {}", w1);

//     let w2 = &mut s1;
//     w2.push_str(" buddy");
//     println!("w2 = {}", w2);

//     println!("s1 = {}", s1);
// }


