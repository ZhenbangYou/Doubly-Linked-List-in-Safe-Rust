use crate::list::*;

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
                println!("insert {r}");
                ls.insert_front(&r);
                println!("{:?}", ls);
                match map.get_mut(&r) {
                    Some(v) => *v += 1,
                    None => {
                        let _ = map.insert(r, 1);
                    }
                }
            } else {
                let r = rng.gen_range(0..100);
                println!("delete {r}");
                match map.get_mut(&r) {
                    Some(v) => {
                        println!("{:?}", ls);
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

    #[test]
    fn test_delete_first() {
        let mut l = List::new();

        l.insert_front(&1);
        l.insert_front(&2);
        l.insert_front(&3);

        l.delete(&3);

        assert_eq!(l.to_vec(), vec![2, 1]);
    }

    #[test]
    fn test_multithreads() {
        use std::collections::HashMap;
        use std::sync::{Arc, Mutex};
        use std::thread;

        let ls = Arc::new(Mutex::new(List::new()));
        let map = Arc::new(Mutex::new(HashMap::new()));

        thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    let mut rng = rand::thread_rng();
                    for _ in 0..100 {
                        let mut ls = ls.lock().unwrap();
                        let mut map = map.lock().unwrap();
                        let is_insert = rng.gen_bool(0.5);

                        if is_insert {
                            let r = rng.gen_range(0..100);
                            println!("insert {r}");
                            ls.insert_front(&r);
                            println!("{:?}", ls);
                            match map.get_mut(&r) {
                                Some(v) => *v += 1,
                                None => {
                                    let _ = map.insert(r, 1);
                                }
                            }
                        } else {
                            let r = rng.gen_range(0..100);
                            println!("delete {r}");
                            match map.get_mut(&r) {
                                Some(v) => {
                                    println!("{:?}", ls);
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
                });
            }
        });
    }
}
