use list::*;

#[cfg(test)]
mod test;

fn main() {}

mod list {
    use std::{
        cell::RefCell,
        fmt::Debug,
        rc::{Rc, Weak},
    };

    pub struct ListNode<T: Clone + Eq> {
        item: T,
        prev: Option<Weak<RefCell<ListNode<T>>>>,
        next: Option<Rc<RefCell<ListNode<T>>>>,
    }

    impl<T: Clone + Eq> ListNode<T> {
        fn new(item: &T) -> ListNode<T> {
            ListNode {
                item: item.clone(),
                prev: None,
                next: None,
            }
        }
    }

    pub struct List<T: Clone + Eq> {
        head: Option<Rc<RefCell<ListNode<T>>>>,
        tail: Option<Rc<RefCell<ListNode<T>>>>,
    }

    impl<T: Clone + Eq + std::fmt::Display> List<T> {
        pub fn new() -> List<T> {
            List {
                head: None,
                tail: None,
            }
        }
        fn find_internal(&self, node: Rc<RefCell<ListNode<T>>>, value: &T) -> bool {
            let node = node.clone();
            let cur = (*node).borrow();
            if &cur.item == value {
                true
            } else {
                match &cur.next {
                    Some(ref next) => self.find_internal(next.clone(), value),
                    None => false,
                }
            }
        }
        pub fn find(&self, value: &T) -> bool {
            match &self.head {
                Some(ref next) => self.find_internal(next.clone(), value),
                None => false,
            }
        }
        pub fn insert_front(&mut self, val: &T) {
            let node = Rc::new(RefCell::new(ListNode::new(val)));
            match &self.head {
                Some(ref old_head) => {
                    let node = node.clone();
                    node.borrow_mut().next = Some(old_head.clone());
                    (*old_head).borrow_mut().prev = Some(Rc::downgrade(&node.clone()));
                    self.head = Some(node);
                }
                None => {
                    self.head = Some(node.clone());
                    self.tail = Some(node);
                }
            }
        }
        fn delete_internal(&mut self, node: &Rc<RefCell<ListNode<T>>>, value: &T) {
            if &node.borrow().item == value {
                self.delete_node(node.clone());
            } else {
                let next = node.borrow().next.clone();
                if let Some(ref next) = next {
                    self.delete_internal(next, value)
                }
            }
        }
        pub fn delete(&mut self, value: &T) {
            if let Some(ref n) = &self.head {
                self.delete_internal(&(n.clone()), value)
            }
        }
        /// *node* must be in the list (*self*)!
        pub fn delete_node(&mut self, node: Rc<RefCell<ListNode<T>>>) {
            let cur = node.borrow();
            match (cur.prev.clone(), cur.next.clone()) {
                (None, None) => {
                    self.head = None;
                    self.tail = None;
                }
                (None, Some(ref next)) => {
                    self.head = Some(next.clone());
                    next.borrow_mut().prev = None;
                }
                (Some(ref prev), None) => {
                    let prev = prev.upgrade().unwrap();
                    self.tail = Some(prev.clone());
                    prev.borrow_mut().next = None;
                }
                (Some(ref prev), Some(ref next)) => {
                    let prev = prev.upgrade().unwrap();
                    prev.borrow_mut().next = Some(next.clone());
                    next.borrow_mut().prev = Some(Rc::downgrade(&prev));
                }
            }
        }
    }

    impl<T: Clone + Eq + std::fmt::Debug> Debug for List<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("List")
                .field("items", &self.to_vec())
                .finish()
        }
    }

    impl<T: Clone + Eq> List<T> {
        pub fn to_vec(&self) -> Vec<T> {
            let mut res = vec![];
            let mut cur = self.head.clone();
            while let Some(ref c) = cur.clone() {
                let c = c.clone();
                let val = c.borrow().item.clone();
                res.push(val);
                cur = c.borrow().next.clone();
            }
            res
        }
    }
}
