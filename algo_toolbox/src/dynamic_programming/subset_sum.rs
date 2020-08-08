pub fn execute() {
    // let mut souvenirs = vec![17, 59, 34, 57, 17, 23, 67, 1, 18, 2, 59];
    let mut souvenirs = vec![3, 3, 3];
    println!("Result ==> {}", subset_sum(&mut souvenirs));
}

pub fn subset_sum(souvenirs: &mut Vec<u32>) -> u8 {
    let total_friends = 3;
    let mut per_person: u32 = souvenirs.iter().sum();

    if per_person % total_friends > 0 {
        return 0;
    }

    per_person = per_person / total_friends;

    let max_val = souvenirs.iter().max();

    if max_val.unwrap() > &per_person {
        return 0;
    }

    let mut souvenir_store = vec![vec![0; souvenirs.len()]; (per_person + 1) as usize];

    for individual_sourvenir in 0..souvenirs.len() {
        for person_value in 0..=per_person {
            if person_value == 0 {
                souvenir_store[person_value as usize][individual_sourvenir] = 1;
            } else if individual_sourvenir == 0 {
                if souvenirs[individual_sourvenir] == per_person {
                    souvenir_store[person_value as usize][individual_sourvenir] = 1;
                } else {
                    souvenir_store[person_value as usize][individual_sourvenir] = 0;
                }
            } else if souvenirs[individual_sourvenir] < person_value {
                souvenir_store[per_person as usize][individual_sourvenir] =
                    souvenir_store[per_person as usize][individual_sourvenir - 1]
            } else if souvenirs[individual_sourvenir] == person_value {
                souvenir_store[person_value as usize][individual_sourvenir] = 1;
            } else {
                if souvenir_store[person_value as usize][individual_sourvenir - 1] == 1 {
                    souvenir_store[person_value as usize][individual_sourvenir] = 1
                } else {
                    let remainder: usize =
                        (souvenirs[individual_sourvenir] - person_value) as usize;
                    souvenir_store[person_value as usize][individual_sourvenir] =
                        souvenir_store[remainder][individual_sourvenir]
                }
            }
        }
    }

    println!("{:?}", souvenir_store);
    souvenir_store[per_person as usize][souvenirs.len() - 1]
}

// using https://github.com/sudheernaidu53/Data-Structures-and-Algorithms-specialization-University-of-California-San-Diego/blob/master/course%201%20-%20Algorithmic%20toolbox/week6_dynamic_programming2/2_partitioning_souvenirs/partition3.py original implmentation
