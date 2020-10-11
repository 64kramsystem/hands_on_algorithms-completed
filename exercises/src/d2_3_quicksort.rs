use std::fmt::Debug;

// Iterating the collection using a chosen pivot, and move the lower elements to its left, and the
// higher to its right. Since we can't move blocks of elements, when we find an element that it's
// lower (`l`) than the pivot (`p`), we take the element to the right of the pivot (`h`), move it
// to the `l` position, then move `p` to the right, and `l` to the left of `p`, so that at any time,
// higher than p values are always to its right.

pub fn pivot<T: PartialOrd>(collection: &mut [T]) -> usize {
    let mut p = 0;

    for i in 1..collection.len() {
        // Subtlety (for testing): values equal to the pivot will be to its right.
        //
        if collection[i] < collection[p] {
            collection.swap(p + 1, i);
            collection.swap(p, p + 1);
            p += 1;
        }
    }

    p
}

pub fn quicksort<T: PartialOrd>(collection: &mut [T]) {
    // The empty collection test can be performed here (include equality test), or for each subarray
    // after the split.
    //
    if collection.len() <= 1 {
        return;
    }

    let p = pivot(collection);

    let (collection_1, collection_2) = collection.split_at_mut(p);

    quicksort(collection_1);
    quicksort(&mut collection_2[1..]);
}

#[cfg(test)]
mod tests {
    use crate::test_sort;

    #[test]
    fn test_pivot() {
        use super::*;

        // Modified, in order to test an array with a value equal to the pivot.
        //
        let mut collection = vec![
            11, 1273, 18273, 8273, 827, 213, 9172397, 2373, 2, 4, 20983, 29831093, 287, 2837, 11,
            92900,
        ];

        let p = pivot(&mut collection);

        for (i, element) in collection.iter().enumerate() {
            // Watch out! All the elements to the left (or in the pivot position) are greater or *equal*
            // to the pivot. Alternatively, the lower numbers can be tests (without equality).
            //
            assert_eq!(
                collection[p] <= *element,
                p <= i,
                "Element {} in wrong position; array: {:?}",
                element,
                collection
            );
        }
    }

    test_sort!(test_quicksort, collection, quicksort(&mut collection));
}
