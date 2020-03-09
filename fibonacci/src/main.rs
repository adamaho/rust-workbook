use std::io;

fn main() {
    let mut n = String::new();

    println!("Enter nth number of fibonocci sequence: ");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read number");

    let parsed_n: i32 = n.trim().parse().expect("Please enter a number!");

    println!("The fibonocci number is: {}", calc_fib(parsed_n));
}

fn calc_fib(n: i32) -> i32 {
    let mut fib: [i32; 2] = [0, 1];

    if n == 0 {
        return 0;
    }

    for d in 0..n-2 {
        let sum = fib[0] + fib[1];

        fib = [fib[1], sum];
    }

    fib[1]
}
