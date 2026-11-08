use std::fs;

fn main() {
    let text = fs::read_to_string("input.txt").expect("Eroare la deschidere fisier");

    let mut result = String::new();

    for word in text.split(' ') {
        let replacement = match word {
            "pt" => "pentru",
            "ptr" => "pentru",
            "dl" => "domnul",
            "dna" => "doamna",
            _ => word,
        };

        result.push_str(replacement);
        result.push(' ');
    }
    result.pop();

    println!("{result}");
    fs::write("output.txt", result).expect("Problema la scriere in file");
}
