use std::io;
use std::str;

fn main() {
    let mut element = String::new();
    let mut symbol = String::new();

    io::stdin.readline(&mut element).expect("Failed to read full element name! Exiting");
    io::stdin.readline(&mut symbol).expect("Failed to read symbol! Exiting");

    if (check(&element, &symbol)) == True{
        println!("Symbol Works");
    } else{
        println!("Does not work, try again.")
    }
}

fn check(element: &str, symbol: &str) -> []{
    if rec_check(element, symbol.chars().nth(1)) && rec_check()== True{
        if rec_check
    }
}

fn rec_check(element: &str letter: &str) -> bool{

}
