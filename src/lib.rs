//! Fancied up version of `ref-with-flag` example from
//! Blandy \& Orendorff's *Programming Rust*.
//!
//! This crate provides a "reference with flag" type as an
//! example of the kind of low-level systems programming
//! that can be done in Rust.
//!
//! **DANGER**: This code as it stands is quite unsafe.
//! See if you can spot where it all goes wrong.

use std::marker::PhantomData;
use std::mem::align_of;
use std::ops::{Deref, DerefMut};

/// A `&T` and a `bool`, wrapped up in a single word.
/// The type `T` must require at least two-byte alignment.
///
/// If you're the kind of programmer who's never met a pointer whose
/// 2⁰-bit you didn't want to steal, well, now you can do it safely!
/// ("But it's not nearly as exciting this way...")
pub struct RefWithFlag<'a, T: 'a> {
    ptr_and_bit: usize,
    behaves_like: PhantomData<&'a T> // occupies no space
}

impl<'a, T: 'a> RefWithFlag<'a, T> {
    pub fn new(ptr: &'a T, bit: bool) -> RefWithFlag<T> {
        assert!(align_of::<T>() % 2 == 0);
        RefWithFlag {
            ptr_and_bit: ptr as *const T as usize | bit as usize,
            behaves_like: PhantomData
        }
    }

    pub fn as_bool(&self) -> bool {
        self.ptr_and_bit & 1 != 0
    }

    pub fn set_bool(&mut self, b: bool) {
        self.ptr_and_bit &= !1;
        self.ptr_and_bit |= b as usize;
    }
}

impl<T> AsRef<T> for RefWithFlag<'_, T> {
    fn as_ref(&self) -> &T {
        let ptr = (self.ptr_and_bit & !1) as *const T;
        unsafe {
            &*ptr
        }
    }
}

impl<T> AsMut<T> for RefWithFlag<'_, T> {
    fn as_mut(&mut self) -> &mut T {
        let ptr = (self.ptr_and_bit & !1) as *mut T;
        unsafe {
            &mut *ptr
        }
    }
}

impl<T> Deref for RefWithFlag<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.as_ref()
    }
}

impl<T> DerefMut for RefWithFlag<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

#[cfg(test)]
mod ref_with_flag_tests {
    use crate::RefWithFlag;

    macro_rules! make_testcase {
        ($name:ident) => {
            let vec = vec![10, 20, 30];
            let $name = RefWithFlag::new(&vec, true);
        };
    }

    macro_rules! make_testcase_mut {
        ($name:ident) => {
            let mut vec = vec![10, 20, 30];
            let mut $name = RefWithFlag::new(&mut vec, true);
        };
    }

    #[test]
    fn use_ref_with_flag() {
        make_testcase!(pab);
        assert_eq!(pab.as_ref()[1], 20);
        assert_eq!(pab.as_bool(), true);
    }

    #[test]
    fn modify_flag() {
        make_testcase_mut!(pab);
        pab.set_bool(false);
        assert_eq!(pab.as_bool(), false);
    }

    #[test]
    fn as_ref() {
        make_testcase_mut!(pab);
        assert_eq!(pab.as_ref()[1], 20);
        pab.as_mut()[1] = 255;
        assert_eq!(pab.as_ref()[1], 255);
    }

    #[test]
    fn deref() {
        make_testcase_mut!(pab);
        assert_eq!(pab[1], 20);
        pab[1] = 255;
        assert_eq!(pab[1], 255);
    }

    // XXX This test should not compile, since we are
    // storing through an immutable slice reference.
    #[test]
    fn bad_mut() {
        let vec = vec![10, 20, 30];
        let mut pab = RefWithFlag::new(&vec, true);
        pab[0] = 0;
        assert_eq!(pab[0], 0);
        panic!("bad_mut: compiled and ran successfully");
    }
}
