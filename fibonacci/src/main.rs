fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    const N: u32 = 10;

    println!("The value of x is {}", fibonacci(N));
}
