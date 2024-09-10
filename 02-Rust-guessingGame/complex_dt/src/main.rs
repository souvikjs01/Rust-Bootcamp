// vector:
// fn main() {
//     // let mut v = Vec::<i32>::new();
//     let mut v: Vec<i32> = Vec::new();
//     v.push(10);
//     v.push(30);
//     v.push(20);
//     v.push(40);
//     v.push(50);

//     println!("vector v = {:?}", v);

//     let mut v2 = vec![1,2,3,4,5];
//     v2.push(7);
//     v2.pop();
//     println!("vector v2 = {:?}", v2)   
// }


// ** vector store in heap so ownership apply here:
// fn main() {
//     let vrr: Vec<&str> = vec!["sol", "eth", "pol"];
//     write_arr(vrr);  // ownership transfer
//     println!("varr = {:?}", vrr);  // error here
// }
// fn write_arr(vrr2: Vec<&str>){
//     println!("vrr2 = {:?}", vrr2);
// }

// solution borrowing:
fn main() {
    let mut vrr: Vec<&str> = vec!["sol", "eth", "pol"];
    write_arr(&mut vrr);  // ownership transfer
    println!("varr = {:?}", vrr);  // error here
}
fn write_arr(vrr2: &mut Vec<&str>){
    vrr2.push("btc");
    println!("vrr2 = {:?}", vrr2);
}