// fn main() {
//     // primitive data types are: Array and Tuple:

//     // let arr1: [u8;5]; // array declaration
//     let mut arr1;
//     arr1 = [1,2,3,4,5,6];

//     println!("arr1[0] = {}", arr1[0]);
// }


fn main() {
    let mut arr: [&str; 3] = ["solana", "eth", "btc"];
    write_arr(&mut arr);
    println!("arr = {:?}", arr);
}
// pass by reference:
fn write_arr(arr2: &mut [&str; 3]) {
    arr2[1] = "cardano";
    println!("arr1 = {:?}", arr2);
}

// (pass by value) new copy of the arr is created here:
// fn write_arr(mut arr1: [&str; 3]) {
//     arr1[1] = "metamusk";
//     println!("arr1 = {:?}", arr1);
// }