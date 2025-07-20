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
        let mut stack = VecDeque::new();

        let mut root = match it.next() {
            Some(v) => Self {
                value: Some(v),
                left: None,
                right: None,
            },
            None => return None,
        };

        stack.push_back(&mut root);

        while let Some(current) = stack.pop_front() {
            if let Some(v) = it.next() {
                current.left = Some(Box::new(Self {
                    value: Some(v),
                    left: None,
                    right: None,
                }));
                stack.push_back(current.left.as_mut().unwrap());
            }

            if let Some(v) = it.next() {
                current.right = Some(Box::new(Self {
                    value: Some(v),
                    left: None,
                    right: None,
                }));
                stack.push_back(current.right.as_mut().unwrap());
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
}
