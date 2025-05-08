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
Maps an Option<T> to Option< U > by applying a function to a contained value (if Some) or returns None (if None).

```rust
pub fn pop(&mut self) -> Option<i32> {
    self.0.take().map(|node| {
        self.0 = node.1;
        node.0
    })
}
```

# Lifetime

what does 
```rust 
fn foo<'a>(&'a A) -> &'a B 
```  
mean? In practical terms, all it means is that the input must live at least as long as the output. So if you keep the output around for a long time, this will expand the region that the input must be valid for. Once you stop using the output, the compiler will know it's ok for the input to become invalid too.

# as_deref(&self) -> Option<&<T as Deref>::Target>

Converts from **Option<T>** (or **&Option<T>**) to **Option<&T::Target>**.

Leaves the original Option in-place, creating a new one with a reference to the original one, additionally coercing the contents via Deref.

```rust
pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<T> List<T> {
    //lifetime elision
    //pub fn iter<'a>(&'a self) -> Iter<'a, T> { 
    pub fn iter(& self) -> Iter<T> {
        Iter(self.0.as_deref())
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.map(|node| {
            self.0 = node.1.as_deref();

            & node.0
        })
    }
}
```