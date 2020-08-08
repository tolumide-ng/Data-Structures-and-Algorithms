pub fn maximum_number_of_prizes(expected_sum: u32) {
    let mut pairwise_integers = Vec::new();
    if expected_sum < 2 {
        pairwise_integers.push(expected_sum);
        expected_sum;
    }
    for current_number in 1..=expected_sum {
        let mut current_sum: u32 = pairwise_integers.iter().sum();
        if current_sum == expected_sum {
            break;
        } else if (current_sum + current_number) < expected_sum {
            pairwise_integers.push(current_number);
            current_sum = pairwise_integers.iter().sum();
        // break;
        } else if (current_sum + current_number) == expected_sum {
            pairwise_integers.push(current_number);
            current_sum = pairwise_integers.iter().sum();
            break;
        } else if current_sum + current_number > expected_sum {
            loop {
                if current_sum + current_number > expected_sum {
                    pairwise_integers.pop();
                    current_sum = pairwise_integers.iter().sum();
                }
                if current_sum + current_number == expected_sum {
                    pairwise_integers.push(current_number);
                    current_sum = pairwise_integers.iter().sum();

                    break;
                }
                if current_sum + current_number < expected_sum {
                    pairwise_integers.push(current_number);
                    current_sum = pairwise_integers.iter().sum();

                    break;
                }
            }
        }
    }
    println!("length {}", pairwise_integers.len());
    println!("contents {:?}", pairwise_integers);
}

pub fn execute() {
    maximum_number_of_prizes(8);
    maximum_number_of_prizes(13);
    maximum_number_of_prizes(100);
}
