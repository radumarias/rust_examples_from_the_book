#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn iter(&self) -> ListIter {
        ListIter { current: Some(self) }
    }
}

use std::os::unix::fs::chroot;
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


    for i in list.iter() {
        println!("Value: {}", i);
    }

    let mut l2_iter = ListIter::new(&list);
    while let Some(i) = l2_iter.next() {
        println!("Value: {}", i);
    }
}

struct ListIter<'a> {
    current: Option<&'a List>,
}

impl<'a> ListIter<'a> {
    fn new(root: &'a List) -> ListIter<'a> {
        ListIter { current: Some(root) }
    }
}

impl<'a> Iterator for ListIter<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.take();
        match current {
            Some(Cons(value, next)) => {
                self.current = Some(next);
                Some(*value)
            }
            _ => None,
        }
    }
}

fn navigate_cons_rec(root: List) {
    match root {
        Cons(value, next) => {
            println!("Value: {}", value);
            navigate_cons_rec(*next);
        }
        Nil => {
            println!("End of list");
        }
    }
}

fn navigate_cons_iter(root: List) {
    let mut current = root;
    loop {
        match current {
            Cons(value, next) => {
                println!("Value: {}", value);
                current = *next;
            }
            Nil => {
                println!("End of list");
                break;
            }
        }
    }
}
