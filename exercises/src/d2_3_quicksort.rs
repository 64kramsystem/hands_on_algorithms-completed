use rand::Rng;

// Iterating the collection using a chosen pivot, and move the lower elements to its left, and the
// higher to its right. Since we can't move blocks of elements, when we find an element that it's
// lower (`l`) than the pivot (`p`), we take the element to the right of the pivot (`h`), move it
// to the `l` position, then move `p` to the right, and `l` to the left of `p`, so that at any time,
// higher than p values are always to its right.

pub fn pivot<T: PartialOrd>(collection: &mut [T]) -> usize {
    // Optimization: use a random element as pivot
    //
    let r = rand::thread_rng().gen_range(0..collection.len());
    collection.swap(r, 0);

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

// Complexity require to make a parallel implementation, due to lifetimes.
// To make this trivial, use Rayon or Crossbeam.
//
pub struct RawSend<T>(*mut [T]);
unsafe impl<T> Send for RawSend<T> {}

pub fn threaded_quicksort<T: 'static + PartialOrd + Send>(collection: &mut [T]) {
    if collection.len() <= 1 {
        return;
    }

    let p = pivot(collection);

    let (collection_1, collection_2) = collection.split_at_mut(p);

    let collection_1 = collection_1 as *mut [T];
    let collection_1 = RawSend(collection_1);

    unsafe {
        let thread = std::thread::spawn(move || {
            threaded_quicksort(&mut *collection_1.0);
        });
        threaded_quicksort(&mut collection_2[1..]);

        thread.join().ok();
    }
}

// Base random generator; on each iteration, the formula is `curr = (current * multiplier + increment) % modulo;`
// Number should be prime-ish and large.
//
pub struct RandGen {
    pub current: usize,
    pub multiplier: usize,
    pub increment: usize,
    pub modulo: usize,
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

    test_sort!(
        test_threaded_quicksort,
        collection,
        threaded_quicksort(&mut collection)
    );
}
