// 1)

// // nth fibonachhi:
// fn main() {
//     println!("{}", fib(4));
// }
// fn fib(num : u32) -> u32 {
//     let mut first = 0;
//     let mut second = 1;
//     if num == 0 {
//         return first;
//     }
//     if num == 1 {
//         return second;
//     }
//     for _ in 0..(num-1) {
//         let temp = second;
//         second += first;
//         first = temp;
//     }
//     return second;
// }

// 2) return length of a string:
fn main() {
    let name: String = String::from("Brooklyn_new_york");
    println!("length of name is = {}", find_len(name));
}
fn find_len(s: String) -> usize {
    return s.len();
}