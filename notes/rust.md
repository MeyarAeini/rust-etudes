# [mem::replace<T>(dest: &mut T, src: T)](https://doc.rust-lang.org/std/mem/fn.replace.html)
Moves src into the referenced dest, returning the previous dest value.

```rust
use std::mem;

pub struct List (Link);
enum Link {
    Empty,
    More(Box<Node>),
}
struct Node(i32,Link);

impl List {
    ...
    pub fn push(&mut self, val:i32) {
        let head = mem::replace(&mut self.0, Link::Empty);
        let node = Node(val, head);
        
        self.0 = Link::More(Box::new(node));
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
    ...
}
```