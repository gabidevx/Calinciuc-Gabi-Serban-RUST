fn coprimes (mut n1 : i32, mut n2 : i32) -> bool
{
    if n1 == 0 || n2 == 0 {
        return false
    }
    while n2 != 0 {
        let rest : i32 = n1 % n2;
        n1 = n2;
        n2 = rest;
    }
    if n1 == 1 {
        return true;
    }
    return false;
}

fn main() {
    for i in 0..101 {
        for j in 0..101 {
            if coprimes(i, j)
            {
                println!("{} si {} sunt coprime", i, j);
            }
        }
    }
}
