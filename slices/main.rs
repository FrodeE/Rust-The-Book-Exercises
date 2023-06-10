fn main() {
    let tekst = String::from("Joumeesjou!");
    let word = return_word(&tekst, 9);
    println!("Word we are looking for is {}", word);
}


// Can use String reference where string slice reference is required!
fn return_word(text: &str, index: usize) -> &str {
    let text_in_bytes = text.bytes();
    let mut contains_space: bool = false;
    for (byte_index, byte) in text_in_bytes.enumerate(){
        if byte == b' '{
            contains_space = true;
        }
        if byte_index >= index {
            if byte == b' ' || (byte_index == text.len()-1 && 
            contains_space == true){
                return &text[index..byte_index+1];
            }
        }
    }
    &text[..]
} 

//let index = first_word(&tekst);
    //let (start, end) = second_word(&tekst);
    
    //println!("First word is {}", &tekst[0..index+1]);
    //println!("Second word is {}", &tekst[start..end+1]);

fn first_word(input: &String) -> usize {
    let input_in_bytes = input.bytes();

    //  enumerate() creates a tuple of the values
    //  first value is index, second is value
    for (index, byte) in input_in_bytes.enumerate(){
        if byte == b' '{
            return index;
        }
    }

    input.len()
}

fn second_word(input: &String) -> (usize, usize){
    let input_in_bytes = input.bytes();
    let mut first_index = 0;
    for (index, byte) in input_in_bytes.enumerate(){
        if byte == b' '{
            if first_index == 0 {
                first_index = index + 1;
            }
            else if first_index != 0 {
                return (first_index, (index-1));
            } 
        }
    }
    (input.len(), input.len())
}




























fn first_word2(input: &String) -> usize {
    let word_bytes = input.bytes();
    for (i, byte) in word_bytes.enumerate(){
        if byte == b' '{
            return i;
        }
    }
    input.len()
}
