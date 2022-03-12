/// Add a permission flag (`n`) to `perm`.
#[inline]
pub fn add(perm: usize, n: usize) -> usize {
    perm | 1 << n
}

// 
/// Remove a permission flag (`n`) to `perm`.
#[inline]
pub fn rm(perm: usize, n: usize) -> usize {
    perm & !(1 << n)
}

/// Check if `perm` has the permission flag `n`.
#[inline]
pub fn check(perm: usize, n: usize) -> bool {
    (perm >> n & 1) == 1
}

#[cfg(test)]
mod tests {
    const READ: usize = 0;
    const WRITE: usize = 1;
    const EXEC: usize = 2;

    #[test]
    fn add_test() {
        assert_eq!(super::add(0, READ), 1);
        assert_eq!(super::add(0, WRITE), 2);
        assert_eq!(super::add(0, EXEC), 4);
        
        assert_eq!(super::add(1 << READ, WRITE), 3);
        assert_eq!(super::add(1 << READ, EXEC), 5);
        assert_eq!(super::add(1 << WRITE, EXEC), 6);

        assert_eq!(super::add(3, EXEC), 7);
    }

    #[test]
    fn rm_test() {
        assert_eq!(super::rm(7, EXEC), 3);

        assert_eq!(super::rm(6, EXEC), 1 << WRITE);
        assert_eq!(super::rm(5, EXEC), 1 << READ);
        assert_eq!(super::rm(3, WRITE), 1 << READ);

        assert_eq!(super::rm(4, EXEC), 0);
        assert_eq!(super::rm(2, WRITE), 0);
        assert_eq!(super::rm(1, READ), 0);
    }

    #[test]
    fn check_test() {
        assert!(super::check(7, EXEC));
        assert!(super::check(7, WRITE));
        assert!(super::check(7, READ));

        assert!(super::check(6, EXEC));
        assert!(super::check(6, WRITE));
        assert!(!super::check(6, READ));

        assert!(super::check(5, EXEC));
        assert!(!super::check(5, WRITE));
        assert!(super::check(5, READ));

        assert!(super::check(4, EXEC));
        assert!(!super::check(4, WRITE));
        assert!(!super::check(4, READ));

        assert!(!super::check(2, EXEC));
        assert!(super::check(2, WRITE));
        assert!(!super::check(2, READ));

        assert!(!super::check(1, EXEC));
        assert!(!super::check(1, WRITE));
        assert!(super::check(1, READ));

        assert!(!super::check(0, EXEC));
        assert!(!super::check(0, WRITE));
        assert!(!super::check(0, READ));
    }
}