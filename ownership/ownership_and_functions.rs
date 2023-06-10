fn main() {
    let s = String::from("hello");

    //  s's value moves into the function
    take_ownership(&s);
    //  and is valid as we BORROWED it.
    println!("S is {}", s);

    //  s's value moves into the function
    // take_ownership(s);
    //  and is no longer valid here.
    //println!("S is {}", s);

    let x = 5;
    //  x would move into the function,
    makes_copy(x);
    //  but i32 is Copy (meaning has trait Copy),
    //  so it's okay to still use x afterward.
    println!("x is {}", x);

    let s1 = gives_ownership();
    let s2 = String::from("Hello chat!");
    let s3 = takes_and_gives_back(&s1);
    let s4 = takes_and_gives_back(&s2);
    println!("s1 is {}, s3 is {}, s4 is {}", s1, s3, s4);

    let t1 = String::from("hello");
    let (t2, len) = calculate_length(t1);
    println!("The length of {} is {}", t2, len);

    let tekst = String::from("tekst on siin.");
    let pikkus = cal_pikkus(&tekst);
    println!("Tekst on {} ja pikkus on {}", tekst, pikkus);
}

fn cal_pikkus(tekst: &String) -> usize{
    let pikkus = tekst.len();
    pikkus
}

fn take_ownership(some_string: &str){
    println!("{}", some_string)
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer)
}

fn gives_ownership() -> String {
    //  this function will move its return value into
    //  the function that calls it
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: &str) -> String {
    a_string.to_string()
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}