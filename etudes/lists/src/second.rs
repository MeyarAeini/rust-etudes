pub struct List<T> (Link<T>);

type Link<T> = Option<Box<Node<T>>>;

struct Node<T>(T,Link<T>);

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

#[cfg(test)]
mod test {
    use super::*;

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