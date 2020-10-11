pub fn bubble_sort<T: PartialOrd>(v: &mut [T]) {
    //1 pass if already sorted
    for start in 0..v.len() {
        let mut sorted = true; //add later

        // BROKEN: should start from 0.
        //
        for i in start..(v.len() - 1) {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                sorted = false;
            }
        }
        if sorted {
            return;
        }
    }
}
