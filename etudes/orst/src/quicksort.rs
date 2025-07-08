use super::Sorter;

pub struct QuickSort;

impl<T> Sorter<T> for QuickSort
where
    T: Ord,
{
    fn sort(slice: &mut [T]) {
        quick_sort(slice);
    }
}
fn quick_sort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if &slice[0] > &slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        //here for slices with small range we can use other algorithms such as insertion sort which are fater for smaller slices
        _ => {}
    }

    let (pivot, rest) = slice
        .split_first_mut()
        .expect("there are at least one item in the slice");

    let mut left = 0;
    let mut right = rest.len() - 1;

    // <= is required since we need to examine the last element and make sur if falls in left side or right side
    while left <= right {
        if &rest[left] <= pivot {
            //left is in a right place
            left += 1;
        } else if &rest[right] > pivot {

            if right == 0 {
                break;
            }
            //right is in a right place
            right -= 1;
        } else {
            //swap right and left which both of them are not in a right place
            rest.swap(right, left);
            left += 1;

            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    //take in account the pivot position for left to have the left position on the whole slice
    left = left + 1;

    slice.swap(0, left - 1);

    let (left, right) = slice.split_at_mut(left - 1);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}
