fn add_space(s: &mut String, mut n: i32) {
    while n > 0 {
        *s += " ";
        n -= 1;
    }
}

fn add_str(s: &mut String, s2: &str) {
  *s += s2;
}

/*
fn add_integer(s: &mut String, n: i32) {
    // to do home
}

fn add_float(s: &mut String, n: f32) {
    // to do home
}
*/

fn main() {
    let mut s: String = String::from("Ana ");
    add_space(&mut s, 10);
    s += "mere";
    println!("{s}");

    add_str(&mut s, " ana are mere mai multe!");
    println!("{s}");
}
