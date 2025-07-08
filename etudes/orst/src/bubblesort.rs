use super::Sorter;

pub struct BubbleSort;

impl<T> Sorter<T> for BubbleSort
where
    T: Ord,
{
    fn sort(slice: &mut [T]) {
        loop {
            let mut swaped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    swaped = true;
                    slice.swap(i - 1, i);
                }
            }

            if !swaped {
                break;
            }
        }
    }
}
