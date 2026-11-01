fn is_prime(x: u32) -> bool {
    if x <= 1 {
        return false;
    } else {
        let mut i: u32 = 2;
        while i * i <= x {
            if x.is_multiple_of(i) {
                return false;
            }
            i += 1;
        }
    }
    true
}

fn next_prime(x: u16) -> Option<u32> {
    let mut n = (x + 1) as u32;
    while !is_prime(n) {
        n += 1;
    }
    if n < u16::MAX as u32 { Some(n) } else { None }
}

fn main() {
    let mut i: u16 = 1;
    let mut done: bool = false;
    while i < u16::MAX && !done {
        let r = next_prime(i);
        match r {
            Some(value) => {
                println!("Prime number, {}", value);
                i = value as u16;
            }
            None => {
                println!("None");
                done = true;
            }
        }
        i += 1;
    }
}
