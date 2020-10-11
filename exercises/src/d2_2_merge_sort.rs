use super::d2_2_merge_sort_source::source_merge_sort;

// Recursively sort the two subcollections (until there is only one element), then sort them by
// iterating both at the same time, and comparing the elements.
//
// Improved version of the course one:
//
// - it doesn't mutate the source collection;
// - it's simpler, and uses the APIs better;
// - it borrows the source;
// - it has less allocations (conversions).
//
pub fn merge_sort_improved<T: PartialOrd + Copy>(collection: &[T]) -> Vec<T> {
    if collection.len() == 1 {
        return collection.to_vec();
    }

    let (collection_1, collection_2) = collection.split_at(collection.len() / 2);

    let collection_1 = merge_sort_improved(collection_1);
    let collection_2 = merge_sort_improved(collection_2);

    let mut sorted_collection: Vec<T> = Vec::with_capacity(collection.len());

    let mut collection_1_iter = collection_1.into_iter().peekable();
    let mut collection_2_iter = collection_2.into_iter().peekable();

    while let (Some(entry_1), Some(entry_2)) = (collection_1_iter.peek(), collection_2_iter.peek())
    {
        if entry_1 < entry_2 {
            sorted_collection.push(collection_1_iter.next().unwrap());
        } else {
            sorted_collection.push(collection_2_iter.next().unwrap());
        }
    }

    // One of the collections still has elements at this point.
    //
    sorted_collection.extend(collection_1_iter);
    sorted_collection.extend(collection_2_iter);

    sorted_collection
}

#[cfg(test)]
mod tests {
    use crate::test_sort;

    test_sort!(
        test_source_merge_sort,
        collection,
        collection = source_merge_sort(collection)
    );

    test_sort!(
        test_merge_sort_immutable,
        collection,
        collection = merge_sort_improved(&collection)
    );
}
