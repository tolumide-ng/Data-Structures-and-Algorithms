#[derive(Debug, Clone)]
pub struct BinaryHeap<T> {
    heap: Vec<(u8, T)>,
    max_size: usize,
    size: usize,
}

impl<T> BinaryHeap<T>
where
    T: PartialEq + Copy + Clone,
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
        if index * 2 < self.size {
            return Some(index * 2);
        } else {
            return None;
        }
    }

    pub fn get_right_child(&self, index: usize) -> Option<usize> {
        if (index * 2) + 1 < self.size {
            return Some(index * 2);
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
        let mut max_index = index;

        // if self.get_left_child(index).unwrap() < self. {}
        let left_child_index = self.get_left_child(index).unwrap();
        if left_child_index <= self.size && self.heap[index].0 > self.heap[left_child_index].0 {
            max_index = left_child_index;
        }

        let right_child_index = self.get_right_child(index).unwrap();
        if right_child_index <= self.size && self.heap[index].0 > self.heap[right_child_index].0 {
            max_index = right_child_index;
        }

        if max_index != index {
            let child = self.heap[max_index];
            let curr_val = self.heap[index];

            std::mem::replace(&mut self.heap[max_index], curr_val);
            std::mem::replace(&mut self.heap[index], curr_val);
            self.sift_down(index);
        }
    }
}
