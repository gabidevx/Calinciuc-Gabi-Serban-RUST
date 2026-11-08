use std::fs;

fn main() {
    let text =
        fs::read_to_string(r"C:\Windows\System32\drivers\etc\hosts").expect("Eroare la citire");

    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut columns = line.split_whitespace();
        let ip = columns.next();
        let hostname = columns.next();

        if let (Some(ip), Some(hostname)) = (ip, hostname) {
            println!("{} => {}", hostname, ip);
        }
    }
}
