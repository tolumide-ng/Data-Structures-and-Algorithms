// use std::io;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug)]
struct Item {
    value: u64,
    weight: u64,
    remaining: f64,
}

#[derive(Debug)]
struct Actual {
    value: f64,
    total: Rc<RefCell<Item>>,
}

// struct Actual<'a> {
//     value: f64,
//     total: Rc<RefCell<&'a Item>>,
// }

impl Actual {
    pub fn reduce(&mut self, amount: f64) {
        // self.total -= amount;
        self.total.borrow_mut().remaining -= amount;
    }
}

impl PartialOrd for Actual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialEq for Actual {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Item {
    pub fn actual_worth(self) -> Actual {
        Actual {
            value: self.value as f64 / self.weight as f64,
            // total: self.weight as f64,
            total: Rc::new(RefCell::new(self)),
        }
    }

    // pub fn reduce(&mut self, amount: f64) {
    //     // self.remaining -= amount;
    //     // self.total -= amount;
    //     // self.total.unwrap()
    // }
}

struct KnapSack {
    max_items: u64,
    capacity: f64,
    current: f64,
    total_items: u64,
}

impl KnapSack {
    pub fn add_item(&mut self, amount: f64) {
        // &self.current = &amount;
        self.current += amount;
        // self.current = *&self.current + amount;
    }
}

pub fn max_loot() {
    let mut the_bag = KnapSack {
        max_items: 3,
        capacity: 50.00,
        current: 0.00,
        total_items: 0,
    };

    let item1 = Item {
        value: 60,
        weight: 20,
        remaining: 20.00,
    };
    let item3 = Item {
        value: 120,
        weight: 30,
        remaining: 30.00,
    };
    let item2 = Item {
        value: 100,
        weight: 50,
        remaining: 100.00,
    };

    let all_items = vec![item1, item2, item3];
    let mut actual_worth: Vec<Actual> = Vec::new();

    for specific_item in all_items.iter() {
        actual_worth.push(specific_item.actual_worth());
    }
    actual_worth.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap());

    for the_item in actual_worth.iter() {
        if the_bag.current != the_bag.capacity {
            let can_take = the_bag.capacity - the_bag.current;
            if the_bag.total_items < the_bag.max_items
                && the_item.total.borrow().remaining as f64 <= can_take
            {
                the_bag.add_item(the_item.total.borrow().remaining as f64);
                the_item.reduce(the_item.total.borrow_mut().remaining as f64);
            } else if the_bag.total_items < the_bag.max_items {
                the_bag.add_item(can_take);
                the_item.reduce(can_take);
            }
            // the_item.reduce()
        }
    }
    // while the_bag.current != the_bag.capacity {}

    // for val in actual_worth.iter() {
    //     println!("{:?}", val);
    // }
}

// pub fn execute() {
//     io::stdin()
//         .read_line(&mut number)
//         .expect("Failed to read line");

//     let number: u32 = match number.trim().parse() {
//         Ok(num) => num,
//         Err(_) => 0,
//     };
// }

// pub fn execute() {
//     // Get knap sack details
//     let mut knap_sack_details = String::new();
//     println!("\n Enter the max items and capacity the knap sack can retain separated by space: ");
//     io::stdin()
//         .read_line(&mut knap_sack_details)
//         .expect("Failed to read line");

//     let knap_info = &knap_sack_details
//         .trim()
//         .split(' ')
//         .flat_map(str::parse::<i32>)
//         .collect::<Vec<_>>();

//     let mut all_items = Vec::new();

//     loop {
//         let mut number = String::new();

//         println!("\nEnter two positive integers separated by a space: ");

//         io::stdin()
//             .read_line(&mut number)
//             .expect("Failed to read line");

//         let splitted_line = number
//             .trim()
//             .split(' ')
//             .flat_map(str::parse::<i32>)
//             .collect::<Vec<_>>();

//         if *&splitted_line.len() == 0 {
//             break;
//         }
//         let mut item = Item::new();
//         item.update(splitted_line[0] as u64, splitted_line[1] as u64);
//         all_items.push(item);
//     }

//     // let mut the_bag = KnapSack {
//     //     max_items: 3,
//     //     capacity: 50,
//     //     current_weight: 0,
//     //     total_items: 0,
//     //     total_values: 0.00,
//     // };

//     let mut the_bag = KnapSack {
//         max_items: knap_info[0] as u64,
//         capacity: knap_info[1] as u64,
//         current_weight: 0,
//         total_items: 0,
//         total_values: 0.00,
//     };

//     // let mut item1 = Item::new();
//     // let mut item2 = Item::new();
//     // let mut item3 = Item::new();
//     // item1.update(60, 20);
//     // item2.update(120, 30);
//     // item3.update(100, 50);

//     // let mut all_items = vec![item1, item2, item3];

//     // println!("number entered {:?}", splitted_line);

//     max_loot(&mut all_items, &mut the_bag);
// }
