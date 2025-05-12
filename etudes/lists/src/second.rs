
struct Node<T>(T,Link<T>);

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> (Link<T>);

impl<T> List<T> {
    pub fn new() -> Self {
        List(None)
    }

    pub fn push(&mut self, val:T) {
        let head = self.0.take();
        let node = Node(val, head);
        
        self.0 = Some(Box::new(node));
    }

    pub fn top(&self) -> Option<&T> {
        //Map takes self by value, which would move the Option out of the thing it's in
        // wrong: self.0.map(|node| {&node.0})
        self.0.as_ref().map(|node| &node.0)
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.0.as_mut().map(|node| &mut node.0)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.take().map(|node| {
            self.0 = node.1;

            node.0
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut next = self.0.take();

        while let Some(mut node) = next {
            next = node.1.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T>{
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

pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<T> List<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut(self.0.as_deref_mut())
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.1.as_deref_mut();

            &mut node.0
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_mut_test(){
        let mut lst = List::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);

        let mut itr = lst.iter_mut();
        let mut next = itr.next();
        assert_eq!(next, Some(&mut 3));
        next = Some(&mut 5);
        assert_eq!(itr.next(), Some(&mut 2));
        assert_eq!(itr.next(), Some(&mut 1));
        assert_eq!(itr.next(), None);
    }

    #[test]
    fn iter_test(){
        let mut lst = List::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);

        let mut itr = lst.iter();
        assert_eq!(itr.next(), Some(&3));
        assert_eq!(itr.next(), Some(&2));
        assert_eq!(itr.next(), Some(&1));
        assert_eq!(itr.next(), None);
    }

    #[test]
    fn into_iter_test(){
        let mut lst = List::new();
        lst.push(1);
        lst.push(2);
        lst.push(3);

        let mut itr = lst.into_iter();
        assert_eq!(itr.next(), Some(3));
        assert_eq!(itr.next(), Some(2));
        assert_eq!(itr.next(), Some(1));
        assert_eq!(itr.next(), None);
    }

    #[test]
    fn top_test(){
        let mut lst = List::new();

        lst.push(1);

        if let Some(x) = lst.top_mut() {
            *x=-10;
        }

        assert_eq!(lst.top(),Some(&-10));

        lst.top_mut().map(|x| {
            *x = 5;
        });

        assert_eq!(lst.top(),Some(&5));
    }
    #[test]
    fn basic_test(){
        let mut lst = List::new();

        assert_eq!(lst.pop(),None);

        lst.push(1);
        lst.push(2);
        lst.push(3);

        assert_eq!(lst.pop(),Some(3));
        assert_eq!(lst.top(),Some(&2));

        lst.push(4);
        lst.push(5);

        assert_eq!(lst.pop(),Some(5));
        assert_eq!(lst.top(),Some(&4));

        assert_eq!(lst.pop(),Some(4));
        assert_eq!(lst.top(),Some(&2));
        
        assert_eq!(lst.pop(),Some(2));
        assert_eq!(lst.pop(),Some(1));
        assert_eq!(lst.pop(),None);

    }
}