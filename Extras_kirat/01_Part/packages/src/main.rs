// packages:

// use chrono::Local;
use chrono::Utc;

fn main() {
    // let now = Local::now();
    let now = Utc::now();
    println!("current time is {}", now);
}
