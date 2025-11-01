pub fn concat_numbers(left: u64, right: u64) -> u64 {
    if right == 0 {
        return left * 10; // Special case: treat 0 as 1 digit
    }

    let digits = right.ilog10() + 1; // Count digits in right number
    left * 10u64.pow(digits) + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_concatenation() {
        assert_eq!(concat_numbers(12, 345), 12345);
        assert_eq!(concat_numbers(1, 2), 12);
    }

    #[test]
    fn test_single_digits() {
        assert_eq!(concat_numbers(5, 9), 59);
        assert_eq!(concat_numbers(0, 1), 1);
        assert_eq!(concat_numbers(1, 0), 10);
    }

    #[test]
    fn test_zero_cases() {
        assert_eq!(concat_numbers(0, 0), 0);
        assert_eq!(concat_numbers(0, 5), 5);
        assert_eq!(concat_numbers(5, 0), 50);
        assert_eq!(concat_numbers(12, 0), 120);
        assert_eq!(concat_numbers(100, 0), 1000);
    }

    #[test]
    fn test_different_lengths() {
        assert_eq!(concat_numbers(1, 23), 123);
        assert_eq!(concat_numbers(12, 3), 123);
        assert_eq!(concat_numbers(123, 4567), 1234567);
        assert_eq!(concat_numbers(99, 1), 991);
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(concat_numbers(999, 999), 999999);
        assert_eq!(concat_numbers(12345, 67890), 1234567890);
        assert_eq!(concat_numbers(1_000_000, 999_999), 1_000_000_999_999);
    }

    #[test]
    fn test_powers_of_ten() {
        assert_eq!(concat_numbers(10, 20), 1020);
        assert_eq!(concat_numbers(100, 200), 100200);
        assert_eq!(concat_numbers(1000, 1), 10001);
    }

    #[test]
    #[should_panic]
    fn test_overflow_behavior() {
        // If this is a concern, test what happens near u64::MAX
        // This test documents the behavior rather than asserting correctness
        let _ = concat_numbers(u64::MAX, 1);
    }
}

#[cfg(test)]
mod proptests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_concat_is_monotonic(left in 0u64..10000, right in 0u64..10000) {
            // Result should always be >= left (when left != 0)
            if left > 0 {
                prop_assert!(concat_numbers(left, right) >= left);
            }
        }

        #[test]
        fn test_concat_preserves_digits(a in 1u64..1000, b in 1u64..1000) {
            let result = concat_numbers(a, b);
            let result_str = result.to_string();
            let expected = format!("{}{}", a, b);
            prop_assert_eq!(result_str, expected);
        }
    }
}
