use std::io;
use std::str;

fn main() {
    let mut element = String::new();
    let mut symbol = String::new();

    io::stdin.readline(&mut element).expect("Failed to read full element name! Exiting");
    io::stdin.readline(&mut symbol).expect("Failed to read symbol! Exiting");

    if (check(&element, &symbol)) == true{
        println!("Symbol Works");
    } else{
        println!("Does not work, try again.")
    }
}

fn check(element: &str, symbol: &str) -> bool{
    for charater in element.chars(){
        if character == symbol.chars().nth(0){
            char
            //Find a way to copy contents to a new string, cut that string, and then iterate over that.
        }else{return false;}
    }
    }
}
