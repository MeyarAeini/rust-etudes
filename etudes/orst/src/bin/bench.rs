use orst::*;
use std::cmp::Ordering;
use std::cell::Cell;
use std::rc::Rc;
use rand::Rng;


#[derive(Clone)]
struct SortEvaluator<T>{
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}
impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("T:Ord")
    }
}
fn main() {
    let counter = Rc::new(Cell::new(0));
    let mut rng = rand::rng();
    for &n in &[0,1,10,100,1000,10000] {
        for _ in 0..10 {
            let mut values = Vec::with_capacity(n);

            for _ in 0..n{
                //rng.random::<usize>() doesnt work because generating an unrestricted usize doesn’t make sense for most applications.
                values.push(SortEvaluator{t:rng.random::<u32>(),cmps:Rc::clone(&counter)});
            }

            let took = bench::<_,BubbleSort>(&values,&counter);
            println!("{} {} {}","BubbleSort", n, took);
            let took = bench::<_,InsertionSort>(&values,&counter);
            println!("{} {} {}","InsertionSort", n, took);
            let took = bench::<_,SelectionSort>(&values,&counter);
            println!("{} {} {}","SelectionSort", n, took);
            let took = bench::<_,QuickSort>(&values,&counter);
            println!("{} {} {}","QuickSort", n, took);
        }
    }
}

fn bench<T:Ord+Clone,S:Sorter<SortEvaluator<T>>>(values:&[SortEvaluator<T>],counter:&Cell<usize>) -> usize {
    let mut values = values.to_vec();
    counter.set(0);
    S::sort(&mut values);
    counter.get()
}
