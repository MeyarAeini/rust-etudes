use crate::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull; //NonNull is like the *mut , it is a raw pointer

pub struct Shared<T> {
    value: T,
    count: Cell<usize>,
}

pub struct Rc<T> {
    state: NonNull<Shared<T>>,       //*const Shared<T>,
    _marker: PhantomData<Shared<T>>, //search for nomicon drop check
}

impl<T> Rc<T> {
    pub fn new(value: T) -> Self {
        let shared = Box::new(Shared {
            value: value,
            count: Cell::new(1),
        });

        Self {
            //SAFETY
            //The box never give us a null pointer so the NonNull:new_unchecked is fine.
            state: unsafe { NonNull::new_unchecked(Box::into_raw(shared)) },
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        //SAFETY
        //the self.state is a box which will be dealocated when the last reference to the Rc is dealocated,
        //since we still have the Rc so the box is not dealocated
        &unsafe { self.state.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let state = unsafe { self.state.as_ref() };
        let c = state.count.get();
        state.count.set(c + 1);

        Self {
            state: self.state,
            _marker: PhantomData,
        }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let state = unsafe { self.state.as_ref() };
        let c = state.count.get();

        if c == 1 {
            //drop(state);//calls to `std::mem::drop` with a reference instead of an owned value does nothing
            let _ = state;

            //SAFETY: we are the _only_ Rc left , and we are being droped.
            // therefor after use there will be no Rc's and not reference to T.
            let _ = unsafe { Box::from_raw(self.state.as_ptr()) };
        } else {
            //there are still some other references to the Rc so just decrease the references count and keep the Rc
            state.count.set(c - 1);
        }
    }
}
