fn prime(num: i32) -> bool {
    if num <= 1 {
        return false;
    } else if num == 2 {
        return true;
    } else {
        let mut i: i32 = 2;
        while i * i <= num {
            if num % i == 0 {
                return false;
            }
            i += 1;
        }
    }
    true
}

fn main() {
    println!("Nr prime de la 0 la 100:");
    let mut i: i32 = 0;
    while i <= 100 {
        if prime(i) {
            print!("{} ", i);
        }
        i += 1;
    }
}
