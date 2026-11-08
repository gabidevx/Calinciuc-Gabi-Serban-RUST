#[derive(Debug)]
enum MyError {
    DivisionByZero,
}

fn impartire(x: f64, y: f64) -> Result<f64, MyError> {
    if y == 0.0 {
        Err(MyError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}

fn get_nat_number_from_string(s: &str) -> Option<u32> {
    s.parse::<u32>().ok()
}

fn main() {
    match impartire(7.6, 2.5) {
        Ok(value) => println!("Rezultat: {value}"),
        Err(e) => println!("Eroare: {:?}", e),
    }
    match impartire(12.3, 0.0) {
        Ok(value) => println!("Rezultat: {value}"),
        Err(e) => println!("Eroare: {:?}", e),
    }
    match get_nat_number_from_string("123") {
        Some(value) => println!("Nr ok: {value}"),
        None => println!("Eroare: nr nu este ok"),
    }
    match get_nat_number_from_string("123abc") {
        Some(value) => println!("Nr ok: {value}"),
        None => println!("Eroare: nr nu este ok"),
    }
}
