// Probable desire of the question was to create an answer to this problem using a linked list,
// but a vector works fine too so I used the vec

#[derive(Debug)]
pub struct BracketFormat {
    bracket_type: String,
    index: usize,
}

#[derive(Debug)]
pub struct BracketResult<'a> {
    status: &'a str,
    index: Option<usize>,
}

pub fn linked_brackets<'a>(elem: &'a mut str) -> BracketResult {
    let mut bracket_store: Vec<BracketFormat> = vec![];
    let mut char_index: usize = 0;

    for char in elem.chars() {
        let char_string = char.to_string();
        if char_string == "(" || char_string == "{" || char_string == "[" {
            bracket_store.push(BracketFormat {
                bracket_type: char_string,
                index: char_index,
            });
        } else if char_string == ")" || char_string == "}" || char_string == "]" {
            let current_store_value = &bracket_store[&bracket_store.len() - 1].bracket_type;
            if current_store_value == "(" && char_string == ")" {
                bracket_store.pop();
            } else if current_store_value == "{" && char_string == "}" {
                bracket_store.pop();
            } else if current_store_value == "[" && char_string == "]" {
                bracket_store.pop();
            } else {
                char_index += 1;
                break;
            }
        }
        char_index += 1;
    }
    // if char_index != elem.len() {}
    if bracket_store.len() > 0 {
        BracketResult {
            status: "Failure",
            index: Some(char_index),
        }
    } else {
        BracketResult {
            status: "Success",
            index: None,
        }
    }
}

pub fn execute() {
    // let mut value = String::from("{[]}()");
    // let mut value = String::from("{");
    let mut value = String::from("foo({bar)");
    println!("RESULT ===>>>> {:?}", linked_brackets(&mut value));
}

#[cfg(test)]
mod test {
    use super::linked_brackets;

    #[test]
    fn correct_brackets() {
        let mut value = String::from("{[]}()");

        assert_eq!(linked_brackets(&mut value).status, "Success");
        assert_eq!(linked_brackets(&mut value).index, None);
    }

    #[test]
    fn wrong_brackets() {
        let mut value = String::from(
            "{Sometimes humans don't always 
        think whent (using code linter's because we trust them () to do the [0] work)",
        );

        assert_eq!(linked_brackets(&mut value).status, "Failure");
        assert_eq!(linked_brackets(&mut value).index, Some(117));
    }
}
