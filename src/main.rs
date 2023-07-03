use list::*;

#[cfg(test)]
mod test;

fn main() {}

mod list {
    use std::{cell::RefCell, rc::Rc};

    struct ListNode<T: Clone + Eq> {
        item: T,
        prev: Option<Rc<RefCell<ListNode<T>>>>,
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
                    Some(next) => self.find_internal(next.clone(), value),
                    None => false,
                }
            }
        }
        pub fn find(&self, value: &T) -> bool {
            match &self.head {
                Some(n) => self.find_internal(n.clone(), value),
                None => false,
            }
        }
        pub fn insert_front(&mut self, val: &T) {
            let node = Rc::new(RefCell::new(ListNode::new(val)));
            match &self.head {
                Some(old_head) => {
                    let node = node.clone();
                    node.borrow_mut().next = Some(old_head.clone());
                    (*old_head).borrow_mut().prev = Some(node.clone());
                    self.head = Some(node);
                }
                None => {
                    self.head = Some(node.clone());
                    self.tail = Some(node);
                }
            }
        }
        fn delete_internal(&mut self, node: Rc<RefCell<ListNode<T>>>, value: &T) {
            let cur = node.borrow();
            let cur_prev = cur.prev.clone();
            let cur_next = cur.next.clone();

            if &cur.item == value {
                cur_prev.map_or_else(
                    || self.head = cur.next.clone(),
                    |x| x.borrow_mut().next = cur.next.clone(),
                );

                cur_next.map_or_else(
                    || self.tail = cur.prev.clone(),
                    |x| x.borrow_mut().prev = cur.prev.clone(),
                );
            } else {
                drop(cur);
                match cur_next {
                    Some(next) => self.delete_internal(next, value),
                    None => {}
                }
            }
        }
        pub fn delete(&mut self, value: &T) {
            match &self.head {
                Some(n) => self.delete_internal(n.clone(), value),
                None => {}
            }
        }
    }

    impl<T: Clone + Eq> List<T> {
        pub fn to_vec(&self) -> Vec<T> {
            let mut res = vec![];
            let mut cur = self.head.clone();
            while let Some(c) = cur.clone() {
                let c = c.clone();
                let val = c.borrow().item.clone();
                res.push(val);
                cur = c.borrow().next.clone();
            }
            res
        }
    }

    impl<T: Clone + Eq> Drop for List<T> {
        fn drop(&mut self) {
            let mut cur = self.head.clone();
            while let Some(node) = cur.clone() {
                node.borrow_mut().prev = None;
                cur = node.borrow_mut().next.clone();
            }
        }
    }
}
