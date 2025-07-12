#![feature(dropck_eyepatch)]

use std::marker::PhantomData;

pub struct Boks<T> {
    t: *mut T,
    _t:PhantomData<T>,
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Self {
            t: Box::into_raw(Box::new(t)),
            _t:PhantomData
        }
    }
}

unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        unsafe {
            let _ = Box::from_raw(self.t);
        }
    }
}

use std::ops::Deref;

impl<T> Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        //SAFETY:
        //the t is created from a valid T by using a Box and it has not been freed since the self
        //is alived
        unsafe { &*self.t }
    }
}

use std::ops::DerefMut;

impl<T> DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        //SAFETY:
        //the t is created from a valid T by using a Box and it has not been freed since the self
        //is alived. this would be the only mut reference to t since the &mut self is the only mutable
        //reference
        unsafe { &mut *self.t }
    }
}

use std::fmt::Debug;

struct Oisan<T:Debug> (T);

impl<T:Debug> Drop for Oisan<T> {
    fn drop(&mut self) {
        println!("{:?}",self.0);
    }
}

fn main() {
    let x = 42;
    let b = Boks::ny(&x);
    println!("{}", *b);

    let mut y = 32;
    let b2 = Box::new(&mut y);
    println!("{}",y);

    let mut z = 122;
    let o = Boks::ny(Oisan(&mut z));
    //println!("{:?}",z); //this line should give out a compile error : immutable borrow occurs here,
    //- mutable borrow might be used here, when `o` is dropped and runs the `Drop` code for type `Boks`
}
