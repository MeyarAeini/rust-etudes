#[derive(Copy, Clone)]
pub enum RefCellState {
    Unshared,
    Shared(isize),
    Exclusive,
}

use crate::cell::Cell;
use std::cell::UnsafeCell;

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    //The cell allow us to mutate a shared reference, for example in the borrow function the self is not mutable and we need to update the state
    state: Cell<RefCellState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefCellState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefCellState::Unshared => {
                self.state.set(RefCellState::Shared(1));
                Some(Ref { refcell: &self })
            }
            RefCellState::Shared(n) => {
                self.state.set(RefCellState::Shared(n + 1));
                Some(Ref { refcell: &self })
            }
            RefCellState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<T>> {
        if let RefCellState::Unshared = self.state.get() {
            self.state.set(RefCellState::Exclusive);
            Some(RefMut { refcell: &self })
        } else {
            None
        }
    }
}

use std::ops::{Deref, DerefMut};

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefCellState::Unshared | RefCellState::Exclusive => unreachable!(),
            RefCellState::Shared(1) => {
                self.refcell.state.set(RefCellState::Unshared);
            }
            RefCellState::Shared(n) => {
                self.refcell.state.set(RefCellState::Shared(n - 1));
            }
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        //SAFETY
        //A Ref instance only created when no exclusive references has been given out.
        // and the refcell's status set to Shared so no exclusive references are given out.
        // so dereferencing is safe
        unsafe { &*self.refcell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefCellState::Unshared | RefCellState::Shared(_) => unreachable!(),
            RefCellState::Exclusive => {
                self.refcell.state.set(RefCellState::Unshared);
            }
        }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        //SAFETY
        //see DerefMut
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        //SAFETY
        //A RefMut instance only created when no other exclusive references has been given out.
        // and the refcell's status set to Exclusive so only one exclusive reference is given out.
        // so dereferencing is safe
        unsafe { &mut *self.refcell.value.get() }
    }
}
