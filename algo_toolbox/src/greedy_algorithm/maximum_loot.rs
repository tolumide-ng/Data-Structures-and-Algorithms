use std::cmp::Ordering;
// use std::io;

#[derive(Debug)]
struct KnapSack {
    max_items: u64,
    capacity: u64,
    current_weight: u64,
    total_items: u64,
    total_values: f64,
}

impl KnapSack {
    pub fn add_item(&mut self, amount: f64, weight: u64) {
        self.total_values += amount;
        self.current_weight += weight;
        self.total_items += 1;
    }
}

#[derive(Debug)]
struct Item {
    value: u64,
    weight: u64,
    remaining: f64,
    worth: f64,
}

impl Item {
    pub fn new() -> Self {
        Item {
            value: 0,
            weight: 0,
            remaining: 0.00,
            worth: 0.00,
        }
    }

    pub fn update(&mut self, value: u64, weight: u64) {
        self.value = value;
        self.weight = weight;
        self.worth = value as f64 / weight as f64;
        self.remaining = weight as f64;
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.worth.partial_cmp(&other.worth)
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.worth == other.worth
    }
}

pub fn max_loot() {
    let mut the_bag = KnapSack {
        max_items: 3,
        capacity: 50,
        current_weight: 0,
        total_items: 0,
        total_values: 0.00,
    };

    let mut item1 = Item::new();
    let mut item2 = Item::new();
    let mut item3 = Item::new();
    item1.update(60, 20);
    item2.update(120, 30);
    item3.update(100, 50);

    let mut all_items = vec![item1, item2, item3];

    all_items.sort_by(|a, b| b.worth.partial_cmp(&a.worth).unwrap());

    for the_item in all_items.iter() {
        if the_bag.current_weight != the_bag.capacity {
            let can_take_weight = the_bag.capacity - the_bag.current_weight;
            if can_take_weight <= 0 {
                break;
            }
            if the_bag.total_items < the_bag.max_items && the_item.weight <= can_take_weight {
                the_bag.add_item(the_item.value as f64, the_item.weight);
            } else if the_bag.total_items < the_bag.max_items {
                the_bag.add_item(
                    (the_bag.capacity - the_bag.current_weight) as f64 * the_item.worth,
                    the_item.weight,
                );
            }
        }
    }

    println!("{:?}", the_bag.total_values)
}

pub fn execute() {
    max_loot();
}
