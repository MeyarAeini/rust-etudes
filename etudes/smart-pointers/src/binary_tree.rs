use std::collections::VecDeque;

#[derive(Debug)]
pub struct Node<T> {
    pub value: Option<T>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: Option<T>) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut it = iter.into_iter();
        let mut queue = VecDeque::new();

        let mut root = match it.next() {
            Some(v) => Self {
                value: Some(v),
                left: None,
                right: None,
            },
            None => return None,
        };

        queue.push_back(&mut root);

        while let Some(current) = queue.pop_front() {
            if let Some(v) = it.next() {
                current.left = Some(Box::new(Self {
                    value: Some(v),
                    left: None,
                    right: None,
                }));
                queue.push_back(current.left.as_mut().unwrap());
            }

            if let Some(v) = it.next() {
                current.right = Some(Box::new(Self {
                    value: Some(v),
                    left: None,
                    right: None,
                }));
                queue.push_back(current.right.as_mut().unwrap());
            }
        }

        Some(root)
    }
}

#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        match Node::from_iter(iter) {
            Some(v) => Self {
                root: Some(Box::new(v)),
            },
            None => Self { root: None },
        }
    }
}

#[derive(Debug)]
pub struct MinHeap<T>
where
    T: std::cmp::PartialOrd,
{
    nodes: Vec<T>,
}

impl<T> MinHeap<T>
where
    T: std::cmp::PartialOrd,
{
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.nodes.push(value);
        let mut i = self.nodes.len() - 1;
        if i == 0 {
            return;
        }

        let mut pi = (i - 1) / 2;
        while let Some(parent) = self.nodes.get(pi) {
            if parent <= self.nodes.get(i).unwrap() {
                break;
            }
            self.nodes.swap(i, pi);
            i = pi;
            if i == 0 {
                break;
            }
            pi = (i - 1) / 2;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let len = self.nodes.len();
        if len == 0 {
            return None;
        }

        if len == 1 {
            return self.nodes.pop();
        }

        self.nodes.swap(0, len - 1);

        let result = self.nodes.pop();

        let mut i = 0;
        while let Some(v) = self.nodes.get(i) {
            let li = (i * 2) + 1;
            if let Some(l) = self.nodes.get(li)
                && l < v
            {
                self.nodes.swap(i, li);
                i = li;
                continue;
            }

            let ri = (i * 2) + 2;
            if let Some(r) = self.nodes.get(ri)
                && r < v
            {
                self.nodes.swap(i, ri);
                i = ri;
                continue;
            }
            break;
        }

        result
    }
    pub fn top(&self) -> Option<&T> {
        self.nodes.get(0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_work() {
        let nd = Node::new(Some(5));
        assert_eq!(nd.value, Some(5));
    }

    #[test]
    fn binary_tree_should_work() {
        let tree: BinaryTree<i32> = BinaryTree::from_iter(vec![1, 2, 3, 4, 5, 6]);
        //tree.root.value = Some(5);
        println!("{:#?}", tree);
        assert!(tree.root.unwrap().left.unwrap().value == Some(2));
    }

    #[test]
    fn min_heap_should_work() {
        let mut heap = MinHeap::new();
        heap.push(10);
        heap.push(5);
        heap.push(12);
        assert_eq!(heap.top(), Some(&5));

        println!("{:#?}", heap);

        heap.push(2);
        println!("{:#?}", heap);

        assert_eq!(heap.pop(), Some(2));
    }
}
