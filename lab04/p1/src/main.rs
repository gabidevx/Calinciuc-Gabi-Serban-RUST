use std::fs;

fn main() {
    let file = fs::read_to_string("input.txt").expect("Eroare la citire");

    let mut longest_bytes = "";
    let mut longest_characters = "";

    let mut max_bytes = 0;
    let mut max_characters = 0;

    for line in file.lines() {
        let bytes = line.len();
        let characters = line.chars().count();

        if bytes > max_bytes {
            max_bytes = bytes;
            longest_bytes = line;
        }

        if characters > max_characters {
            max_characters = characters;
            longest_characters = line;
        }
    }

    println!("Cea mai lunga linie: (bytes) {longest_bytes}");
    println!("Numar de biti: {max_bytes}");

    println!("Cea mai lunga linie: (characters) {longest_characters}");
    println!("Numar de caractere: {max_characters}");
}
