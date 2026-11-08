#[derive(Debug)]
enum MyError {
    Overflow,
}

fn checked_add(x: u32, y: u32) -> Result<u32, MyError> {
    let z: u64 = x as u64 + y as u64;
    let maxim: u64 = u32::MAX as u64;
    if z > maxim {
        Err(MyError::Overflow)
    } else {
        Ok(z as u32)
    }
}

fn checked_mul(x: u32, y: u32) -> Result<u32, MyError> {
    let z: u64 = x as u64 * y as u64;
    if z > u32::MAX as u64 {
        Err(MyError::Overflow)
    } else {
        Ok(z as u32)
    }
}

fn calculate(x: u32, y: u32) -> Result<u32, MyError> {
    let a = checked_add(x, y)?;
    let b = checked_mul(a, y)?;

    Ok(b)
}

fn main() {
    match calculate(2, 3) {
        Ok(value) => println!("Rezultat: {value}"),
        Err(e) => println!("Eroare: {:?}", e),
    }

    match calculate(3_000_000_000, 2_000_000_000) {
        Ok(value) => println!("Rezultat: {value}"),
        Err(e) => println!("Eroare: {:?}", e),
    }
}
