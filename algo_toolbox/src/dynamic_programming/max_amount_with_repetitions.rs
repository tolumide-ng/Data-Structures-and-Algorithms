// My implementation of a solution to this problem would be assuming that the
// worth of each bar of gold is different from one another

// use std::collections::HashMap;
use std::cmp;

#[derive(Debug, Clone, Copy)]
pub struct KnapSack {
    weight: usize,
    value: usize,
}

impl KnapSack {
    fn new(weight: usize, value: usize) -> KnapSack {
        return KnapSack { weight, value };
    }
}

pub fn execute() {
    let mut store = vec![
        KnapSack::new(3, 2),
        KnapSack::new(4, 3),
        KnapSack::new(6, 1),
        KnapSack::new(5, 4),
    ];

    let result = knapsack_without_repetition(&mut store, 8);
    println!("MAX PROFIT: >> {}", result);
}

fn knapsack_without_repetition(knap_sack: &mut Vec<KnapSack>, max_size: usize) -> usize {
    knap_sack.sort_by(|a, b| a.weight.cmp(&b.weight));

    let mut knap_store = vec![vec![0; knap_sack.len() + 1]; max_size + 1];

    for gold_value in 1..=knap_sack.len() {
        for size_value in 1..=max_size {
            if size_value >= knap_sack[gold_value - 1].weight {
                let remainder = size_value - knap_sack[gold_value - 1].weight;
                let remainder_value = knap_store[remainder][gold_value - 1];
                let total_value = remainder_value + knap_sack[gold_value - 1].value;

                knap_store[size_value][gold_value] =
                    cmp::max(total_value, knap_store[size_value][gold_value - 1]);
            } else {
                knap_store[size_value][gold_value] = knap_store[size_value][gold_value - 1];
            }
        }
    }

    // get selected coins/gold into a the knap_sack
    let mut current_gold = knap_sack.len();
    let mut current_size = max_size;
    let mut max_profit = knap_store[max_size][knap_sack.len()];
    let mut selected_knap_sacks: Vec<KnapSack> = vec![];

    loop {
        if current_size == 0 || current_gold == 0 {
            break;
        }
        if knap_store[current_size][current_gold] == knap_store[current_size][current_gold - 1] {
            current_gold -= 1;
            continue;
        }

        if knap_store[current_size][current_gold] != knap_store[current_size][current_gold - 1] {
            let the_knap = knap_sack[current_gold - 1];
            selected_knap_sacks.push(the_knap);

            max_profit -= the_knap.value;
            current_gold -= 1;
            while knap_store[current_size][current_gold] != max_profit {
                current_size -= 1;
            }

            continue;
        }
    }

    println!("items in the knap sack >>>  {:?}", selected_knap_sacks);

    // println!("results ===> {:?}", knap_store);
    knap_store[max_size][knap_sack.len()]
}
