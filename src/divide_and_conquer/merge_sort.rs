pub fn merge_sort_caller(list: &mut Vec<u32>) -> Option<&Vec<u32>> {
    return merge_sort(list, 0, list.len() as u32 - 1);
    // println!("{}", list.len() as u32 - 1);
}

pub fn merge_sort(list: &mut Vec<u32>, left: u32, right: u32) -> Option<&Vec<u32>> {
    if left == right {
        return None;
    }
    if left > right || left > (list.len() - 1) as u32 {
        return Some(list);
    }
    let mid = left + (right - left) / 2;

    merge_sort(list, left, mid);
    merge_sort(list, mid + 1, right);
    merge_halves(list, left, mid, right);
    return Some(list);
}

pub fn merge_halves(list: &mut Vec<u32>, left: u32, mid: u32, right: u32) -> Option<Vec<u32>> {
    let mut left_half = vec![0; (mid - left + 1) as usize];
    let mut right_half = vec![0; (right - mid) as usize];
    left_half.copy_from_slice(&list[left as usize..(mid + 1) as usize]);
    right_half.copy_from_slice(&list[(mid + 1) as usize..(right + 1) as usize]);

    let mut left_marker: usize = 0;
    let mut right_marker: usize = 0;
    for i in left..=right {
        match i {
            _ if left_marker < left_half.len() && right_marker < right_half.len() => {
                if left_half[left_marker] < right_half[right_marker] {
                    list[i as usize] = left_half[left_marker];
                    left_marker += 1;
                } else {
                    list[i as usize] = right_half[right_marker];
                    right_marker += 1;
                }
            }
            _ if left_marker < left_half.len() => {
                list[i as usize] = left_half[left_marker];
                left_marker += 1;
                // }
            }
            _ if right_marker < right_half.len() => {
                list[i as usize] = right_half[right_marker];
                right_marker += 1;
            }
            _ => panic!(),
        }
    }

    return Some(list.to_vec());
}

pub fn execute() {
    println!(
        "weirdo --->>>>  {:?}",
        // merge_sort_caller(&mut vec![4, 5, 3, 12, 2, 4, 43, 9])
        // merge_sort_caller(&mut vec![4, 5, 3, 23, 654, 1, 123, 34, 12])
        merge_sort_caller(&mut vec![4, 5, 3, 23, 654, 1, 123, 34, 12])
    );
}
