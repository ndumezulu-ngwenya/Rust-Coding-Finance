#[allow(dead_code)]
/// Calculates the GCD of a given i32 array or returns None if the array is empty.
fn calculate_gcd_array(ints: &[i32]) -> Option<i32> {
    if ints.is_empty() {
        return None;
    }

    let mut res = ints[0];
    for i in ints {
        res = calculate_gcd(res, *i);
    }

    Some(res)
}

/// Calculates the GCD of two i32 integer values.
fn calculate_gcd(mut a: i32, mut b: i32) -> i32 {
    if a < b {
        (a, b) = (b, a);
    }

    while b != 0 {
        (b, a) = (a % b, b);
    }

    a
}

#[cfg(test)]
mod tests {
    use super::{calculate_gcd, calculate_gcd_array};

    #[test]
    fn test_calculate_gcd_array() {
        assert_eq!(calculate_gcd_array(&[]), None);
        assert_eq!(calculate_gcd_array(&[6]), Some(6));
        assert_eq!(calculate_gcd_array(&[4, 64, 32, 120]), Some(4));
    }

    #[test]
    fn test_calculate_gcd() {
        assert_eq!(calculate_gcd(11, 22), 11);
    }
}
