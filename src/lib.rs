pub mod sorts;

pub use sorts::bubblesort;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::sorts::bubblesort;

        let mut arr = vec![1, 7, 3, -9, 5];
        
        bubblesort(&mut arr);

        assert_eq!(arr, vec![-9, 1, 3, 5, 7]);
    }
}