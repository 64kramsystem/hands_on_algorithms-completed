// The name "dynamic" is somewhat misleading. Dynamic programming means essentially to store and reuse
// the computed values.

pub fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    fibonacci(n - 1) + fibonacci(n - 2)
}

pub fn fibonacci_dynamic(n: u32) -> (u32, u32) {
    if n == 0 {
        return (1, 0);
    }

    let (last, beforelast) = fibonacci_dynamic(n - 1);

    (last + beforelast, last)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fibonacci_dynamic() {
        use super::*;

        for i in 0..20 {
            assert_eq!(fibonacci(i), fibonacci_dynamic(i).0);
        }
    }
}
