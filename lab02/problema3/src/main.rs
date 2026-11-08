fn add_space(s: &mut String, mut n: i32) {
    while n > 0 {
        *s += " ";
        n -= 1;
    }
}

fn add_str(s: &mut String, s2: &str) {
    *s += s2;
}

fn add_integer(s: &mut String, n: i32) {
    let mut x = n as i32;
    let negative = x < 0;

    if negative {
        x = -x;
    }

    let mut digits = [0 as i32; 11];
    let mut len = 0;

    if x == 0 {
        digits[0] = 0;
        len = 1;
    } else {
        while x > 0 {
            digits[len] = (x % 10) as i32;
            x /= 10;
            len += 1;
        }
    }

    if negative {
        *s += "-";
    }

    for i in (0..len).rev() {
        if i != len - 1 && (i + 1) % 3 == 0 {
            *s += "_";
        }

        match digits[i] {
            0 => *s += "0",
            1 => *s += "1",
            2 => *s += "2",
            3 => *s += "3",
            4 => *s += "4",
            5 => *s += "5",
            6 => *s += "6",
            7 => *s += "7",
            8 => *s += "8",
            9 => *s += "9",
            _ => {}
        }
    }
}

fn add_float(s: &mut String, n: f64) {
    let x = n as i32;
    add_integer(s, x);
    let mut fp = n - (n as i32) as f64;
    fp *= 1_000_000.0 + 0.1;

    *s += ".";

    let mut count = 0;

    let mut digits = [0i32; 6];
    while count < 6 {
        let integer = (fp as i32) % 10;
        fp = fp - integer as f64;
        digits[count] = integer;
        fp /= 10.0;
        count += 1;
    }

    let mut end = 0;
    while end < 5 && digits[end] == 0 {
        end += 1;
    }

    for i in (end..6).rev() {
        match digits[i] {
            0 => *s += "0",
            1 => *s += "1",
            2 => *s += "2",
            3 => *s += "3",
            4 => *s += "4",
            5 => *s += "5",
            6 => *s += "6",
            7 => *s += "7",
            8 => *s += "8",
            9 => *s += "9",
            _ => {}
        }
    }
}

fn main() {
    let mut s = String::new();

    add_space(&mut s, 40);
    add_str(&mut s, "I");
    add_space(&mut s, 1);
    add_str(&mut s, "💚\n");

    add_space(&mut s, 40);
    add_str(&mut s, "RUST.\n");

    add_str(&mut s, "\n");

    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 6);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "lastest");
    add_space(&mut s, 9);
    add_str(&mut s, "is\n");

    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 8);
    add_str(&mut s, "has");
    add_space(&mut s, 13);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 9);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".\n");

    add_space(&mut s, 20);

    println!("{}", s);
}
