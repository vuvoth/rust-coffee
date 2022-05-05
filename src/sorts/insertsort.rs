pub fn insertsort<T>(slice: &mut [T]) where T: Ord {
    let length = slice.len();

    for begin_unsorted_id in 1..length {
        let mut insert_id = begin_unsorted_id;
        while  insert_id > 0 && slice[insert_id] < slice[insert_id - 1]{
            slice.swap(insert_id, insert_id - 1);
            insert_id = insert_id - 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::insertsort;

    #[test]
    fn it_works() {
        let mut arr = vec![1, 10, 5, 5, -9];
        insertsort(&mut arr);
        
        assert_eq!(arr, vec![-9, 1, 5, 5, 10]);
    }

    #[test]
    fn it_works_one_element() {
        let mut arr = vec![1];
        insertsort(&mut arr);
        
        assert_eq!(arr, vec![1]);
    }

    #[test]
    fn it_works_empty_list() {
        let mut arr: Vec<i32> = Vec::new();
        insertsort(&mut arr);
        assert_eq!(arr, []);
    }
}