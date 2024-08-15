fn main() {
    println!("Hello, world!");
}

mod test {
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
            let x = 42;
            let ptr = x as *mut i32 as usize as *mut i32;
            *ptr = 42;
        }
    }
}
