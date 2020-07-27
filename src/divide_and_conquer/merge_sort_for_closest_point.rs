pub mod merge_sort_module {
    use crate::divide_and_conquer::struct_defs::{Pair, SortBy, T};

    pub fn merge_sort(list: &mut T, left: usize, right: usize, sort_by: SortBy) -> Option<&T> {
        if left == right {
            return None;
        }
        if left > right || left > list.len() - 1 {
            println!("this happened");
            return Some(list);
        }

        let mid = left + (right - left) / 2;

        merge_sort(list, left, mid, sort_by);
        merge_sort(list, mid + 1, right, sort_by);
        merge_halves(list, left, mid, right, sort_by);

        return Some(list);
    }

    fn merge_halves<'a>(
        list: &mut T,
        left: usize,
        mid: usize,
        right: usize,
        sort_by: SortBy,
    ) -> Option<&T> {
        let dummy_points = Pair { x: 0, y: 0 };

        let mut left_half: T = vec![dummy_points; mid - left + 1];
        let mut right_half = vec![dummy_points; right - mid];
        left_half.clone_from_slice(&list[left..mid + 1]);
        right_half.clone_from_slice(&list[(mid + 1)..right + 1]);

        let mut left_marker: usize = 0;
        let mut right_marker: usize = 0;

        for i in left..=right {
            match i {
                _ if left_marker < left_half.len() && right_marker < right_half.len() => {
                    match sort_by {
                        SortBy::X => {
                            if left_half[left_marker].x < right_half[right_marker].x {
                                list[i as usize] = left_half[left_marker];
                                left_marker += 1;
                            } else {
                                list[i as usize] = right_half[right_marker];
                                left_marker += 1;
                            }
                        }
                        SortBy::Y => {
                            if left_half[left_marker].y < right_half[right_marker].y {
                                list[i as usize] = left_half[left_marker];
                                left_marker += 1;
                            } else {
                                list[i as usize] = right_half[right_marker];
                                left_marker += 1;
                            }
                        }
                    }
                }
                _ if left_marker < left_half.len() => {
                    list[i as usize] = left_half[left_marker];
                    left_marker += 1;
                }
                _ if right_marker < right_half.len() => {
                    list[i as usize] = right_half[right_marker];
                    right_marker += 1;
                }
                _ => panic!(),
            }
        }
        return Some(list);
    }
}
