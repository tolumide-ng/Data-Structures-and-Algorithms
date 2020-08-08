#[derive(Debug, Clone)]
struct Details<'a> {
    operators: Vec<&'a str>,
    operands: Vec<i16>,
}

struct MinMaxDictionary {
    minimum: i16,
    maximum: i16,
}

pub fn execute() {
    let info = Details {
        operators: vec!["-", "+", "*", "-", "+"],
        operands: vec![5, 8, 7, 4, 8, 9],
    };

    println!("Result =======<>>> {}", max_num_operands(info));
}

fn max_num_operands(info: Details) -> i16 {
    let operands_length = info.operands.len();
    let mut min_store: Vec<Vec<i16>> = vec![vec![0; operands_length]; operands_length];
    let mut max_store: Vec<Vec<i16>> = vec![vec![0; operands_length]; operands_length];

    for i in 0..operands_length {
        min_store[i][i] = info.operands[i as usize];
        max_store[i][i] = info.operands[i as usize];
    }

    // println!("min store >>>>>>>> {:?}", min_store);
    // println!("min store >>>>>>>> {:?}", max_store);

    for value_one in 1..operands_length {
        for value_two in 0..(operands_length - value_one) {
            let total_value = value_one + value_two;

            let result = min_max_calc(
                &max_store,
                &min_store,
                value_two,
                total_value,
                &info.operators,
            );
            min_store[value_two][total_value] = result.minimum;
            max_store[value_two][total_value] = result.maximum;
            // println!("edited <>>>>>>>>>>>{:?}", max_store);
        }
    }
    println!("MAX STORE {:?}", max_store);
    println!("MIN STORE {:?}", min_store);
    max_store[0][operands_length - 1]
}

fn min_max_calc<'a>(
    max_store: &Vec<Vec<i16>>,
    min_store: &Vec<Vec<i16>>,
    first_value: usize,
    second_value: usize,
    operators: &Vec<&'a str>,
) -> MinMaxDictionary {
    let mut min_value = 0;
    let mut max_value = 0;
    use std::cmp;

    println!(
        "first value ======>>>>> {:?}, {:?}",
        first_value, second_value
    );
    // println!("second value ======>>>>> {:?}", second_value);

    for value in first_value..second_value {
        let a = calc(
            max_store[first_value][value] as i16,
            max_store[value + 1][second_value] as i16,
            operators[value as usize],
        );
        let b = calc(
            max_store[first_value][value] as i16,
            max_store[value + 1][second_value] as i16,
            operators[value as usize],
        );
        let c = calc(
            max_store[first_value][value] as i16,
            max_store[value + 1][second_value] as i16,
            operators[value as usize],
        );
        let d = calc(
            max_store[first_value][value] as i16,
            max_store[value + 1][second_value] as i16,
            operators[value as usize],
        );
        min_value = *((vec![a, b, c, d]).iter().min().unwrap());
        max_value = *vec![a, b, c, d].iter().max().unwrap();
    }

    MinMaxDictionary {
        minimum: min_value,
        maximum: max_value,
    }
}

fn calc<'a>(first: i16, second: i16, op: &'a str) -> i16 {
    println!(
        "all of em ---------->>>>>>>>>>>>>>> first {}, second {}, op {}",
        first, second, op,
    );
    if op == "+" {
        first + second
    } else if op == "-" {
        first - second
    } else {
        first * second
    }
}
