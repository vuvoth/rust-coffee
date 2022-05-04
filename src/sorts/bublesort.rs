pub fn bubblesort(slice: &mut [i64]) {
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
