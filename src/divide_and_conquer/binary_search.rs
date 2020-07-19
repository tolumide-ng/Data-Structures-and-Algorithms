pub struct Container {
    length: u32,
    content: Vec<u32>,
}

impl Container {
    fn new(a: Vec<u32>) -> Self {
        let length = a[0];

        Container { length, content: a }
    }
}

pub fn binary_search_recursive(
    dictionary: Container,
    search_item: u32,
    low: u32,
    high: u32,
) -> i32 {
    let dictionary_length = dictionary.length;

    if high < low {
        println!("high >>>> {}", high);
        println!("low >>>> {}", low);
        return -1;
    }

    let mid_point = (low + ((high - low) / 2)) as usize;
    // (1) ===> midpoint = 1 + (5-1)/2 => 3
    // dic[3] ===> 8  <-------------<------------<-------|
    // <==== info ====>                                  |
    // dic[5, 1, 5, 8, 12, 13], 5, 1, 2                  |
    // (2) ===> midpoint = 1 + (2-1)/2 => 1              |
    // dic[1] ===>1                                      |
    // <=== info ===>                                    |
    // dic[5, 1, 5, 8, 12, 13], 5, 2, 5                  |
    // (3) ===> midpoint = 2 + (5-2)/2 = 3               |
    // dic[3] ===> 8--------->----------->----->---------|
    // THE RECURSION NEVER ENDS AND SO THE THREAD PAINCKS AFTER THE STACK IS FULL
    //  I DEFINITELY/PROBABLY MADE A MISTAKE HERE, SO I WOULD BE GOING WITH THE NON-RECURSIVE IMPLEMENTATION

    if dictionary.content[mid_point] == search_item {
        return mid_point as i32 - 1;
    } else if dictionary.content[mid_point] > search_item {
        return binary_search_recursive(dictionary, search_item, 1, (mid_point - 1) as u32);
    } else {
        return binary_search_recursive(
            dictionary,
            search_item,
            mid_point as u32 + 1,
            dictionary_length,
        );
    }
    // return result;
}

pub fn binary_search(dictionary: Container, search_item: u32) -> i32 {
    let dic_length = dictionary.length;
    let mut left: usize = 1;
    let mut right = dictionary.length as usize;

    loop {
        if left <= right {
            let mid = left + ((right - left) / 2);
            if dictionary.content[mid] == search_item {
                return mid as i32 - 1;
            }
            if search_item < dictionary.content[mid] {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        } else {
            // println!("_++++++++++++NOT FOUND++++++++++_");
            return -1;
        }
    }
}

// 5, 8, 1, 23, 1, 11
// 2, 0, -1, 0, -1

pub fn execute() {
    // let dictionary = Container::new(vec![5, 1, 5, 8, 12, 13]);
    let searches = Container::new(vec![5, 8, 1, 23, 1, 11]);
    // let dic_length = dictionary.length;
    // let low = 1;

    // let result = binary_search_recursive(dictionary, 5, low, dic_length as u32);
    let mut result_vec = Vec::new();
    for index in 1..searches.content.len() {
        let dictionary = Container::new(vec![5, 1, 5, 8, 12, 13]);
        // println!("<<<<<<+++++++++++++++++++>>>>>>>>>>");
        // println!("the search content {}", searches.content[index]);
        let result = binary_search(dictionary, searches.content[index]);
        result_vec.push(result);
        // println!(
        //     "<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<the result is {}>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>",
        //     result
        // );
    }

    println!("OUTPUT: {:?}", result_vec);
}
