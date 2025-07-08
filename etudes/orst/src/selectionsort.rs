use super::Sorter;

pub struct SelectionSort;

impl<T> Sorter<T> for SelectionSort
where
    T: Ord,
{
    fn sort(slice: &mut [T]) {
        for i in 0..slice.len() {
            let min_idx = i + min_index(&slice[i..]);
            //let min_idx_iter = i + min_index_iter(&slice[i..]).expect("none empty slice always");

            //assert_eq!(min_idx, min_idx_iter);

            if i != min_idx {
                slice.swap(i, min_idx);
            }
        }
    }
}

fn min_index<T>(slice: &[T]) -> usize
where
    T: Ord,
{
    let mut min_idx = 0;

    for j in 1..slice.len() {
        if slice[j] < slice[j - 1] {
            min_idx = j;
        }
    }

    min_idx
}

fn min_index_iter<T>(slice: &[T]) -> Option<usize>
where
    T: Ord,
{
    slice
        .iter()
        .enumerate()
        .min_by_key(|&(_, v)| v)
        .map(|(i, _)| i)
}
