use std::ops::DerefMut;
use std::ptr;
struct Node<T> {
    element: T,
    next: Link<T>,
}

type Link<T> = *mut Node<T>;

pub struct Queue<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, element: T) {
        unsafe {
            let node = Box::into_raw(Box::new(Node {
                element,
                next: ptr::null_mut(),
            }));

            if self.tail.is_null() {
                self.head = node;
            } else {
                (*self.tail).next = node;
            }
            self.tail = node;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let current = Box::from_raw(self.head);

                self.head = current.next;
                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }

                Some(current.element)
            }
        }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
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
