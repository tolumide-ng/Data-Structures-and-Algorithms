type T = Vec<i32>;

pub struct Lottery {
    total_points: u32,
    total_segments: u32,
    points: T,
    starts: T,
    ends: T,
}

pub fn execute() {
    let all_input: Vec<i32> = vec![2, 3, 0, 5, 7, 10, 1, 6, 13];
    // let later_input = vec![3, 2, 0, 5, -3, 2, 7, 10, 1, 6];

    let number_of_segments = all_input[0];
    let number_of_points = all_input[1];
    let mut points = vec![0; number_of_points as usize];
    // let mut segments = vec![0; number_of_segments * 2];
    points.copy_from_slice(&all_input[all_input.len() - number_of_points as usize..]);
    // segments.copy_from_slice(&all_input[2..(all_input.len() - number_of_points)]);
    let mut starts = vec![];
    let mut ends = vec![];

    for val in 2..(number_of_segments * 2) + 2 {
        if val % 2 == 0 {
            starts.push(val);
        } else {
            ends.push(val);
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

pub fn organizing_lottery_caller(lottery: Lottery) {
    const LEFT: u32 = 1;
    const POINT: u32 = 2;
    const RIGHT: u32 = 3;

    let starts_l = vec![LEFT; lottery.starts.len()];
    let points_p = vec![POINT; lottery.points.len()];
    let ends_r = vec![RIGHT; lottery.points.len()];

    println!("so far >>>>> starts ==> {:?}", starts_l);
    println!("so far >>>>> ends ==> {:?}", ends_r);
    println!("so far >>>>> points ==> {:?}", points_p);

    // add in order
    let pair_numbers = lottery.starts, lottery.ends, lottery.points;
    let pair_letters = starts_l, ends_r, points_p;
    
    randomized_quick_sort(pair_numbers, pair_letters, left, right);
}



pub fn randomized_quick_sort(numbers: Vec<i32>, letters: Vec<u32>, left: u32, right: u32) {
    if left > right {
        return
    }

    let x = rand::random::<u8>();
}