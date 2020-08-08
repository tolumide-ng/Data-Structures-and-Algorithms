pub fn quick_sort_caller(list: &mut Vec<u32>) -> Option<&Vec<u32>> {
    return quick_sort(list, 0, list.len() as u32 - 1);
}

pub fn get_pivot(list: &mut Vec<u32>, low: u32, high: u32) -> u32 {
    // if low > high {
    //     return list[high as usize];
    // }
    let mid_num = low + (high - low) / 2;

    let mut find_mid_pivot = vec![
        list[low as usize],
        list[high as usize],
        list[mid_num as usize],
    ];
    println!(
        "VALUES {:?}, {:?}, {:?}",
        list[low as usize], list[high as usize], list[mid_num as usize]
    );
    find_mid_pivot.sort();

    let mid_value = find_mid_pivot[1];

    let pivot = match mid_value {
        _ if mid_value == list[low as usize] as u32 => low,
        _ if mid_value == list[high as usize] as u32 => high,
        _ if mid_value == list[mid_num as usize] as u32 => mid_num,
        _ => panic!(),
    };

    return pivot;
}

pub fn quick_sort(list: &mut Vec<u32>, low: u32, high: u32) -> Option<&Vec<u32>> {
    if low == high {
        return None;
    }
    if high > (list.len() - 1) as u32 {
        return Some(list);
    }
    if low > high {
        return Some(list);
    }
    // do threshold check to use quick selection later

    let p = partition(list, low, high);
    quick_sort(list, low, p - 1);
    quick_sort(list, p + 1, high);
    return Some(list);
}

pub fn partition(list: &mut Vec<u32>, low: u32, high: u32) -> u32 {
    // println!("low, {}, high, {}, LIST {:?}", low, high, list);
    if low > high {
        return list[high as usize];
    }

    let pivot_index = get_pivot(list, low, high);
    let pivot_value = list[pivot_index as usize];
    // list[pivot_index], list[low] = list[low], list[pivot_index]
    // swap pivot position to ensure the pivot is at the beginning of the list
    if pivot_index != low {
        let a = list[pivot_index as usize];
        let b = list[low as usize];

        list[pivot_index as usize] = b;
        list[low as usize] = a;
    }

    let mut border = low as usize;

    for i in low..=high {
        if list[i as usize] < pivot_value {
            let new_val = list[i as usize];
            border += 1;
            let val_at_border = list[border];
            list[i as usize] = val_at_border;
            list[border] = new_val;
        }
    }
    let a = list[low as usize];
    let b = list[border];
    list[border] = a;
    list[low as usize] = b;

    return border as u32;
}

pub fn execute() {
    println!(
        "Result {:?}",
        quick_sort_caller(&mut vec![3, 9, 5, 5, 128, 2, 12, 4, 12, 43])
    );
}
