use std::ops::{Add, Deref};
use std::rc::Rc;
use rand::distributions::uniform::SampleBorrow;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

struct Wrapper(Vec<String>);

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Vec<String> {
        &self.0
    }
}

struct List<T> {
    data: T,
    next: Option<Box<List<T>>>,
    v: i32,
}
unsafe impl<T> Send for List<T>
    where
        T: Send, // from the field `data`
        Option<Box<List<T>>>: Send, // from the field `next`,
        i32: Send, // from the field `v`
{ }


fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    for s in w.iter() {
        println!("{}", s);
    }

    let mut data = Rc::new(5);

    *Rc::make_mut(&mut data) += 1;
}