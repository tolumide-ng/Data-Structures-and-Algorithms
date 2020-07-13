// pub mod fibi {
pub fn fib1(n: i32) -> i32 {
    if n <= 1 {
        return n;
    } else {
        return fib1(n - 1) + fib1(n - 2);
    }
}
pub fn fib2(n: usize) -> i32 {
    let mut vec = vec![0, 1];
    println!("before ===>>>  {:?}", vec);
    // vec.push(0); //assign constant value at 0 at index 0
    // vec.push(1); //assign constant value of 1 at index 1
    for val in 2..=n {
        let result = vec[val - 1] + vec[val - 2];
        vec.push(result);
    }
    println!("{:?}", vec);
    vec[n]
}
pub fn execute() {
    // println!("{}", fib1(10));
    println!("{}", fib2(47));
}
// Visualize fibonacci different implementations here below
// https://www.cs.usfca.edu/~galles/visualization/DPFib.html
// }
