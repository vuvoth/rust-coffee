pub fn bubblesort<T>(slice: &mut [T])
where
    T: Ord,
{
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
        };
    }
    use super::bubblesort;

    sort_test_template! {normal_case, [5, 4, 10, 4, -10], bubblesort}

    sort_test_template! {one_element, [4], bubblesort}

    sort_test_template! {empty, bubblesort}
}
