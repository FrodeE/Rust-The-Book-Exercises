fn main() {
    let mut s = String::from("hello");
    s.push_str(", Tom!");
    println!("{}", s);

    //  Creates a copy of the value, since they are
    //  both stored on the stack. Do not depend of
    //  one another (if one goes out of scope)
    let a = 1;
    let b = a;

    //  Not creating a copy - y stores information
    //  of the pointer, length and capacity on the
    //  stack WITHOUT creating copy of the heap value.
    //  After y assignment, owner is moved and x is
    //  dropped!
    let x = String::from("hello");
    let y = x;

    //  Cloning example for Strings on heap.
    //  Creates a copy of stack AND heap values
    //  from s1 to s2.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    //  Cloning/copying since dealing with derivative
    //  types.
    let tup:(u32, bool) = (117, true);
    let tup_clone = tup;
    //  Destructuring tuple 1:
    println!("tup = {:?}, tup_clone = {:?}", tup, tup_clone);
    //  OR 2:
    let (tup_x, tup_y) = tup;
    let (tup_clone_x, tup_clone_y) = tup_clone;
    println!("tup_x = {:?}, tup_clone_x = {:?}", tup_x, tup_clone_x);
    println!("tup_y = {:?}, tup_clone_y = {:?}", tup_y, tup_clone_y);
   
}