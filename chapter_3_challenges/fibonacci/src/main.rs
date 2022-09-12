use std::io;

fn main() {
    loop {
        println!("Please input n");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Can't read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let result = fib(n);
        println!("Result = {result}");
    }
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}
