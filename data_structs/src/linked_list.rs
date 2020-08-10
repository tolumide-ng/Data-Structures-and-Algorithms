use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::mem;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct LinkedList<T> {
    head: Link<T>,
    length: u8,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug, Clone)]
pub struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T: Debug + Copy + PartialEq> LinkedList<T> {
    pub fn new(elem: T) -> Self {
        let node = Node { elem, next: None };

        LinkedList {
            head: Some(Rc::new(RefCell::new(node))),
            length: 1,
        }
    }

    pub fn push(&mut self, elem: T) {
        let node = Some(Rc::new(RefCell::new(Node {
            elem,
            next: self.head.clone(),
        })));
        self.head = node;
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        } else {
            let popped = self.head.clone().unwrap().borrow().elem;

            self.head = mem::replace(&mut self.head.take().unwrap().borrow_mut().next, None);
            self.length -= 1;

            Some(popped)
        }
    }

    pub fn print_list(&self) {
        println!("{:?}", self)
    }

    pub fn find_item(&self, elem: T) -> bool {
        let mut current_value = mem::replace(&mut self.head.clone(), None);
        // Rc::new(RefCell::new(Node { elem, next: None })),
        let total_length = self.length;
        let mut match_detected: bool = false;
        let mut current_index = 0;

        while !match_detected {
            current_index += 1;
            if current_value.clone().unwrap().borrow().elem == elem {
                match_detected = true;
            }
            current_value = mem::replace(&mut current_value.unwrap().borrow_mut().next, None);
            if current_index == total_length {
                break;
            }
        }
        if match_detected {
            true
        } else {
            false
        }
    }

    // pub fn insert_at(&mut self, elem: T, position: u8) {
    //     let current_state = self.head.clone();

    //     if (position - 1) > self.length {
    //         return;
    //     } else {

    //     }
    // }
}

pub fn execute() {
    let mut linked_list = LinkedList::new(10);
    linked_list.push(12);
    linked_list.push(23);
    linked_list.print_list();
    println!("the list {:?}", linked_list);
    println!("{:?}", linked_list.find_item(20));
    println!("first pop {:?}", linked_list.pop());
    println!("second pop {:?}", linked_list.pop());
    println!("third pop {:?}", linked_list.pop());
    println!("fourth pop {:?}", linked_list.pop());
    linked_list.print_list();
}
