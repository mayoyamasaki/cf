use std::io;
fn main() {
    let mut buf = String::new();
    // pass nubmer of chars
    io::stdin().read_line(&mut buf).unwrap();
    io::stdin().read_line(&mut buf).unwrap();
    let v: u32 = buf.split_whitespace()
               .map(|word| {
                   word.chars()
                       .map(|c| if c.is_uppercase() {1} else {0})
                       .sum()
               })
               .max().unwrap();
    println!("{}", v);
}
