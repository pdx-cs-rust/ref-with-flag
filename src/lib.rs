#![allow(dead_code)]

mod ref_with_flag {
    use std::mem::align_of;

    /// A `&T` and a `bool`, wrapped up in a single word.
    /// The type `T` must require at least two-byte alignment.
    ///
    /// If you're the kind of programmer who's never met a pointer whose
    /// 2⁰-bit you didn't want to steal, well, now you can do it safely!
    /// ("But it's not nearly as exciting this way...")
    pub struct RefWithFlag<T> {
        ptr_and_bit: *const T,
    }

    impl<'a, T: 'a> RefWithFlag<T> {
        pub fn new(ptr: &'a T, bit: bool) -> RefWithFlag<T> {
            assert!(align_of::<T>() % 2 == 0);
            RefWithFlag {
                ptr_and_bit: (ptr as *const T as usize | bit as usize) as *const T,
            }
        }

        pub fn as_ref(&self) -> &'a T {
            let ptr = (self.ptr_and_bit as usize & !1) as *const T;
            // Safety: ptr is guaranteed to point to a
            // correctly aligned live value by the type and
            // borrow checking.
            unsafe {
                &*ptr
            }
        }

        pub fn as_bool(&self) -> bool {
            self.ptr_and_bit as usize & 1 != 0
        }
    }
}

mod ref_with_flag_tests {
    #[test]
    fn use_ref_with_flag() {
        use ref_with_flag::RefWithFlag;

        let vec = vec![10, 20, 30];
        let pab = RefWithFlag::new(&vec, true);
        assert_eq!(pab.as_ref()[1], 20);
        assert_eq!(pab.as_bool(), true);
    }
}
