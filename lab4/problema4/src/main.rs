#[derive(Debug)]
enum MyError {
    NotAscii,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, MyError> {
    if !c.is_alphabetic() {
        Err(MyError::NotLetter)
    } else {
        Ok(c.to_uppercase().next().unwrap())
    }
}

fn to_lowercase(c: char) -> Result<char, MyError> {
    if !c.is_alphabetic() {
        Err(MyError::NotLetter)
    } else {
        Ok(c.to_lowercase().next().unwrap())
    }
}

fn print_char(c: char) -> Result<(), MyError> {
    if c.is_control() {
        Err(MyError::NotPrintable)
    } else {
        println!("Caracterul este: {c}");
        Ok(())
    }
}

fn char_to_number(c: char) -> Result<u32, MyError> {
    if !c.is_ascii() {
        Err(MyError::NotAscii)
    } else if !c.is_ascii_digit() {
        Err(MyError::NotDigit)
    } else {
        Ok(c.to_digit(10).unwrap())
    }
}

fn char_to_number_hex(c: char) -> Result<u32, MyError> {
    if !c.is_ascii() {
        Err(MyError::NotAscii)
    } else if !c.is_ascii_hexdigit() {
        Err(MyError::NotBase16Digit)
    } else {
        Ok(c.to_digit(16).unwrap())
    }
}

fn print_error(error: MyError) {
    match error {
        MyError::NotAscii => println!("Caracterul nu este Ascii"),
        MyError::NotDigit => println!("Caracterul nu este cifra"),
        MyError::NotBase16Digit => println!("Caracterul nu este cifra in baza 16"),
        MyError::NotLetter => println!("Caracterul nu este litera"),
        MyError::NotPrintable => println!("Caracterul nu este printabil"),
    }
}

fn main() {
    match to_uppercase('a') {
        Ok(value) => println!("Upper 'a': {value}"),
        Err(e) => print_error(e),
    }
    match to_lowercase('A') {
        Ok(value) => println!("Lower 'A': {value}"),
        Err(e) => print_error(e),
    }
    match to_lowercase('Ș') {
        Ok(value) => println!("Lower 'Ș': {value}"),
        Err(e) => print_error(e),
    }
    match to_uppercase('A') {
        Ok(value) => println!("Upper 'A': {value}"),
        Err(e) => print_error(e),
    }
    match to_lowercase('1') {
        Ok(value) => println!("Lower '1': {value}"),
        Err(e) => print_error(e),
    }
    match print_char('k') {
        Ok(_) => println!("Caracterul este printabil"),
        Err(e) => print_error(e),
    }
    match char_to_number('5') {
        Ok(value) => println!("Numarul '5' este: {value}"),
        Err(e) => print_error(e),
    }
    match char_to_number_hex('F') {
        Ok(value) => println!("In baza 16 F: {value}"),
        Err(e) => print_error(e),
    }
    match char_to_number('A') {
        Ok(value) => println!("Numarul 'A' este: {value}"),
        Err(e) => print_error(e),
    }
}
