pub fn execute() {
    let mut bin_heap = BinaryHeap::new(20);
    bin_heap.insert((2, "f"));
    bin_heap.insert((10, "f"));
    bin_heap.insert((5, "f"));
    bin_heap.insert((1, "f"));
    bin_heap.insert((12, "f"));
    bin_heap.insert((2, "f"));
    println!("EXTRACT MIN {:?}", bin_heap.extract_min());
    bin_heap.remove(1);
}

#[derive(Debug, Clone)]
pub struct BinaryHeap<T> {
    heap: Vec<(i8, T)>,
    max_size: usize,
    size: usize,
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
        if (index == 0 && self.size > 0) {
            return Some(1);
        }
        if index * 2 < self.size {
            return Some(index * 2);
        } else {
            return None;
        }
    }

    pub fn get_right_child(&self, index: usize) -> Option<usize> {
        if (index == 0 && self.size > 0) {
            return Some(index + 2);
        }
        if (index * 2) + 1 < self.size {
            return Some(index * 2 + 1);
        }
        None
    }

    pub fn sift_up(&mut self, index: usize) {
        let mut val = index;
        loop {
            if val > 0 && (&self.heap[self.get_parent(val).unwrap()]).0 > self.heap[val].0 {
                let curr_value = self.heap[val];
                let parent_index = self.get_parent(val).unwrap();
                let curr_parent = self.heap[parent_index];

                std::mem::replace(&mut self.heap[val], curr_parent);
                std::mem::replace(&mut self.heap[parent_index], curr_value);
                val = parent_index;
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
                if (left_child_index <= self.size)
                    && (self.heap[new_index].0 > self.heap[left_child_index].0)
                {
                    new_index = left_child_index;
                }
            }
            None => {}
        }

        let right_child_index = self.get_right_child(index);

        match right_child_index {
            Some(right_child_index) => {
                if (right_child_index <= self.size)
                    && (self.heap[new_index].0 > self.heap[right_child_index].0)
                {
                    new_index = right_child_index;
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
            self.heap.push(value);
            self.sift_up(self.size - 1);
        }
    }

    pub fn extract_min(&mut self) -> Option<(i8, T)> {
        if self.heap.len() > 1 {
            let result = self.heap[0];
            let last_value = self.heap.pop().unwrap();
            std::mem::replace(&mut self.heap[0], last_value);
            self.size -= 1;
            self.sift_down(0);
            return Some(result);
        }
        return None;
    }

    pub fn remove(&mut self, index: usize) -> Option<(i8, T)> {
        if index < self.size {
            let result = self.heap[index];
            std::mem::replace(&mut self.heap[index], (f64::NEG_INFINITY as i8, result.1));
            self.sift_up(index);
            return self.extract_min();
        }
        return None;
    }

    pub fn change_priority(&mut self, index: usize, new_value: (i8, T)) {
        let old_value = self.heap[index];
        std::mem::replace(&mut self.heap[index], new_value);
        if new_value.0 > old_value.0 {
            self.sift_down(index);
        } else {
            self.sift_up(index);
        }
    }
}
