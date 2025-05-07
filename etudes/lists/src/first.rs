use std::mem;

pub struct List (Link);

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node(i32,Link);


impl List {
    pub fn new() -> Self {
        List(Link::Empty)
    }

    pub fn push(&mut self, val:i32) {
        let head = mem::replace(&mut self.0, Link::Empty);
        let node = Node(val, head);
        
        self.0 = Link::More(Box::new(node));
    }

    pub fn top(&mut self) -> Option<i32> {
        match &self.0 {
            Link::More(node) => Some(node.0),
            Link::Empty => None,
        }
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.0, Link::Empty) {
            Link::More(node) => {
                self.0 = node.1;

                Some(node.0)
            },
            Link::Empty => {
                None
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut next = mem::replace(&mut self.0, Link::Empty);

        while let Link::More(mut node) = next {
            next = mem::replace(&mut node.1, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test(){
        let mut lst = List::new();

        assert_eq!(lst.pop(),None);

        lst.push(1);
        lst.push(2);
        lst.push(3);

        assert_eq!(lst.pop(),Some(3));

        lst.push(4);
        lst.push(5);

        assert_eq!(lst.pop(),Some(5));
        assert_eq!(lst.pop(),Some(4));
        assert_eq!(lst.pop(),Some(2));
        assert_eq!(lst.pop(),Some(1));
        assert_eq!(lst.pop(),None);

    }
}