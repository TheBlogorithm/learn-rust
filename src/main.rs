fn main() {
    println!("Hello, world!");
    let (num1, num2) = (15.0, 20.0);
    println!("Sum of {} and {}.", num1, num2);
    {
        let ns = num_sum(num1, num2);
        println!("{}", ns);
    }
}

// Input arguments in Rust must be statically typed
fn num_sum(n1: f32, n2: f32) -> f32 {
    n1 + n2
}