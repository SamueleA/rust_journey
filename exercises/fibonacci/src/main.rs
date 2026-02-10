use std::io;

fn main() {
    println!("Enter nth fibonacci number to compute to.");

    let nth_fibonacci: u64;

    loop {
        let mut nth_fibonacci_read = String::new();

        io::stdin()
            .read_line(&mut nth_fibonacci_read)
            .expect("Failed to read line");

        nth_fibonacci = match nth_fibonacci_read.trim().parse::<u64>() {
            Ok(n) => n,
            Err(_) => continue,
        };

        break;
    }

    fibonacci_calculate(nth_fibonacci);
}

fn fibonacci_calculate(max_fib_index: u64) {
    let mut current_fib_index = 1;
    let mut first_num = 0;
    let mut second_num = 1;

    while current_fib_index <= max_fib_index {
        if current_fib_index == 1 {
            println!("{first_num}");
        } else if current_fib_index == 2 {
            println!("{second_num}")
        } else {
            let temp_num = first_num + second_num;

            first_num = second_num;
            second_num = temp_num;
            println!("{second_num}");
        }

        current_fib_index += 1;
    }
}
