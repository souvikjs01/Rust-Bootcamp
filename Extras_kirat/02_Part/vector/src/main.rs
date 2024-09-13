fn main() {
    // let mut  vec: Vec<i32> = Vec::new();
    // vec.push(10);
    // vec.push(15);
    // vec.push(20);
    // vec.push(13);
    // vec.push(12);

    let mut vec = vec![10, 15,20, 13, 12];
    even_vec(&mut vec);
    println!("{:?}", vec);


    // let even = even_vec(vec);
    // println!("vec = {:?}", vec);
    // println!("even vec = {:?}", even_vec(vec));
}
// borrowing:
fn even_vec(v: &mut Vec<i32>) {
    let mut i=0;
    while i < v.len() {
        if v[i] % 2 != 0 {
            v.remove(i);
        } else {
            i += 1;
        }
    }
}


// fn even_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut new_vec = Vec::new();
//     for n in vec {
//         if n%2 == 0 {
//             new_vec.push(n);
//         }
//     }
//     return new_vec;
// }
