fn greet(n: String) -> String {
    println!("Hello {}", n);
    n
}

fn greet_two(n: &String) {
    println!("Hello {}", n);
}

fn greet_mut(n: &mut String) {
    *n = format!("{} Yoo!", n)
}

fn main() {
    let name = "Juan".to_string();
    // When you call a function with a non-trivial type
    // (like String) .. you are 'moving' the ownership to the function
    greet(name);

    // So this won't compile
    // greet(name);

    // In Rust every variable must have an owner...
    // at line 7 'name' was owned by main, after calling
    // greet in line 10, 'name' was owned by greet.

    // To get around this we have a couple of options
    // 1 - We could use shadowing
    let last_name = "Fernandez".to_string();
    let last_name = greet(last_name);

    // 2 - We could change the function so that it borrows the
    // reference instead of owning the reference.
    greet_two(&last_name);
    greet_two(&last_name);

    // 3 - Yet another option is to clone things
    // which makes a deep copy of the String variable
    greet(last_name.clone());
    greet(last_name.clone());

    // Mutable references
    let mut other = "Hello".to_string();
    greet_mut(&mut other);

    // TLDR - Borrow checker:
    // Every variable has one and only one owner
    // Non-trivial datatypes are never explicitly cloned.
    // You can have as many inmutable references as you want.
    // You can only have a single mutable reference.
    // When mixing mutable and inmutable, ordering matters.
}
