// pub mod singly_linked_list {
// }
use std::cell::RefCell;
use std::rc::Rc;

pub fn execute() {}

type Link<T> = Option<Rc<LinkedList<T>>>;

pub struct LinkedList<T> {
    head: T,
}
