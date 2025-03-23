use learn_rust::multiply;

fn main() {
    println!("Hello, world!");
    let (num1, num2) = (15, 20);
    println!("Sum of {} and {}.", num1, num2);
    {
        let ns = num_sum(num1, num2);
        println!("{}", ns);
        println!("Multiplied by 2: {}", multiply(ns, 2));
    }
}

// Input arguments in Rust must be statically typed
fn num_sum(n1: i32, n2: i32) -> i32 {
    n1 + n2
}