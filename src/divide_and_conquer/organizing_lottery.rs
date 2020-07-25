// I had intended to use binary search to solve this problem, but considering the fact that I had to loop through
// list of segments to create a NEW flat segment that contains all values in all segments which means an O(n*2),
// doing this for each use made it wrong so I checked out other's implementation online
// Please check out this medium post as she gives a good insight into why this is a sorting algorithm question
// https://towardsdatascience.com/course-1-algorithmic-toolbox-part-3-divide-and-conquer-dd9022bfa2c0

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type T = Rc<RefCell<Vec<i32>>>;

pub struct Lottery {
    total_points: u32,
    total_segments: u32,
    points: T,
    starts: T,
    ends: T,
}

pub fn execute() {
    // let all_input: Vec<i32> = vec![2, 3, 0, 5, 7, 10, 1, 6, 13];
    let all_input = vec![3, 2, 0, 5, -3, 2, 7, 10, 1, 6];

    let number_of_segments = all_input[0];
    let number_of_points = all_input[1];
    let points = Rc::new(RefCell::new(vec![0; number_of_points as usize]));
    // let mut segments = vec![0; number_of_segments * 2];
    points
        .borrow_mut()
        .copy_from_slice(&all_input[all_input.len() - number_of_points as usize..]);
    let starts = Rc::new(RefCell::new(vec![]));
    let ends = Rc::new(RefCell::new(vec![]));

    for val in 2..(number_of_segments * 2) + 2 {
        if val % 2 == 0 {
            starts.borrow_mut().push(all_input[val as usize]);
        } else {
            ends.borrow_mut().push(all_input[val as usize]);
        }
    }

    let lottery = Lottery {
        total_points: number_of_points as u32,
        total_segments: number_of_segments as u32,
        points,
        starts,
        ends,
    };

    println!("RESULT ===>>> {:?}", organizing_lottery_caller(lottery));
}

pub fn organizing_lottery_caller(lottery: Lottery) -> HashMap<i32, u32> {
    const LEFT: &str = "LEFT";
    const POINT: &str = "POINT";
    const RIGHT: &str = "RIGHT";

    let starts_l = vec![LEFT; lottery.starts.borrow().len()];
    let points_p = vec![POINT; lottery.points.borrow().len()];
    let ends_r = vec![RIGHT; lottery.ends.borrow().len()];

    // add in order
    let pair_numbers: Vec<i32> = vec![
        lottery.starts.borrow_mut().clone(),
        lottery.ends.borrow_mut().clone(),
        lottery.points.borrow_mut().clone(),
    ]
    .concat();

    let pair_letters = vec![starts_l, ends_r, points_p].concat();
    let mut combined_pairs: Vec<(i32, &str)> = vec![];

    if pair_numbers.len() == pair_letters.len() {
        for i in 0..pair_numbers.len() {
            let new_pair = (pair_numbers[i], pair_letters[i]);
            // println!("the new pair");
            combined_pairs.push(new_pair);
        }
    }

    use quick_sort::randomized_quick_sort;

    randomized_quick_sort(&mut combined_pairs, 0, (pair_numbers.len() - 1) as u32);
    println!("THE COMBINED PAIRS {:?}", combined_pairs);
    let mut hash_map: HashMap<i32, u32> = HashMap::new();

    // for value in lottery.points.borrow().iter() {
    //     hashMap
    // }

    let mut count_left = 0;

    // let mut result = vec![(0, 0); lottery.points.borrow().len()];
    for value in combined_pairs {
        if value.1 == "LEFT" {
            count_left += 1;
        } else if value.1 == "RIGHT" {
            count_left -= 1;
        } else if value.1 == "POINT" {
            hash_map.insert(value.0, count_left);
        }
    }

    hash_map
}

mod quick_sort {
    use rand::{thread_rng, Rng};

    type R = Vec<(i32, &'static str)>;

    pub fn randomized_quick_sort(list: &mut R, left: u32, right: u32) -> Option<&R> {
        if left == right {
            return None;
        }

        if left > right {
            return Some(list);
        }

        let p = partition(list, left, right);
        randomized_quick_sort(list, left, p);
        randomized_quick_sort(list, p + 1, right);

        // println!("Here now {:?}", list);

        Some(list)
    }

    fn partition(list: &mut R, left: u32, right: u32) -> u32 {
        // println!("pivot and list {:?} {} {}", list, left, right);

        let pivot_index = get_pivot(list, left, right);

        // println!("pivot index {} and list {:?}", pivot_index, list);

        let pivot_value = list[pivot_index];

        if pivot_index != left as usize {
            let pivot = list[pivot_index];
            let begin = list[left as usize];

            list[pivot_index] = begin;
            list[left as usize] = pivot;
        }

        let mut border = left as usize;

        for i in left..=right {
            if list[i as usize].0 < pivot_value.0 {
                let new_val = list[i as usize];
                border += 1;
                let val_at_border = list[border];
                list[i as usize] = val_at_border;
                list[border] = new_val;
            }
        }

        // Swap border from 0 to current border position
        let a = list[border];
        let b = list[left as usize];

        list[left as usize] = a;
        list[border] = b;

        border as u32
    }

    fn get_pivot(list: &mut R, left: u32, right: u32) -> usize {
        // println!("LIST ====> {:?}", list);
        // println!("LEFT ====> {:?}", left);
        // println!("RIGHT ====> {:?}", right);
        thread_rng().gen_range(left, right) as usize
    }
}
