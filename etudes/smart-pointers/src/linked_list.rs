type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        match self.head {
            Some(_) => false,
            None => true,
        }
    }

    pub fn push(&mut self, value: T) {
        let new_head = Node {
            value,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_head));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;

            node.value
        })
    }

    pub fn top(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }
}

pub struct LinkedListIterator<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn to_iter<'a>(&'a self) -> LinkedListIterator<'a, T> {
        LinkedListIterator {
            current: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref();

            &node.value
        })
    }
}

pub struct LinkedListIterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for LinkedListIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref_mut();

            &mut node.value
        })
    }
}

impl<T> LinkedList<T> {
    pub fn to_iter_mut(&mut self) -> LinkedListIterMut<'_, T> {
        LinkedListIterMut {
            current: self.head.as_deref_mut(),
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();

        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_list_is_working() {
        let mut list = LinkedList::new();
        list.push(5);
        list.push(6);
        assert_eq!(list.top(), Some(&6));

        if let Some(t) = list.top_mut() {
            *t = 15;
        }
        assert_eq!(list.top(), Some(&15));

        assert_eq!(list.pop(), Some(15));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn link_list_iter_is_working() {
        let mut list = LinkedList::new();
        list.push(5);
        list.push(6);
        list.push(7);

        let mut it = list.to_iter();
        assert_eq!(it.next(), Some(&7));
        assert_eq!(it.next(), Some(&6));
        assert_eq!(it.next(), Some(&5));
        assert_eq!(it.next(), None);
    }
}
