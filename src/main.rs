use list::*;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test1() {
        let mut ls = List::new();
        ls.insert_front(&1);
        ls.insert_front(&3);
        ls.insert_front(&-121);
        assert_eq!(ls.find(&3), true);
        assert_eq!(ls.find(&5), false);
        ls.delete(&1);
        assert_eq!(ls.find(&1), false);
    }

    #[test]
    fn test2() {
        use std::collections::HashMap;

        let mut ls = List::new();
        let mut rng = rand::thread_rng();
        let mut map = HashMap::new();
        for _ in 0..100 {
            let is_insert = rng.gen_bool(0.5);

            if is_insert {
                let r = rng.gen_range(0..100);
                println!("{r}");
                ls.insert_front(&r);
                match map.get_mut(&r) {
                    Some(v) => *v += 1,
                    None => {
                        let _ = map.insert(r, 1);
                    }
                }
            } else {
                let r = rng.gen_range(0..100);
                println!("{r}");
                match map.get_mut(&r) {
                    Some(v) => {
                        println!("cnt: {}", *v);
                        *v -= 1;
                        if *v == 0 {
                            map.remove(&r);
                        }
                        assert_eq!(ls.find(&r), true);
                        ls.delete(&r);
                    }
                    None => {
                        assert_eq!(ls.find(&r), false);
                    }
                }
            }
        }
    }
}

mod list {
    use std::{cell::RefCell, rc::Rc};

    pub struct ListNode<T: Clone + Eq> {
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

    impl<T: Clone + Eq> List<T> {
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
                    self.head = Some(node);
                }
                None => {
                    self.head = Some(node.clone());
                    self.tail = Some(node);
                }
            }
        }
        fn delete_internal(&mut self, node: Rc<RefCell<ListNode<T>>>, value: &T) {
            let node = node.clone();
            let cur = (*node).borrow();
            if &cur.item == value {
                match &cur.prev {
                    Some(prev) => match &cur.next {
                        Some(next) => {
                            prev.borrow_mut().next = Some(next.clone());
                            next.borrow_mut().prev = Some(prev.clone());
                        }
                        None => {
                            prev.borrow_mut().next = None;
                            self.tail = Some(prev.clone());
                        }
                    },
                    None => match &cur.next {
                        Some(next) => {
                            next.borrow_mut().prev = None;
                            self.head = Some(next.clone());
                        }
                        None => {
                            self.head = None;
                            self.tail = None;
                        }
                    },
                }
            } else {
                match &cur.next {
                    Some(next) => self.delete_internal(next.clone(), value),
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
}
