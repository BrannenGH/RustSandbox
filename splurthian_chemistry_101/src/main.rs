use std::str;
use std::string::String;

fn main() {
    let test_cases = [("Spenglerium", "Ee"), ("Zeddemorium", "Zr"),
     ("Venkmine", "Kn"), ("Stantzon", "Zt"), ("Melintzum", "Nn"), ("Tullium", "Ty"), ("Ẫthium", "Ẫi")];
    for test_case in test_cases.iter(){
        if check(test_case.0, test_case.1) == true{
            println!("The symbol {} works for {}", test_case.1, test_case.0);
        } else{
            println!("The symbol {} does not work for {}", test_case.1, test_case.0);
        }
    }
}

fn check(element: &str, symbol: &str) -> bool{
    let mut first_letter = String::new();
    let mut second_letter = String::new();
    let mut checker = false;
    for letter in symbol.char_indices(){
        first_letter.push(letter.1);
        if symbol.is_char_boundary(letter.0) == false && checker == false{
            continue;
        }
        checker = true;
        second_letter.push(letter.1);
        if symbol.is_char_boundary(letter.0){
            continue;
        }
        break;
    }
    println!("{}, {}", first_letter, second_letter);
    let mut result: bool = false;
    for letter in element.char_indices(){
        if letter.1.to_lowercase().nth(0).unwrap() == first_letter{
            let new_strs = element.split_at(next_index(letter.0, element));
            for character in new_strs.1.chars(){
                if character == second_letter{
                    result = true;
                }
            }
        }
    }
    return result;
}

fn next_index(mut letter_index: usize, element: &str) -> usize{
    letter_index += 1;
    while !element.is_char_boundary(letter_index) && letter_index <= element.len(){
        letter_index += 1;
    }
    if letter_index > element.len(){
        return element.len();
    } else {
        return letter_index;
    }
}
