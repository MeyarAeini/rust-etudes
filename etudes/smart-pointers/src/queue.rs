use std::ops::DerefMut;

struct Node<T> {
    element: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: std::ptr::null_mut(),
        }
    }

    pub fn push(&mut self, element: T) {
        let mut node = Box::new(Node {
            element,
            next: None,
        });

        let node_ptr: *mut _ = node.deref_mut();

        if self.tail.is_null() {
            self.tail = node_ptr;
            self.head = Some(node);
        } else {
            unsafe {
                (*self.tail).next = Some(node);
            }

            self.tail = node_ptr;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            self.head = head.next;
            if self.head.is_none() {
                self.tail = std::ptr::null_mut();
            }

            Some(head.element)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_working() {
        let mut queue = Queue::new();
        queue.push(10);
        queue.push(12);
        assert_eq!(queue.pop(), Some(10));
        assert_eq!(queue.pop(), Some(12));
        assert_eq!(queue.pop(), None);
        queue.push(1);
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), None);
    }
}
