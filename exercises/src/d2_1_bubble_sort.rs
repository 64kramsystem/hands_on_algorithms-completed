// "Bubble" the highest element for each cycle to the last position(s), by swapping adjacent pairs.
//
pub fn bubble_sort<T: PartialOrd>(collection: &mut [T]) {
    // If the collection is already sorted, only one pass is performed.
    //
    for start in 0..collection.len() {
        let mut sorted = true;

        // For each cycle, we can ignore the last `start` elements, because on each pass moves the
        // highest element [considered] to the end of the [considered] interval.
        //
        for i in 0..(collection.len() - 1 - start) {
            if collection[i] > collection[i + 1] {
                collection.swap(i, i + 1);
                sorted = false;
            }
        }

        if sorted {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;
    use crate::test_sort;

    test_sort!(test_bubble_sort, collection, bubble_sort(&mut collection));
}
