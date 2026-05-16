fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use std::cell::UnsafeCell;

    #[test]
    pub fn fail_on_sb_only() {
        let mut vec = vec![42; 2];
        let ms = &mut vec[..];
        unsafe {
            let from = ms.as_ptr();
            // SB inserts an implicit write, killing the raw pointer
            let to = ms.as_mut_ptr();
            std::ptr::copy_nonoverlapping(from, to.add(1), 1);
        }
    }

    #[test]
    pub fn fail_on_tb_only() {
        unsafe {
            let mut x = 0;
            let xref = &mut x;
            let xraw = xref as *mut i32;
            let xref2 = &mut *xraw;
            let xraw2 = xref2 as *mut i32;

            xraw2.write(0);
            xraw.read();
            xraw2.write(0);
        }
    }

    #[test]
    pub fn do_not_fail() {
        let mut x1 = 42;
        let x2 = &mut x1;
        *x2 += 1;
        let x3 = &mut *x2;
        *x3 += 1;
        *x2 -= 2;
    }

    #[test]
    pub fn fail_on_both() {
        let mut x1 = 42;
        let x2 = &mut x1;
        *x2 += 1;
        let x3 = &mut *x2;
        let x3p = x3 as *mut i32;
        *x3 += 1;
        *x2 -= 2;
        unsafe {
            *x3p += 1;
        }
    }

    #[test]
    pub fn fail_due_to_strict_provenance() {
        unsafe {
            let mut x = 42;
            let ptr = (&mut x) as *mut i32 as usize as *mut i32;
            *ptr = 42;
        }
    }

    #[test]
    /// Fails when `-Zmiri-tree-borrows-implicit-writes` is enabled
    pub fn fail_with_implicit_writes() {
        let mut x = 0u8;
        let ptr = &raw mut x;
        let res = dereference(&mut x, ptr);
        assert_eq!(*res, 0);

        fn dereference<T>(x: T, y: *mut u8) -> T {
            let _ = unsafe { *y };
            x
        }
    }

    #[test]
    /// Normally fails when `-Zmiri-tree-borrows-implicit-writes` is enabled, but should be ignored with changes to Miri that create a custom ignore list.
    pub fn should_be_ignored() {
        let mut x = 0u8;
        let ptr = &raw mut x;
        let res = ignore_this(&mut x, ptr);
        assert_eq!(*res, 0);
    }

    fn ignore_this<T>(x: T, y: *mut u8) -> T {
        let _ = unsafe { *y };
        x
    }

    #[test]
    /// Passes only when `-Zmiri-tree-borrows-no-precise-interior-mut` is set
    pub fn fail_for_precise_interior_mut() {
        #[repr(C)]
        struct Foo {
            x: u32,
            y: UnsafeCell<u32>,
        }
        let f = Foo {
            x: 41,
            y: UnsafeCell::new(1),
        };
        let fr: *const Foo = &f;
        let fr = fr.cast_mut();
        unsafe {
            (*fr).x += 1;
        }
        assert_eq!(f.x, 42);
    }
}
