pub trait IteratorExt: Iterator {
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
        Self: Sized;
}

//: ?Sized
impl<T> IteratorExt for T
where
    T: Iterator,
{
    fn our_flatten(self) -> Flatten<Self>
    where
        Self::Item: IntoIterator,
        Self: Sized,
    {
        flatten(self)
    }
}

pub fn flatten<I>(input: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(input.into_iter())
}

pub struct Flatten<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    outer: T,
    front: Option<<T::Item as IntoIterator>::IntoIter>,
    back: Option<<T::Item as IntoIterator>::IntoIter>,
}

impl<T> Flatten<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    pub fn new(input: T) -> Self {
        Self {
            outer: input,
            front: None,
            back: None,
        }
    }
}

impl<T> Iterator for Flatten<T>
where
    T: Iterator,
    T::Item: IntoIterator,
{
    type Item = <T::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.front {
                if let Some(i) = inner.next() {
                    return Some(i);
                }

                self.front = None;
            }

            if let Some(x) = self.outer.next() {
                self.front = Some(x.into_iter());
            } else {
                return self.back.as_mut()?.next();
            }
        }
    }
}

impl<T> DoubleEndedIterator for Flatten<T>
where
    T: DoubleEndedIterator,
    T::Item: IntoIterator,
    <T::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner) = self.back {
                if let Some(i) = inner.next_back() {
                    return Some(i);
                }

                self.back = None;
            }

            if let Some(x) = self.outer.next_back() {
                self.back = Some(x.into_iter());
            } else {
                return self.front.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn one() {
        assert_eq!(flatten(std::iter::once(vec![0])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec![0, 1])).count(), 2);
    }

    #[test]
    fn three_widen() {
        assert_eq!(flatten(vec![vec![0, 1], vec![2]]).count(), 3);
    }

    #[test]
    fn reverse_widen_three() {
        assert_eq!(
            flatten(vec![vec![0, 1], vec![2]]).rev().collect::<Vec<_>>(),
            vec![2, 1, 0]
        );
    }

    #[test]
    fn reverse_two() {
        assert_eq!(
            flatten(std::iter::once(vec![0, 1]))
                .rev()
                .collect::<Vec<_>>(),
            vec![1, 0]
        );
    }

    #[test]
    fn both_end() {
        let mut both = flatten(vec![vec![1, 2, 3], vec![101, 102, 103]]);
        assert_eq!(both.next(), Some(1));
        assert_eq!(both.next_back(), Some(103));
        assert_eq!(both.next(), Some(2));
        assert_eq!(both.next_back(), Some(102));
        assert_eq!(both.next_back(), Some(101));
        assert_eq!(both.next_back(), Some(3));
        assert_eq!(both.next(), None);
        assert_eq!(both.next_back(), None);
    }

    #[test]
    fn inf() {
        let mut inf = flatten((0..).map(|i| 0..i));
        assert_eq!(inf.next(), Some(0));
        assert_eq!(inf.next(), Some(0));
        assert_eq!(inf.next(), Some(1));
    }

    #[test]
    fn deep() {
        assert_eq!(flatten(flatten(vec![vec![vec![1, 2]]])).count(), 2);
    }

    #[test]
    fn ext() {
        assert_eq!(vec![vec![1, 2]].into_iter().our_flatten().count(), 2);
    }
}
