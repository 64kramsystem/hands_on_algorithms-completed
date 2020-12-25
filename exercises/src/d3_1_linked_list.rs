// PartialOrd is for the exercise.
// Copy is for the list values function.
//
pub struct LinkedList<T: PartialOrd + Copy>(Option<(T, Box<LinkedList<T>>)>);

impl<T: PartialOrd + Copy> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let previous = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(previous))));
    }

    pub fn push_back_iterative(&mut self, data: T) {
        let mut current = self;

        while let Some((_, ref mut child)) = current.0 {
            current = child;
        }

        current.push_front(data);
    }

    pub fn push_back_recursive(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back_recursive(data),
            None => self.push_front(data),
        }
    }

    // Exercise
    //
    pub fn sorted_push_iterative(&mut self, value: T) {
        let mut current = self;

        loop {
            // The split `Some` arms are required due to limitations of the BCK; with Polonius, the
            // code works with a single `Some` arm.
            //
            match current.0 {
                Some((ref current_value, _)) if *current_value > value => {
                    break;
                }
                Some((_, ref mut child)) => {
                    current = child;
                }
                _ => {
                    break;
                }
            }
        }
        current.push_front(value);
    }

    pub fn values(&self) -> Vec<T> {
        let mut values = vec![];

        let mut current = self;

        while let Some((ref child_data, ref child)) = current.0 {
            values.push(*child_data);
            current = child;
        }

        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back_iterative() {
        let values = vec![
            1273, 18273, 8273, 827, 11, 213, 9172397, 2373, 2, 4, 20983, 29831093, 287, 2837, 11,
            92900,
        ];

        let mut list = LinkedList::new();

        for value in &values {
            list.push_back_iterative(*value);
        }

        assert_eq!(list.values()[..], values[..]);
    }

    #[test]
    fn sorted_insert() {
        let values = vec![
            1273, 18273, 8273, 827, 11, 213, 9172397, 2373, 2, 4, 20983, 29831093, 287, 2837, 11,
            92900,
        ];

        let mut sorted_values = values.clone();
        sorted_values.sort();

        let mut list = LinkedList::new();

        for value in values {
            list.sorted_push_iterative(value);
        }

        assert_eq!(list.values()[..], sorted_values[..]);
    }
}
