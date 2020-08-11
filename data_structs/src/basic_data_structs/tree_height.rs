use std::cmp::Ordering;
use std::rc::Rc;
use std::rc::Weak;

pub fn execute() {
    // let mut value = "0 1 0 1 0".to_string();
    let mut value = "-1 0 4 0 3".to_string();
    // let mut value = "4 -1 4 1 1".to_string();
    let result = tree_height(&mut value);
    println!("THE RESULT >>>>>>>> {:?}", result);
}

#[derive(Debug)]
pub struct Node {
    value: i8,
    index: usize,
    owner: Option<Weak<Node>>,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value >= other.value
    }
}

pub fn tree_height<'a>(elem: &'a mut str) -> usize {
    let mut node_store: Vec<Node> = vec![];
    let mut char_index = 0;
    let elements = elem.split(" ");

    for char in elements {
        let value = char.to_string();
        let num = value.parse::<i8>().unwrap();

        let node = Node {
            value: num,
            index: char_index,
            owner: None,
        };
        node_store.push(node);
        char_index += 1;
    }

    node_store.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());

    // create the tree
    let mut the_tree_pointer = &node_store[0];
    let mut depth = 1;

    for node_index in 1..node_store.iter().len() {
        // if node_store[node_index].value > the_tree.value {
        //     node_store[node_index].owner = Some(Rc::clone(&the_tree))
        // }
        if node_store[node_index].value > the_tree_pointer.value {
            the_tree_pointer = &node_store[node_index];
            depth += 1;
        }
    }
    return depth;
}

#[cfg(test)]
mod test {
    use super::tree_height;

    #[test]
    fn test_tree_height() {
        let mut tree = "0 1 0 1 0".to_string();

        assert_eq!(tree_height(&mut tree), 2);
    }

    #[test]
    fn test_one_more_tree_height() {
        let mut tree = "4 -1 4 1 1".to_string();
        assert_eq!(tree_height(&mut tree), 3);
    }
}
