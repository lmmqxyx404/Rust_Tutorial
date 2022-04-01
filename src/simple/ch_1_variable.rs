#[cfg(test)]
mod variable {
    #[test]
    fn var() {
        let mut a = 1;
        let b = false;
        // bool can not use as to convert to u8;
        // assert_eq!(a as bool,b);
        // assert_eq!(a, b as bool);
        // assert!(b as u8);
        // assert!(a as bool);
        assert_eq!(a<0,b);
    }
}
