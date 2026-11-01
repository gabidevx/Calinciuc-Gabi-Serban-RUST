fn checked_add(x: u32, y: u32) -> u32 {
    let z: u64 = x as u64 + y as u64;
    let maxim: u64 = u32::MAX as u64;
    if z > maxim {
        panic!("Au fost adunate numerele x={x}, y={y}, care nu au rezultatul in u32.",);
    } else {
        x + y
    }
}

fn checked_mul(x: u32, y: u32) -> u32 {
    let z: u64 = x as u64 * y as u64;
    if z > u32::MAX as u64 {
        panic!("Au fost inmultite doua numere x={x}, y={y}, nu au rezultatul in u32.",);
    } else {
        x * y
    }
}

fn main() {
    let x = checked_add(2, 3);
    let y = checked_add(3_000_000_000, 3_000_000_000);
    let z = checked_mul(4, 6);
    let t = checked_mul(2_000_000, 5_000_000);

    println!("{}", u32::MAX);

    println!("{x}, {y}, {z}, {t}");
}
