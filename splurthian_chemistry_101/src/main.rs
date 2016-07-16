use std::io;
use std::str;
use std::char;
use std::slice;

fn main() {
    let mut element = String::new();
    let mut symbol = String::new();

    io::stdin().read_line(&mut element).expect("Failed to read full element name! Exiting");
    io::stdin().read_line(&mut symbol).expect("Failed to read symbol! Exiting");

    if check(&element, &symbol) == true{
        println!("Symbol Works");
    } else{
        println!("Does not work, try again.");
    }
}

fn check(element: &str, symbol: &str) -> bool{
    for character in element.char_indicies(){
        if character.1.to_lowercase() == symbol.chars().nth(0).to_lowercase(){
            let splitTup = element.split_at(character.0 * 4); //assumes char is four bytes and is in latin based UTF-8, won't work with non-UTF8
            for newChar in splitTup.1.chars(){
                if newChar.to_lowercase() == symbol.chars().nth(1).to_lowercase(){
                    return true;
                }
                else{
                    return false;
                }
            }
        }
        else{
            return false;
        }
    }
}
