use std::str;

fn main() {
    let test_cases = [("Spenglerium", "Ee"), ("Zeddemorium", "Zr"),
     ("Venkmine", "Kn"), ("Stantzon", "Zt"), ("Melintzum", "Nn"), ("Tullium", "Ty")];
    for test_case in test_cases.iter(){
        if check(test_case.0, test_case.1) == true{
            println!("The symbol {} works for {}", test_case.1, test_case.0);
        } else{
            println!("The symbol {} does not work for {}", test_case.1, test_case.0);
        }
    }
}

fn check(element: &str, symbol: &str) -> bool{
    let first_letter = symbol.chars().nth(0).unwrap().to_lowercase().nth(0).unwrap();
    let second_letter = symbol.chars().nth(1).unwrap().to_lowercase().nth(0).unwrap();
    let mut result: bool = false;
    for letter in element.char_indices(){
        if letter.1.to_lowercase().nth(0).unwrap() == first_letter{
            let new_strs = element.split_at(letter.0 + 1);
            //Will not work for non-one byte strings, I will try to implement this
            for character in new_strs.1.chars(){
                if character == second_letter{
                    result = true;
                }
            }
        }
    }
    return result;
}
