pub struct Elements<'a> {
    string_one_length: usize,
    string_one: &'a str,
    string_two_length: usize,
    string_two: &'a str,
}

pub fn execute() {
    let element = Elements {
        string_one_length: 6,
        string_one: "babbab",
        string_two_length: 6,
        string_two: "abaaba",
    };

    lcs(element);
}

fn lcs(elem: Elements) -> u8 {
    use std::cmp;

    let mut comparison_store =
        vec![vec![0; elem.string_two_length + 1]; elem.string_one_length + 1];
    let mut second_string = elem.string_two.chars();

    for two_value in 0..=elem.string_two_length {
        let mut first_string = elem.string_one.chars();
        let mut val_at_two = String::new();
        if two_value > 0 {
            val_at_two = second_string.next().unwrap().to_string();
        }

        for one_value in 0..=elem.string_one_length {
            if two_value == 0 || one_value == 0 {
                continue;
            }
            if one_value >= 1 && two_value >= 1 {
                let val_at_one = first_string.next().unwrap().to_string();
                if val_at_one != val_at_two {
                    let left = comparison_store[one_value - 1][two_value];
                    let top = comparison_store[one_value][two_value - 1];
                    let result = cmp::max(left, top);
                    comparison_store[one_value][two_value] = result;
                }
                if val_at_one == val_at_two {
                    let diagonal = comparison_store[one_value - 1][two_value - 1] + 1;
                    comparison_store[one_value][two_value] = diagonal;
                }
            }
        }
    }

    println!(
        "Result: {:?}",
        comparison_store[elem.string_one_length][elem.string_two_length] // comparison_store
    );

    comparison_store[elem.string_one_length][elem.string_two_length]
}
