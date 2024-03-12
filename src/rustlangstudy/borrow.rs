#[cfg(test)]
mod tests {
    #[test]
    fn onwership_test() {
        let mut x;

        x = 32; 

        let y = &x;

        assert_eq!(*y, 32);   
        x = 42;

        let y = x; 
        assert_eq!(y, x);
    }
}