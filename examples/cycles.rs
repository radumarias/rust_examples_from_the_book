use std::cell::RefCell;
use std::rc::Rc;
use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    match a.as_ref() {
        Cons(_, next) => {
            *next.borrow_mut() = Rc::clone(&b);
        }
        Nil => {}
    }
    // println!("a after changing a = {:?}", a);

    let r = RefCell::new(Rc::new(5));
    println!("{:?}", r.borrow());
    *r.borrow_mut() = Rc::new(10);
    println!("{:?}", r.borrow());
}