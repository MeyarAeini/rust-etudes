use std::mem;

pub struct List {
    head: Link
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node{
    value: i32,
    next: Link,
}


impl List {
    pub fn new() -> Self{
        List{ head: Link::Empty}
    }

    pub fn push(&mut self, val:i32) {
        let node = Node { 
            value: val, 
            next: mem::replace(&mut self.head, Link::Empty),
        };
        self.head = Link::More(Box::new(node));
        // let new_head = Link::More(Box::new(Node(val,self.0)));
        // self.0 = new_head;
    }

    pub fn top(&mut self) -> Option<i32> {
        match &self.head {
            Link::More(node) => Some(node.value),
            Link::Empty => None,
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        let top = self.top();

        self.head = self.head.next;

        top
    }
}