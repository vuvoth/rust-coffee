pub fn bubblesort<T>(slice: &mut [T]) where T: Ord {
    let mut swaped = true;
    while swaped {
        swaped = false;
        for i in 1..slice.len() {
            if slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                swaped = true;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::bubblesort;

    #[test]
    fn it_works_with_i32_vector() {

        let mut arr = vec![1, 7, 3, -9, 5];
        
        bubblesort(&mut arr);

        assert_eq!(arr, vec![-9, 1, 3, 5, 7]);
    }

    #[test]
    fn it_works_with_chars() {
        let mut arr = vec!['a', 'e', 'c'];

        bubblesort(&mut arr);

        assert_eq!(arr, vec!['a', 'c', 'e']);
    }
}