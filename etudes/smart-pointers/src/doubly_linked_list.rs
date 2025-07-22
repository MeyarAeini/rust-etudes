use std::cell::{Ref, RefCell};
use std::rc::Rc;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    back: Link<T>,
    front: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            back: None,
            front: None,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let head = Node::new(value);
        match self.back.take() {
            Some(back) => {
                back.borrow_mut().prev = Some(Rc::clone(&head));
                head.borrow_mut().next = Some(back);
                self.back = Some(head);
            }
            None => {
                self.front = Some(Rc::clone(&head));
                self.back = Some(head);
            }
        }
    }

    pub fn push_front(&mut self, value: T) {
        let tail = Node::new(value);
        match self.front.take() {
            Some(front) => {
                front.borrow_mut().next = Some(Rc::clone(&tail));
                tail.borrow_mut().prev = Some(front);
                self.front = Some(tail);
            }
            None => {
                self.back = Some(Rc::clone(&tail));
                self.back = Some(tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.back.take().map(|link| {
            if let Some(new_back) = link.borrow_mut().next.take() {
                new_back.borrow_mut().prev.take(); //make the prev of the new head None
                self.back = Some(new_back);
            } else {
                //this is the last node
                self.front.take();
            }

            Rc::try_unwrap(link) //return the inner value of Rc if it only has one strong reference
                .ok() //convert Result<T,E> to Option<T>
                .unwrap() //Convert Option<T> to T, now T:Refcell<Node<U>>
                .into_inner() //Consume RefCell<T> and return the inner value
                .value
        })
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.front.take().map(|link| {
            if let Some(new_front) = link.borrow_mut().prev.take() {
                new_front.borrow_mut().next.take(); //make the next of the new tail None
                self.front = Some(new_front);
            } else {
                //this is the last node
                self.back.take();
            }

            Rc::try_unwrap(link) //return the inner value of Rc if it only has one strong reference
                .ok() //convert Result<T,E> to Option<T>
                .unwrap() //Convert Option<T> to T, now T:Refcell<Node<U>>
                .into_inner() //Consume RefCell<T> and return the inner value
                .value
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.back
            .as_ref()
            .map(|it| Ref::map(it.borrow(), |it| &it.value))
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.front
            .as_ref()
            .map(|it| Ref::map(it.borrow(), |it| &it.value))
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

pub struct IntoIter<T>(LinkedList<T>);
impl<T> LinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}
impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn push_works() {
        let mut list = LinkedList::new();
        list.push_back(15);
        list.push_back(16);
        list.push_front(17);
        assert_eq!(&*list.peek_back().unwrap(), &16);
        assert_eq!(list.pop_back(), Some(16));
        assert_eq!(list.pop_front(), Some(17));
        assert_eq!(list.pop_back(), Some(15));
        assert_eq!(list.pop_back(), None);
        //        println!("{:#?}", list);
    }
    #[test]
    fn into_iter_works() {
        let mut list = LinkedList::new();
        list.push_back(15);
        list.push_back(16);
        list.push_front(17);
        let mut it = list.into_iter();
        assert_eq!(it.next_back(), Some(17));
        assert_eq!(it.next(), Some(16));
        assert_eq!(it.next_back(), Some(15));
        assert_eq!(it.next(), None);
        assert_eq!(it.next_back(), None);
    }
}
