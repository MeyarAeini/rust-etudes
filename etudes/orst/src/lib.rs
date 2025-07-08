pub trait Sorter<T>
where
    T: Ord,
{
    fn sort(slice: &mut [T]);
}

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter<T>,
{
    S::sort(slice);
}

mod bubblesort;
mod insertionsort;
mod quicksort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use quicksort::QuickSort;
pub use selectionsort::SelectionSort;

#[cfg(test)]
mod test {
    use super::*;

    struct StdSort;
    impl<T> Sorter<T> for StdSort
    where
        T: Ord,
    {
        fn sort(slice: &mut [T]) {
            slice.sort();
        }
    }

    #[test]
    fn std_works() {
        let mut things = [4, 2, 3, 1];

        sort::<_, StdSort>(&mut things);

        assert_eq!(&things, &[1, 2, 3, 4]);
    }

    #[test]
    fn bubble_works() {
        let mut things = [4, 2, 3, 1];

        sort::<_, bubblesort::BubbleSort>(&mut things);

        assert_eq!(&things, &[1, 2, 3, 4]);
    }
    #[test]
    fn insertion_works() {
        let mut things = [4, 2, 3, 1];

        sort::<_, insertionsort::InsertionSort>(&mut things);

        assert_eq!(&things, &[1, 2, 3, 4]);
    }
    #[test]
    fn selection_works() {
        let mut things = [4, 2, 3, 1];

        sort::<_, selectionsort::SelectionSort>(&mut things);

        assert_eq!(&things, &[1, 2, 3, 4]);
    }
    #[test]
    fn quick_works() {
        let mut things = [4, 2, 3, 1];

        sort::<_, quicksort::QuickSort>(&mut things);

        assert_eq!(&things, &[1, 2, 3, 4]);
    }
}
