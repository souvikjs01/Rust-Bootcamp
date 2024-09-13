use std::collections::HashMap;
// fn main() {
//     let mut users: HashMap<String, u32> = HashMap::new();

//     users.insert(String::from("harkirat"), 23);
//     users.insert(String::from("alex"), 24);
//     users.insert(String::from("valvarde"), 28);
//     users.insert(String::from("paredese"), 29);
//     users.insert(String::from("gavi"), 22);

//     let user_age = users.get("harkirat"); // 23

//     match user_age {
//         Some(age) => println!("age is {}", age),
//         None => println!("User not found in db"),
//     }
// }

// 2)
// problem
fn group_values_by_key(v: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in v {
        hm.insert(key, value);
    }
    return hm;
}

fn main() {
    let input_vec = vec![(String::from("souvik"), 22), (String::from("kirat"), 27)];
    let hm = group_values_by_key(input_vec);
    println!("{:?}", hm);
}
