pub fn fib_modulo(m: u128, n: u128) -> u128 {
    let mut prev = 0;
    let mut curr = 1;

    for _ in 1..n - 1 {
        let old = prev;
        let new = curr;
        prev = new;
        curr = new + old;
    }
    curr % n
}
