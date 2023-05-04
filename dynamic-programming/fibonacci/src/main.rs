fn main() {
    println!("Fibonacci");
    println!("fib(0)={}", fib(0));
    println!("fib(1)={}", fib(1));
    println!("fib(2)={}", fib(2));
    println!("fib(3)={}", fib(3));
    println!("fib(4)={}", fib(4));
    println!("fib(5)={}", fib(5));
    println!("fib(6)={}", fib(6));
    println!("fib(7)={}", fib(7));
    println!("fib(8)={}", fib(8));
    println!("fib(9)={}", fib(9));
    println!("fib(10)={}", fib(10));
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    let mut arr = vec![0; (n+1) as usize];
    arr[0] = 0;
    arr[1] = 1;
    for i in 2..(n+1) {
        arr[i as usize] = arr[(i-1) as usize] + arr[(i-2) as usize];
    }
    return arr[n as usize];
}
