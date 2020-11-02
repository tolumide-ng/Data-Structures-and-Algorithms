pub fn execute() {
    let mut bin_heap = BinaryHeap::new(20);
    bin_heap.insert((2, "f"));
    bin_heap.insert((10, "f"));
    bin_heap.insert((5, "f"));
    bin_heap.insert((1, "c"));
    bin_heap.insert((12, "f"));
    bin_heap.insert((2, "j"));
    bin_heap.insert((1, "d"));
    println!("EXTRACT MIN {:?}", bin_heap.extract_min());
    println!("EXTRACT MIN {:?}", bin_heap.extract_min());
    println!("EXTRACT MIN {:?}", bin_heap.extract_min());
    bin_heap.remove(1);
}

#[derive(Debug, Clone, Copy)]
pub struct Node<T> {
    pub weight: i8,
    pub node: T,
    time_added: u8,
}

#[derive(Debug, Clone)]
pub struct BinaryHeap<T> {
    heap: Vec<Node<T>>,
    max_size: usize,
    size: usize,
    realistic_size: usize,
}

use std::fmt::{Debug, Display};

impl<T> BinaryHeap<T>
where
    T: PartialEq + Copy + Clone + Display + Debug,
{
    pub fn new(max_size: usize) -> Self {
        BinaryHeap {
            max_size,
            heap: Vec::new(),
            size: 0,
            realistic_size: 0,
        }
    }

    pub fn get_parent(&self, index: usize) -> Option<usize> {
        if index < self.size && index > 0 {
            if index % 2 == 0 {
                return Some(index / 2);
            } else {
                return Some((index - 1) / 2);
            }
        }
        return None;
    }

    pub fn get_left_child(&self, index: usize) -> Option<usize> {
        if (index == 0 && self.heap.len() > 1) {
            return Some(1);
        }
        if index * 2 < self.heap.len() {
            return Some(index * 2);
        } else {
            return None;
        }
    }

    pub fn get_right_child(&self, index: usize) -> Option<usize> {
        if index == 0 && self.heap.len() > 2 {
            return Some(2);
        }
        if (index * 2) + 1 < self.heap.len() {
            return Some(index * 2 + 1);
        }
        None
    }

    pub fn sift_up(&mut self, index: usize) {
        let mut val = index;
        loop {
            if val > 0 {
                let parent = &self.heap[self.get_parent(val).unwrap()];
                let current = self.heap[val];
                if parent.weight > current.weight
                    || (parent.weight == current.weight && parent.time_added > current.time_added)
                {
                    let curr_value = self.heap[val];
                    let parent_index = self.get_parent(val).unwrap();
                    let curr_parent = self.heap[parent_index];
                    std::mem::replace(&mut self.heap[val], curr_parent);
                    std::mem::replace(&mut self.heap[parent_index], curr_value);
                    val = parent_index;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    pub fn sift_down(&mut self, index: usize) {
        let mut new_index = index;
        let left_child_index = self.get_left_child(index);

        match left_child_index {
            Some(left_child_index) => {
                if left_child_index <= self.size {
                    let current = self.heap[new_index];
                    let child = self.heap[left_child_index];
                    if (current.weight > child.weight)
                        || (current.weight == child.weight && current.time_added > child.time_added)
                    {
                        new_index = left_child_index;
                    }
                }
            }
            None => {}
        }

        let right_child_index = self.get_right_child(index);

        match right_child_index {
            Some(right_child_index) => {
                if (right_child_index <= self.size) {
                    let current = self.heap[new_index];
                    let child = self.heap[right_child_index];
                    if current.weight > child.weight
                        || (current.weight == child.weight && current.time_added > child.time_added)
                    {
                        new_index = right_child_index;
                    }
                }
            }
            None => {}
        }

        if new_index != index {
            let child = self.heap[new_index];
            let curr_val = self.heap[index];

            std::mem::replace(&mut self.heap[new_index], curr_val);
            std::mem::replace(&mut self.heap[index], child);

            self.sift_down(new_index);
        }
    }

    pub fn insert(&mut self, value: (i8, T)) {
        if self.size != self.max_size {
            self.size += 1;
            self.heap.push(Node {
                weight: value.0,
                node: value.1,
                time_added: (self.size) as u8,
            });

            self.sift_up(self.heap.len() - 1);
        }
    }

    pub fn extract_min(&mut self) -> Option<Node<T>> {
        if self.heap.len() > 1 {
            let result = self.heap[0];
            let last_value = self.heap.pop().unwrap();
            std::mem::replace(&mut self.heap[0], last_value);
            self.size -= 1;
            self.sift_down(0);
            return Some(result);
        } else if self.heap.len() == 1 {
            return self.heap.pop();
        }
        return None;
    }

    pub fn remove(&mut self, index: usize) -> Option<Node<T>> {
        if index < self.size {
            let result = self.heap[index];
            std::mem::replace(
                &mut self.heap[index],
                Node {
                    weight: f64::NEG_INFINITY as i8,
                    node: result.node,
                    time_added: (self.size + 1) as u8,
                },
            );

            self.sift_up(index);
            return self.extract_min();
        }

        return None;
    }

    pub fn change_priority(&mut self, index: usize, new_value: Node<T>) {
        let old_value = self.heap[index];
        std::mem::replace(&mut self.heap[index], new_value);
        if new_value.weight > old_value.weight {
            self.sift_down(index);
        } else {
            self.sift_up(index);
        }
    }

    pub fn total_length(&mut self) -> usize {
        return self.heap.len();
    }
}

#[cfg(test)]
mod test {
    use super::BinaryHeap;

    #[test]
    fn priority_queue() {
        let mut bin_heap = BinaryHeap::new(20);
        bin_heap.insert((2, "f"));
        bin_heap.insert((10, "f"));
        bin_heap.insert((5, "f"));
        bin_heap.insert((1, "c"));
        bin_heap.insert((12, "f"));
        bin_heap.insert((2, "j"));
        bin_heap.insert((1, "d"));
        let result = bin_heap.extract_min().unwrap();

        assert_eq!(result.weight, 1);
        assert_eq!(result.node, "c");
        assert_eq!(result.time_added, 4);

        let second_result = bin_heap.extract_min().unwrap();

        assert_eq!(second_result.weight, 1);
        assert_eq!(second_result.node, "d");
        assert_eq!(second_result.time_added, 7);

        let third_result = bin_heap.extract_min().unwrap();

        assert_eq!(third_result.weight, 2);
        assert_eq!(third_result.node, "f");
        assert_eq!(third_result.time_added, 1);
    }
}
