pub fn money_change(val: i32) -> i32 {
    val / 10 as i32 + val % 5 + ((val / 10 as i32) % 5)
}

pub fn execute() {
    println!("{}", money_change(37));
}
