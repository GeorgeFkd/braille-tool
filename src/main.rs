use std::env;
use std::fs;
mod braille;
fn main() {
    //read text from user
    println!("Welcome to braille translator:");
    let file_path = "braille-2.txt";
    println!("Reading file: {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let braille_structure = braille::braille::convert_to_braille(contents, "gr");
    println!("Result of braille conversion:\n {}", braille_structure);
}
