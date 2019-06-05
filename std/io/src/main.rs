use std::io;

fn main() {
    let mut name = String::new();

    println!("Tell me your name: ");

    io::stdin().read_line(&mut name)
        .expect("Error: read line");

    println!("Hi {}", name);
}
