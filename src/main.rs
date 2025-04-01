use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        println!("Arguments: {:?}", &args[1..]);
    } else {
        println!("Hello!");
    }
}

// fn add(a: i32, b: i32) -> i32 {
//     return a * b;
// }