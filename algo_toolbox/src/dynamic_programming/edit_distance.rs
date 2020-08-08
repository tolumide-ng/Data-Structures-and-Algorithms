#[derive(Debug)]
pub struct Strings<'a> {
    string_one: &'a str,
    string_two: &'a str,
}

pub fn execute() {
    let compare = Strings {
        string_one: "short",
        string_two: "ports",
    };
    let result = edit_distance(compare);
    println!("edit distance is ===>>> {}", result);
}

// table ==> string_one ===> x-axis (one_val)
//       ==> string_two ===> y-axis (two_val)

fn edit_distance(input: Strings) -> u8 {
    use std::cmp;
    let mut compare_store = vec![vec![0; input.string_two.len() + 1]; input.string_one.len() + 1];

    let mut second_string = input.string_two.chars();

    for two_val in 0..=input.string_two.len() {
        let mut val_at_two: String = String::from("");
        let mut first_string = input.string_one.chars();

        if two_val >= 1 {
            val_at_two = second_string.next().unwrap().to_string();
        }
        for one_val in 0..=input.string_one.len() {
            if two_val == 0 && one_val >= 1 {
                compare_store[one_val][two_val] = compare_store[one_val - 1][two_val] + 1;
            }

            if one_val == 0 && two_val >= 1 {
                compare_store[one_val][two_val] = compare_store[one_val][two_val - 1] + 1;
            }

            if two_val >= 1 && one_val != 0 {
                let val_at_one = first_string.next().unwrap();
                if val_at_one.to_string() != val_at_two {
                    let back = compare_store[one_val - 1][two_val] + 1;
                    let top = compare_store[one_val][two_val - 1] + 1;
                    let diagonal = compare_store[one_val - 1][two_val - 1] + 1;
                    let mut result = cmp::min(back, top);
                    result = cmp::min(result, diagonal);
                    compare_store[one_val][two_val] = result;
                }

                if val_at_one.to_string() == val_at_two {
                    compare_store[one_val][two_val] = compare_store[one_val - 1][two_val - 1]
                }
            }
        }
    }

    // let mut get_first_string = input.string_one.chars();
    // let mut get_second_string = input.string_two.chars();
    // let mut first_string_index = input.string_one.len() - 1;
    // let mut second_string_index = input.string_two.len() - 1;

    // loop {
    //     let top = compare_store[first_string_index][second_string_index - 1];
    //     let bottom = compare_store[first_string_index - 1][second_string_index];
    //     let diagonal = compare_store[first_string_index - 1][second_string_index - 1];
    // &input.string_one[len-2..][0..1]

    //     let a = get_first_string.next().unwrap().to_string();
    //     // let b
    // }

    compare_store[input.string_one.len() - 1][input.string_two.len() - 1]
}
