//! Checked combinatorics primitives used by the Amiga GFA BASIC 3.5 runtime.
//!
//! The 3.5 addendum documents FACT, VARIAT, and COMBIN as arithmetic
//! functions over integer-expression arguments. It explicitly defines
//! FACT(0) = 1. The companion 1990 book defines VARIAT/COMBIN for n >= k.
//!
//! These helpers use u128 and return None when the input domain is invalid or
//! the exact result cannot be represented. This is an implementation boundary,
//! not a claim about the original interpreter's overflow/error behavior.

/// Return n! when it fits in an unsigned 128-bit integer.
///
/// The documented special case FACT(0) = 1 is handled naturally.
pub fn factorial(n: u32) -> Option<u128> {
    (1..=n).try_fold(1_u128, |acc, value| acc.checked_mul(u128::from(value)))
}

/// Return the number of ordered k-permutations of n elements without repetition.
///
/// The documented domain is n >= k. A zero-length permutation is one
/// arrangement, including when n and k are both zero.
pub fn variat(n: u32, k: u32) -> Option<u128> {
    if k > n {
        return None;
    }
    if k == 0 {
        return Some(1);
    }

    ((n - k + 1)..=n).try_fold(1_u128, |acc, value| {
        acc.checked_mul(u128::from(value))
    })
}

/// Return the number of k-combinations of n elements without repetition.
///
/// Uses an exact multiplicative recurrence and reduces each fraction before
/// multiplication to avoid avoidable intermediate overflow.
pub fn combin(n: u32, k: u32) -> Option<u128> {
    if k > n {
        return None;
    }

    let k = k.min(n - k);
    (1..=k).try_fold(1_u128, |acc, step| {
        let mut numerator = u128::from(n - k + step);
        let mut denominator = u128::from(step);

        let common_with_acc = gcd(acc, denominator);
        let reduced_acc = acc / common_with_acc;
        denominator /= common_with_acc;

        let common_with_numerator = gcd(numerator, denominator);
        numerator /= common_with_numerator;
        denominator /= common_with_numerator;

        // The binomial recurrence is integral at every step.
        debug_assert_eq!(denominator, 1);
        reduced_acc.checked_mul(numerator)
    })
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

#[cfg(test)]
mod tests {
    use super::{combin, factorial, variat};

    #[test]
    fn factorial_matches_documented_zero_and_small_values() {
        assert_eq!(factorial(0), Some(1));
        assert_eq!(factorial(1), Some(1));
        assert_eq!(factorial(5), Some(120));
    }

    #[test]
    fn factorial_reports_unrepresentable_results() {
        assert_eq!(factorial(34), Some(295_232_799_039_604_140_847_618_609_643_520_000_000));
        assert_eq!(factorial(35), None);
    }

    #[test]
    fn variat_matches_permutations_without_repetition() {
        assert_eq!(variat(5, 0), Some(1));
        assert_eq!(variat(5, 1), Some(5));
        assert_eq!(variat(5, 3), Some(60));
        assert_eq!(variat(5, 5), Some(120));
        assert_eq!(variat(2, 3), None);
    }

    #[test]
    fn combin_matches_combinations_without_repetition() {
        assert_eq!(combin(5, 0), Some(1));
        assert_eq!(combin(5, 1), Some(5));
        assert_eq!(combin(5, 2), Some(10));
        assert_eq!(combin(5, 3), Some(10));
        assert_eq!(combin(5, 5), Some(1));
        assert_eq!(combin(2, 3), None);
    }

    #[test]
    fn combin_reduces_intermediate_products() {
        assert_eq!(combin(100, 50), Some(100_891_344_545_564_193_334_812_497_256));
    }

    #[test]
    fn functions_detect_exact_result_overflow() {
        assert_eq!(variat(35, 35), None);
        assert_eq!(combin(200, 100), None);
    }
}
