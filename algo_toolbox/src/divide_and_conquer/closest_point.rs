mod closest_point {
    use crate::divide_and_conquer::merge_sort_for_closest_point::merge_sort_module::merge_sort;
    use crate::divide_and_conquer::struct_defs::{Pair, SortBy, T};

    fn closest_point_caller(list: &mut T) -> f32 {
        let list_length = list.len() - 1;
        let mut list_for_y = list.clone();

        let merge_by_x = merge_sort(list, 0, list_length, SortBy::X).unwrap();
        let merge_by_y = merge_sort(&mut list_for_y, 0, list_length, SortBy::Y).unwrap();

        let (p1, p2, mi) = closest_pair(merge_by_x, merge_by_y);
        mi
    }

    fn get_distance(point1: Pair, point2: Pair) -> f32 {
        ((point1.x - point2.x).pow(2) as f32 + (point1.y - point2.y).pow(2) as f32).sqrt()
    }

    fn brute_check(sorted_by_x: &T) -> (Pair, Pair, f32) {
        let mut distance_between = get_distance(sorted_by_x[0], sorted_by_x[1]);
        let mut point1 = sorted_by_x[0];
        let mut point2 = sorted_by_x[1];
        let list_length = sorted_by_x.len();
        if list_length == 2 {
            return (point1, point2, distance_between);
        }

        for i in 0..list_length {
            for j in i + 1..(list_length - 1) {
                if i != 0 && j != 0 {
                    let new_distance_between = get_distance(sorted_by_x[i], sorted_by_x[j]);
                    if new_distance_between < distance_between {
                        distance_between = new_distance_between;
                        point1 = sorted_by_x[i];
                        point2 = sorted_by_x[j];
                    }
                }
            }
        }

        (point1, point2, distance_between)
    }

    fn closest_pair(ax: &T, ay: &T) -> (Pair, Pair, f32) {
        let list_length = ax.len();
        if list_length <= 3 {
            return brute_check(ax);
        }
        // let left = 0;
        // let right = list_length;

        // let mid = left + (right - left) / 2;
        let mid = list_length / 2;
        let dummy_points = Pair { x: 0, y: 0 };
        let mut left_ax = vec![dummy_points; mid];
        let mut right_ax = vec![dummy_points; list_length - (mid + 1)];
        left_ax.clone_from_slice(&ax[..mid]);
        right_ax.clone_from_slice(&ax[mid..]);

        let mid_point = ax[mid].x;
        let mut left_ay: T = vec![];
        let mut right_ay: T = vec![];

        for point in ay {
            //split ay into 2 arrays using midpoint
            if point.x < mid_point {
                left_ay.push(*point);
            } else {
                left_ay.push(*point);
            }
        }

        let (left1, right1, distance_between1) = closest_pair(&left_ax, &left_ay);
        let (left2, right2, distance_between2) = closest_pair(&right_ax, &right_ay);

        // Determine smaller distance between points of 2 arrays;
        let mut distance_between: f32;
        let store_points;
        if distance_between1 <= distance_between2 {
            distance_between = distance_between1;
            store_points = (left1, right1);
        } else {
            distance_between = distance_between2;
            store_points = (left2, right2);
        }
        let (left3, right3, distance_between3) =
            closest_split_pair(ax, ay, distance_between, store_points);

        if distance_between <= distance_between3 {
            return (store_points.0, store_points.1, distance_between);
        } else {
            return (left3, right3, distance_between3);
        }
    }

    pub fn closest_split_pair(
        sorted_by_x: &T,
        sorted_by_y: &T,
        min_between: f32,
        best_pair: (Pair, Pair),
    ) -> (Pair, Pair, f32) {
        use std::cmp;
        let list_length = sorted_by_x.len();
        let mid_point_x = sorted_by_x[list_length / 2].x;

        // Create subarray of points not further than delta from the midpoint on x-sorted list
        let mut delta_y = vec![];
        for point in sorted_by_y {
            if ((mid_point_x as f32 - min_between) <= point.x as f32)
                && (point.x as f32 <= (mid_point_x as f32 + min_between))
            {
                delta_y.push(point)
            }
        }

        let mut best = min_between;
        let length_of_y = delta_y.len();

        for i in 0..(length_of_y - 1) {
            for j in i + 1..cmp::min(i + 5, length_of_y) {
                let left = delta_y[i];
                let right = delta_y[j];
                let distance = get_distance(*left, *right);
                best = distance;
            }
        }

        (best_pair.0, best_pair.1, best)
    }
}

// My Solution is completely based on this python implementation
// https://github.com/sudheernaidu53/Data-Structures-and-Algorithms-specialization-University-of-California-San-Diego/blob/494f557441d84e1c35ba296c6b91f825cede6e29/course%201%20-%20Algorithmic%20toolbox/week4_divide_and_conquer/6_closest_points/closest.py#L28
