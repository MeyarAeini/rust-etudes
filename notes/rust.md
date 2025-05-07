# [mem::replace<T>(dest: &mut T, src: T)](https://doc.rust-lang.org/std/mem/fn.replace.html)
Moves src into the referenced dest, returning the previous dest value.

### using our enum
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

### using Option<T>
```rust
pub struct List (Link);
type Link = Option<Box<Node>>;
struct Node(i32,Link);

impl List {
    ...
    pub fn push(&mut self, val:i32) {
        let head = self.0.take();
        let node = Node(val, head);
        
        self.0 = Some(Box::new(node));
    }
    pub fn pop(&mut self) -> Option<i32> {
        match self.0.take() {
            Some(node) => {
                self.0 = node.1;

                Some(node.0)
            },
            None => {
                None
            }
        }
    }
    ...
}
```

# Option<T>.map
Maps an Option<T> to Option<U> by applying a function to a contained value (if Some) or returns None (if None).

```rust
pub fn pop(&mut self) -> Option<i32> {
    self.0.take().map(|node| {
        self.0 = node.1;
        node.0
    })
}
```