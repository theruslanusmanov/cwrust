fn main() {
    println!("Hello, world!");
}

// https://www.codewars.com/kata/5513795bd3fafb56c200049e/train/rust
fn count_by(x: u32, n: u32) -> Vec<u32> {
    (1..=n).into_iter().map(|number| number*x).collect::<Vec<u32>>()
}