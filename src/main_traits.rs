use std::fmt::{Debug, Display};

fn some_function<T, U>(_t: &T, _u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
{ 1 }

trait T1 {
    fn f1(&self) -> i32 {
        1
    }
}

struct S1;

impl T1 for S1 {}

struct S2;

impl T1 for S2 {}

fn f1(s: i32) -> Box<dyn T1> {
    match s {
        1 => { Box::new(S1) }
        2 => { Box::new(S2) }
        _ => { panic!() }
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let s1 = f1(1).f1();
    let p = Pair::new(1, 2);
    p.cmp_display();

    let s = 3.to_string();
}