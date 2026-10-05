fn main() {
    let mut i: i32 = 99;
    while i >= 0 {
        if i >= 2 {
            println!("{} bottles of beer on the wall,", i);
            println!("{} bottles of beer.", i);
            println!("Take one down, pass it around,");
            println!("{} bottles of beer on the wall.", i - 1);
        } else if i == 1 {
            println!("1 bottle of beer on the wall,");
            println!("1 bottle of beer.");
            println!("Take one down, pass it around,");
            println!("No bottles of beer on the wall.");
        } else if i == 0 {
            println!("No bottles of beer on the wall,");
            println!("No bottles of beer.");
            println!("Go to the store, buy some more,");
            println!("99 bottles of beer on the wall.");
        }
        println!();
        i -= 1;
    }
}
