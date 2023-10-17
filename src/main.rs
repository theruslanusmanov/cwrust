fn main() {
    println!("Hello, world!");
    let res = bouncing_ball(30.0, 0.66, 1.5);
    println!("{:?}", res)
}

/// "is2(0) Thi1s(1) T4est(2) 3a(3)"  -->  "Thi1s is2 3a T4est" (1, 0, 3, 2)
/// https://www.codewars.com/kata/55c45be3b2079eccff00010f/train/rust
fn order(sentence: &str) -> String {
    if sentence.is_empty() { return String::from("") }
    String::from("")
}

/// https://www.codewars.com/kata/5544c7a5cb454edb3c000047/train/rust
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

/// https://www.codewars.com/kata/5513795bd3fafb56c200049e/train/rust
fn count_by(x: u32, n: u32) -> Vec<u32> {
    (1..=n).into_iter().map(|number| number * x).collect::<Vec<u32>>()
}

fn divisors(n: u32) -> u32 {
    (1..=n).filter(|x| n % x == 0).count() as u32
}

