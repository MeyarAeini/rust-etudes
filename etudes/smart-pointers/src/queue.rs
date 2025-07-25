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
        //SAFETY
        //We know this is the only exclusive mutable reference of the queue
        //
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
        //SAFETY
        //We know this is the only exclusive muttable reference of the queue
        //And we know that the push function create a Box to allocate the node so it is safe to use
        //Box::from _raw
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

    pub fn peek(&self) -> Option<&T> {
        //SAFETY
        //We know that the head is allocated using Box and as_ref is safe
        unsafe { self.head.as_ref() }.map(|head| &head.element)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        //SAFETY
        //We know that the head is allocated using Box and this is the only exclusive reference
        unsafe { self.head.as_mut() }.map(|head| &mut head.element)
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

pub struct IntoIter<T>(Queue<T>);
impl<T> Queue<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'queue, T> {
    next: Option<&'queue Node<T>>,
}

impl<T> Queue<T> {
    pub fn iter<'queue>(&'queue self) -> Iter<'queue, T> {
        //SAFETY
        //We know that the as_ref() definition is Returns None if the pointer is null,
        //or else returns a shared reference to the value wrapped in Some. If the value may be uninitialized, as_uninit_ref must be used instead.
        //We know that the push only uses Safe Box::new operation to allocate the item and it is
        //SAFE
        unsafe {
            Iter {
                next: self.head.as_ref(),
            }
        }
    }
}

impl<'queue, T> Iterator for Iter<'queue, T> {
    type Item = &'queue T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.next.take() {
            //SAFETY
            //We know that the as_ref() definition is Returns None if the pointer is null,
            //or else returns a shared reference to the value wrapped in Some. If the value may be uninitialized, as_uninit_ref must be used instead.
            //We know that the push only uses Safe Box::new operation to allocate the item and it is
            //SAFE
            unsafe {
                self.next = next.next.as_ref();
            }
            Some(&next.element)
        } else {
            None
        }
    }
}

pub struct IterMut<'queue, T> {
    next: Option<&'queue mut Node<T>>,
}

impl<T> Queue<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            //SAFETY
            //We know this is an exclusive reference to queue and the head is allocated by Box so
            //it is a safe operation to use as_mut()
            next: unsafe { self.head.as_mut() },
        }
    }
}

impl<'queue, T> Iterator for IterMut<'queue, T> {
    type Item = &'queue mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.next.take() {
            //SAFETY
            //We know this is an exclusive reference to queue and the head is allocated by Box so
            //it is a safe operation to use as_mut()

            self.next = unsafe { node.next.as_mut() };

            Some(&mut node.element)
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

    #[test]
    fn into_iter_is_working() {
        let mut queue = Queue::new();
        queue.push(10);
        queue.push(12);
        queue.push(1);
        let mut it = queue.into_iter();
        assert_eq!(it.next(), Some(10));
        assert_eq!(it.next(), Some(12));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn iter_is_working() {
        let mut queue = Queue::new();
        queue.push(10);
        queue.push(12);
        queue.push(1);
        let mut it = queue.iter();
        assert_eq!(it.next(), Some(&10));
        assert_eq!(it.next(), Some(&12));
        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn empty_iter_is_working() {
        let mut queue: Queue<()> = Queue::new();
        let mut it = queue.iter();
        assert_eq!(it.next(), None);
    }

    #[test]
    fn iter_mut_is_working() {
        let mut queue = Queue::new();
        queue.push(10);
        queue.push(12);
        queue.push(1);
        for item in queue.iter_mut() {
            *item += 1;
        }

        assert_eq!(queue.pop(), Some(11));
        assert_eq!(queue.pop(), Some(13));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), None);
    }
}
