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

macro_rules! sort_test_template {
    ($suit_name:ident, $value: expr, $sort_program:expr) => {
        #[test] 
        fn $suit_name() {
            let mut input = Vec::from($value);
            let mut output = Vec::from($value);
            $sort_program(&mut input);
            output.sort();
            assert_eq!(input, output);
        }
    };

    ($suit_name:ident, $sort_program:expr) => {
        #[test] 
        fn $suit_name() {
            let mut input: Vec<i32> = Vec::new();
            $sort_program(&mut input);
            assert_eq!(input, vec![]);
        }
    }
}

#[cfg(test)]
mod test {

    use super::insertsort;

    sort_test_template! {normal_case, [5, 4, 10, 4, -10], insertsort}

    sort_test_template! {one_element, [4], insertsort} 

    sort_test_template! {empty, insertsort}

}