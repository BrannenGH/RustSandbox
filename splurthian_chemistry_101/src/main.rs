use std::io;
use std::str;
use std::char;
use std::slice;
use std::string::String;

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
    for character in element.chars().enumerate(){
        if &character.0.to_lowercase().last().unwrap() == &symbol.chars().nth(0).unwrap().to_lowercase().last().unwrap(){
            let newStrs = element.split_at(character.1);
            for newChar in newStrs.1.chars(){
                if &newChar.to_lowercase().last().unwrap() == &symbol.chars().nth(1).unwrap().to_lowercase().last().unwrap(){
                    return true;
                } else{
                    return false;
                }
            }
        } else{
            return false;
        }
    }
}
