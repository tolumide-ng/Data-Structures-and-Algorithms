pub fn euclid_gcd(a: u128, b: u128) -> u128 {
    // for a/b i.e a divided by b
    if b == 0 {
        return a;
    } else {
        let remainder = a % b;
        // remainder
        return euclid_gcd(b, remainder);
    }
}

// same method, just another implementation
pub fn gcd_naive(a: u128, b: u128) -> u128 {
    use std::cmp;
    let m = cmp::max(a, b);
    let n = cmp::min(a, b);
    if m % n == 0 {
        return n;
    }
    gcd_naive(n, m % n)
}

pub fn execute() {
    println!("{}", euclid_gcd(3918848, 1653264));
    println!("{}", euclid_gcd(28851538, 1183019));
    println!("{}", gcd_naive(3918848, 1653264));
    println!("{}", gcd_naive(28851538, 1183019));
}

// Explanation
// to find the greatest common divisor of a/b
// rather than adding the two values and looping from 0 to find the largest divisor
// we would say a%b = a'
// a = a' + b*q where q is the multiple of b in a
// where q is a/b
// i.e 75/20 => 75%20 => 15
// i.e 75(a) = 15(a') + 20(b)*3(q)
// make the solution recursive and therefore repeat this for (b, a')
