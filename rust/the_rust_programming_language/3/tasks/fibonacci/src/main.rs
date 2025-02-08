fn fibonacci(n: u64) -> u64 {
    if n < 2 {
        n
    }
    else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    println!("F0: {}", fibonacci(0));
    println!("F1: {}", fibonacci(1));
    println!("F2: {}", fibonacci(2));
    println!("F3: {}", fibonacci(3));
    println!("F19: {}", fibonacci(19));
}
