fn main() {
    // println!("Hello, world!");
    // another_function(117, 343);
    let y = expression_example();
    println!("y's value is {}", y);

    let y_plus_three = add_three(y as i32);
    println!("y's new value is {}", y_plus_three);
    println!("Is it greater than 5? {}", greater_than_number(y_plus_three));
    println!("25 is divisible by {}", if_to_match(26));
    let number: i32 = -2;
    let is_positive = if number >= 0 { true} else {false};
    println!("The vale of is_positive is {}", is_positive);
    count_to_ten();
    countdown(3);
    countdown_from_five();
}

fn countdown_from_five() {
    let numbers = [5,4,3,2,1];
    for number in numbers.iter() {
        println!("{}", number);
    }

    for number in (1..6).rev() {
        println!("{}", number);
    }
}

fn count_to_ten() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10{
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}

fn countdown(mut x: u32) {
    while x > 0 {
        println!("{}!", x);
        x -= 1;
    }
    println!("LIFTOFF!");
}

fn if_to_match(number: u32) -> i32 {
    if number % 4 == 0 {
        println!("number is divisible by 4");
        4
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
        3
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
        2
    } else {
        println!("number is not divisible by 4, 3, or 2");
        0
    }
}


fn another_function(x: u32, y: u32) {
    println!("X is {}, Y is {}", x, y);
}

fn expression_example() -> u32 {
    let x = 5;  // statement - does not return a value

    {
        let x = 3;
        x + 1
    }
}

fn add_three(x: i32) -> i32 {
    x + 3
}

fn greater_than_number(x: i32) -> bool {
    if x > 5 {
        true
    } else{
        false
    }
}