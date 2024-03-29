use std::mem;
use crate::Link::Empty;
use crate::Link::More;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn pop(&mut self) -> Option<i32> {
        let result;
        match &mut self.head {
            Empty => {
                result = None;
            }
            More(node) => {
                result = Some(node.elem);
                self.head = mem::replace(&mut node.next, Empty);
            }
        };
        result
    }

    pub fn pop2(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}


fn main() {}