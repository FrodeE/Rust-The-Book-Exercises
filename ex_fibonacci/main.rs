use std::io;

fn main() {
    println!("What fibonacci number would you like?");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            panic!("Please input a valid positive number.");
        }
    };
    let result = calculate_fibonacci_number(n);
    println!("Fibonacci's sequence element {} is {}", n, result);
}


fn calculate_fibonacci_number(num: i64) -> i64 {
    if num == 1{
        return 0;
    } else if num == 2{
        return 1;
    }
    let mut iterator = 1;
    let mut first_value = 0;
    let mut second_value = 1;
    let mut value: i64 = 0;
    while iterator <= num{
        if iterator >= 3{
            value = first_value + second_value;
            println!("Value is {}", value);
            first_value = second_value;
            second_value = value;
        }
        iterator += 1;
    }
    value
}