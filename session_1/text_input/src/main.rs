use std::io::stdin;

fn read_input() -> String {
    let mut buffer = String::new();

    stdin()
        .read_line(&mut buffer)
        .expect("Can't read from stdin");

    buffer.trim().to_string()
}

fn main() {
    // String are meant to be editable
    // str are slices
    println!("What's your name?");
    let name = read_input();
    println!("Hello {}", name);
}
