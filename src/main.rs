fn main() {
    println!("Hello, world!");
}

// https://www.codewars.com/kata/5513795bd3fafb56c200049e/train/rust
fn count_by(x: u32, n: u32) -> Vec<u32> {
    (1..=n).into_iter().map(|number| number * x).collect::<Vec<u32>>()
}

// https://www.codewars.com/kata/5544c7a5cb454edb3c000047/train/rust
fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h > 0.0 && (0.0..1.0).contains(&bounce) && window < h {
        let mut rest_height = h * bounce;
        let mut count = 1;
        while rest_height > window {
            count += 2;
            rest_height = rest_height * bounce;
        }
        return count;
    }
    -1
}