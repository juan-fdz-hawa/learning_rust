fn double(n: i32) -> i32 {
    // Expressions do not end with a ';'
    // Blocks always return the last expression
    // so an explicit 'return' is not required.
    n * n
}

fn main() {
    let m = 5;

    // Variables are inmutable by default
    // ... this won't work
    // m += 10;

    // Unless you create a mutable variable
    let mut n = 5;
    n += 1;

    // You can also use shadowing
    let mut m = 10;
    m += 1;

    println!("{m}");

    // Scopes can be defined using {...}
    {
        let m = 21;
        print!("{m}");
    }
    // 'Different' variable since we are outside
    // the scope.
    println!("{m}");

    println!("{}", double(m));

}
