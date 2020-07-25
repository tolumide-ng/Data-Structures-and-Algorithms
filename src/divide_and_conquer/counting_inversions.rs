// important videos/lectures ==>
// https://www.youtube.com/watch?v=7_AJfusC6UQ&t=181s
// https://www.youtube.com/watch?v=Vj5IOD7A6f8

pub fn execute() {
    println!(
        "RESULT ===> COUNT => {}",
        // counting_inversions_caller(&mut vec![231, 12, 232, 12, 342, 21])
        // counting_inversions_caller(&mut vec![2, 3, 9, 2, 9])
        // counting_inversions_caller(&mut vec![6, 5, 4, 3, 2, 1]) // counting_inversions_caller(&mut vec![2, 3, 9, 2, 9]) // (&mut vec![4, 5, 3, 23, 654, 1, 123, 34, 12])
        // counting_inversions_caller(&mut vec![4, 5, 3, 23, 654, 1, 123, 34, 12])
        counting_inversions_caller(&mut vec![1, 2, 3, 6, 5, 4])
    )
}

pub fn counting_inversions_caller(list: &mut Vec<u32>) -> u32 {
    counting_inversions(list, 0, list.len() - 1).0.unwrap()
}

fn counting_inversions(
    list: &mut Vec<u32>,
    left: usize,
    right: usize,
) -> (Option<u32>, Option<&Vec<u32>>) {
    if left == right {
        return (Some(0), None);
    }

    if left > right || left > list.len() - 1 {
        return (Some(0), Some(list));
    }

    let mut inversion_count = 0;
    let mid_val = left + ((right - left) / 2);

    inversion_count += counting_inversions(list, left, mid_val).0.unwrap();
    inversion_count += counting_inversions(list, mid_val + 1, right).0.unwrap();
    inversion_count += merge_halves(list, left, mid_val, right);

    (Some(inversion_count), Some(list))
}

fn merge_halves(list: &mut Vec<u32>, left: usize, mid: usize, right: usize) -> u32 {
    let mut left_half = vec![0; mid - left + 1];
    let mut right_half = vec![0; right - mid];
    left_half.copy_from_slice(&list[left..mid + 1]);
    right_half.copy_from_slice(&list[mid + 1..right + 1]);

    let mut inversion_count: u32 = 0;
    let mut left_marker: usize = 0;
    let mut right_marker: usize = 0;

    for i in left..=right {
        match i {
            _ if left_marker < left_half.len() && right_marker < right_half.len() => {
                if left_half[left_marker] <= right_half[right_marker] {
                    list[i] = left_half[left_marker];
                    left_marker += 1;
                } else {
                    list[i] = right_half[right_marker];
                    right_marker += 1;
                    inversion_count += (mid - left_marker - left + 1) as u32;
                }
            }
            _ if left_marker < left_half.len() => {
                list[i] = left_half[left_marker];
                left_marker += 1;
            }
            _ if right_marker < right_half.len() => {
                list[i] = right_half[right_marker];
                right_marker += 1;
            }
            _ => panic!(),
        }
    }

    inversion_count
}
