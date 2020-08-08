pub fn major_element(list: &Vec<u32>, left: u32, right: u32) -> i32 {
    if right == left {
        return -1;
    }
    if left + 1 == right {
        return list[left as usize] as i32;
    }

    let left_elem = major_element(list, 0, left + (right - left) / 2);
    let right_elem = major_element(list, left + (right - left) / 2, right);

    let mut lcount = 0;
    for i in left..right {
        if list[i as usize] == left_elem as u32 {
            lcount += 1;
        }
    }
    if lcount > (right - left) / 2 {
        return left_elem;
    }

    let mut rcount = 0;
    for i in left..right {
        if list[i as usize] == right_elem as u32 {
            rcount += 1;
        }
    }
    if rcount > (right - left) / 2 {
        return right_elem;
    }
    return -1;
}

pub fn execute() {
    println!("RESULT >>>> {}", major_element(&vec![3, 2, 3, 4, 3], 0, 5))
}
