fn rot13(input: &str) -> Result<String, char> {
    let mut result = String::new();

    for c in input.chars() {
        if !c.is_ascii() {
            return Err(c);
        }

        let new_c = if c.is_ascii_lowercase() {
            ((c as u8 - b'a' + 13) % 26 + b'a') as char
        } else if c.is_ascii_uppercase() {
            ((c as u8 - b'A' + 13) % 26 + b'A') as char
        } else {
            c
        };
        result.push(new_c);
    }
    Ok(result)
}

fn main() {
    let text = "Ana are mere";
    let text2 = "aNA ARE MERE";
    let text3 = "AnA_aRe_MeRe";
    let text4 = "abc🎁🎶🎉👀🎈🎃🍕☕🍉def";

    match rot13(text) {
        Ok(result) => println!("{text} devine {result}"),
        Err(e) => println!("Am gasit caracterul : {e}"),
    }
    match rot13(text2) {
        Ok(result) => println!("{text} devine {result}"),
        Err(e) => println!("Am gasit caracterul : {e}"),
    }
    match rot13(text3) {
        Ok(result) => println!("{text} devine {result}"),
        Err(e) => println!("Am gasit caracterul : {e}"),
    }
    match rot13(text4) {
        Ok(result) => println!("{text} devine {result}"),
        Err(e) => println!("EROARE! Am gasit caracterul : {e}"),
    }
}
