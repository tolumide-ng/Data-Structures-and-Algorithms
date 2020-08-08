use std::cmp;

pub fn comapre_values(a: &str, b: &str) -> bool {
    let first_combo = [a, b].join("");
    let second_combo = [b, a].join("");
    first_combo > second_combo
}

pub fn maximum_salary(values: &mut Vec<u32>) {
    let mut answer = String::from("");
    values.sort();
    loop {
        let highest = values[values.len() - 1];
        if values.len() == 1 {
            answer.push_str(values[0].to_string().as_str());
            values.pop();
            break;
        }

        println!("all values {:?}", values);

        for index in 0..values.len() - 1 {
            println!("current index {}", index);
            let result = comapre_values(
                values[index].to_string().as_str(),
                highest.to_string().as_str(),
            );
            if result {
                answer.push_str(values[index].to_string().as_str());
                values.remove(index);
                break;
            } else {
                answer.push_str(highest.to_string().as_str());
                values.pop();
                break;
            }
        }
    }

    println!("the answer {}", answer);
}

pub fn execute() {
    // let mut testing_vec = vec![9, 4, 6, 1, 9];
    // let mut testing_vec = vec![21, 2];
    let mut testing_vec = vec![23, 39, 92];
    maximum_salary(&mut testing_vec);
}
