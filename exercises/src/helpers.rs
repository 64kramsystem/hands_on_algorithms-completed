#[macro_export]
macro_rules! test_sort {
  ($test_name:ident, $collection:ident, $stat:stmt) => {
      use super::*;

      #[test]
      fn $test_name() {
          let mut $collection = vec![
              1273, 18273, 8273, 827, 11, 213, 9172397, 2373, 2, 4, 20983, 29831093, 287,
              2837, 11, 92900,
          ];

          $stat

          let expected_collection = vec![
              2, 4, 11, 11, 213, 287, 827, 1273, 2373, 2837, 8273, 18273, 20983, 92900,
              9172397, 29831093,
          ];

          assert_eq!(&$collection, &expected_collection);
      }
  };
}

// Written for testing a discarded attempt; may be used, or not, in the future.
//
#[macro_export]
macro_rules! test_sort_refs {
  ($test_name:ident, $collection:ident, $stat:stmt) => {
      use super::*;

      #[test]
      fn $test_name() {
          let collection_vec = vec![
            1273, 18273, 8273, 827, 11, 213, 9172397, 2373, 2, 4, 20983, 29831093, 287,
            2837, 11, 92900,
          ];
          let mut $collection = collection_vec.iter().collect::<Vec<&i32>>();

          $stat

          let expected_collection_vec = vec![
            2, 4, 11, 11, 213, 287, 827, 1273, 2373, 2837, 8273, 18273, 20983, 92900,
            9172397, 29831093,
          ];
          let expected_collection = expected_collection_vec.iter().collect::<Vec<&i32>>();

          assert_eq!(&$collection, &expected_collection);
      }
  };
}
