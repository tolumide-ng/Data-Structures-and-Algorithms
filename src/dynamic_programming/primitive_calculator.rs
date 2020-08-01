pub fn execute() {
    primitive_calculator(10)
}

pub fn primitive_calculator(number: usize) {
    use std::cmp;
    // let mut money_coins = vec![vec![0; coin_denomination.len()]; money + 1];
    // allowed operators are =>>>> ["+1", "*2", "*3"]
    let operators: usize = 3;
    // let mut possible_operators = vec![0; vec![0; OPERATORS]];
    let mut min_operators = vec![vec![0; operators]; number];

    for n in 0..number {
        for ops in 0..operators {
            // println!("mnumber = {}, ops = {}", n, ops);
            // n is already 1

            // else if ops == 0 {
            //     min_operators[n][ops] = min_operators[n][ops - 1] + 1;
            // }

            if n == 0 {
                min_operators[n][ops] = 0;
            } else {
                let num_value = n + 1;

                let mut far_one = min_operators[n - 1][ops];
                let mut min_value = far_one + 1;
                let far_two;
                let far_three;
                if n >= 1 {
                    if (num_value - 1) + 1 == num_value
                        || (ops >= 1 && (num_value - 1) * 2 == num_value)
                        || (ops == 2 && (num_value - 1) * 3 == num_value)
                    {
                        far_one = min_operators[n - 1][ops] + 1;
                        min_value = cmp::min(far_one, min_value);
                        // println!("far one values {} {}", far_one, min_value);
                    }
                }
                if n >= 2 {
                    if (num_value - 2) + 1 == num_value
                        || (ops >= 1 && (num_value - 2) * 2 == num_value)
                        || (ops == 2 && (num_value - 2) * 3 == num_value)
                    {
                        far_two = min_operators[n - 2][ops] + 1;
                        min_value = cmp::min(far_two, min_value);
                    }
                }
                if n >= 3 {
                    if (num_value - 3) + 1 == num_value
                        || (ops >= 1 && (num_value - 3) * 2 == num_value)
                        || (ops == 2 && (num_value - 3) * 3 == num_value)
                    {
                        far_three = min_operators[n - 3][ops] + 1;
                        min_value = cmp::min(far_three, min_value);
                    }
                }
                min_operators[n][ops] = min_value;
            }
        }
    }

    println!("the whole {:?}", min_operators);
    println!("FACTS ONLY!! {}", min_operators[number - 1][operators - 1]);
}
