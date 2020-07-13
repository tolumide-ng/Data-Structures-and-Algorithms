// calculates the lowest common multiple

// use super::gcd::gcd_naive;
use super::gcd::gcd_naive;

pub fn lcm(a: u128, b: u128) -> u128 {
    use std::cmp;
    let m = cmp::max(a, b);
    let n = cmp::min(a, b);

    if n == 0 {
        return n;
    }
    m * n / gcd_naive(m, n)
}

pub fn execute() {
    println!("{}", lcm(9, 6));
}
