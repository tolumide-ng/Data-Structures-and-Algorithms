#[derive(Debug)]
pub struct BinaryHeap {
    heap: Vec<u8>,
    max_size: usize,
    size: usize,
}

impl BinaryHeap {
    pub fn new(max_size: usize) -> Self {
        BinaryHeap {
            heap: Vec::new(),
            max_size,
            size: 0,
        }
    }
    pub fn get_parent(&self, val: usize) -> Option<usize> {
        if val < self.size {
            if val % 2 == 0 {
                return Some(val / 2);
            } else {
                return Some((val - 1) / 2);
            }
        }
        None
    }

    pub fn get_leftchild(&self, val: usize) -> Option<usize> {
        if val * 2 <= self.size {
            return Some(val * 2);
        }
        None
    }

    pub fn get_rightchild(&self, val: usize) -> Option<usize> {
        if (val + 1) * 2 <= self.size {
            return Some((val * 2) + 1);
        }
        None
    }

    pub fn sift_up(&mut self, index: usize) {
        let mut val = index;
        loop {
            if val > 0 && (&self.heap[*&self.get_parent(val).unwrap()] < &self.heap[val]) {
                let curr_val = self.heap[val];
                let curr_par = self.heap[*&self.get_parent(val).unwrap()];
                let par_index = self.get_parent(val).unwrap();

                std::mem::replace(&mut self.heap[val], curr_par);

                std::mem::replace(&mut self.heap[par_index], curr_val);
                val = par_index;
            } else {
                break;
            }
        }
    }

    pub fn sift_down(&mut self, val: usize) {
        let mut max_index = val;
        if (&self.get_leftchild(val).unwrap_or(*&self.size + 1) <= &self.size)
            && (self.heap[*&self.get_leftchild(val).unwrap()] > self.heap[max_index])
        {
            max_index = *&self.get_leftchild(val).unwrap();
        }

        if &self.get_rightchild(val).unwrap_or(*&self.size + 1) <= &self.size
            && self.heap[*&self.get_rightchild(val).unwrap()] > self.heap[max_index]
        {
            max_index = *&self.get_rightchild(val).unwrap();
        }

        if val != max_index {
            let child = self.heap[max_index];
            let curr_val = self.heap[val];

            std::mem::replace(&mut self.heap[max_index], curr_val);
            std::mem::replace(&mut self.heap[val], child);
            &self.sift_down(val);
        }
    }

    pub fn insert(&mut self, val: u8) {
        if self.size != self.max_size {
            self.size += 1;
            self.heap.push(val);
            self.sift_up(self.heap.len() - 1);
        }
    }

    pub fn extract_max(&mut self) -> Option<u8> {
        if self.heap.len() > 1 {
            let result = self.heap[0];
            let last_val = self.heap.pop().unwrap();
            std::mem::replace(&mut self.heap[0], last_val);
            print!("after removing max {:?}", &self.heap);
            self.size -= 1;
            self.sift_down(0);
            return Some(result);
        }
        None
    }

    pub fn remove(&mut self, val: usize) -> Option<u8> {
        if val < self.size {
            std::mem::replace(&mut self.heap[val], u8::MAX);
            self.sift_up(val);
            println!("ME >>>>. {:?}", &self.heap);
            return self.extract_max();
        }
        None
    }

    pub fn change_priority(&mut self, index: usize, value: u8) {
        let old_value = self.heap[index];
        std::mem::replace(&mut self.heap[index], value);
        if old_value > value {
            self.sift_down(index);
        } else {
            self.sift_up(index)
        }
    }
}

pub fn execute() {
    let mut bin_heap = BinaryHeap::new(20);
    bin_heap.insert(8);
    // println!("bin heap ---------->>>>>>>>>>>>>> {:#?}", &bin_heap);
    bin_heap.insert(3);
    bin_heap.insert(14);
    bin_heap.insert(30);
    bin_heap.insert(19);
    bin_heap.insert(26);
    bin_heap.insert(31);
    bin_heap.insert(45);
    bin_heap.insert(56);
    bin_heap.insert(0);
    bin_heap.insert(9);
    bin_heap.insert(7);
    // bin_heap.change_priority(4, 6);
    // println!("A MAX_VALUE {:#?}", bin_heap.remove(3));

    println!("bin heap ---------->>>>>>>>>>>>>> {:#?}", &bin_heap);
}
