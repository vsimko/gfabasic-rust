//! Limited candidate implementations for inherited Amiga GFA BASIC 3.5 functions.
//!
//! Detailed behavior inherited only from the 3.0 base manual remains partial.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static RANDOM_STATE: AtomicU64 = AtomicU64::new(0);

fn next_random_u64() -> u64 {
    let mut current = RANDOM_STATE.load(Ordering::Relaxed);
    loop {
        if current == 0 {
            let seed = (SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as u64)
                | 1;
            current = match RANDOM_STATE.compare_exchange(
                0,
                seed,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => seed,
                Err(actual) => actual,
            };
        }
        let mut next = current;
        next ^= next >> 12;
        next ^= next << 25;
        next ^= next >> 27;
        if next == 0 {
            next = 0x9e37_79b9_7f4a_7c15;
        }
        match RANDOM_STATE.compare_exchange_weak(
            current,
            next,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => return next.wrapping_mul(0x2545_f491_4f6c_dd1d),
            Err(actual) => current = actual,
        }
    }
}

fn random_unit() -> f64 {
    ((next_random_u64() >> 11) as f64) * (1.0 / ((1_u64 << 53) as f64))
}

/// Candidate RAND bound behavior follows the 3.0 base manual; exact 3.5 range,
/// width, distribution, and seeding remain unverified.
pub fn rand_integer(bound: i64) -> Option<i64> {
    let bound = u64::try_from(bound).ok().filter(|v| *v > 0)?;
    let accepted_below = u64::MAX - (u64::MAX % bound);
    loop {
        let sample = next_random_u64();
        if sample < accepted_below {
            return i64::try_from(sample % bound).ok();
        }
    }
}

/// Numeric successor for the evidenced integer-expression form.
pub fn succ_integer(value: i64) -> Option<i64> {
    value.checked_add(1)
}

/// Repeat the supplied byte string; no historical count ceiling is asserted.
pub fn repeat_string(count: i64, value: &[u8]) -> Option<Vec<u8>> {
    let count = usize::try_from(count).ok()?;
    let length = value.len().checked_mul(count)?;
    let mut output = Vec::new();
    output.try_reserve_exact(length).ok()?;
    for _ in 0..count {
        output.extend_from_slice(value);
    }
    Some(output)
}

/// Positive one-argument HEX$ subset; negative/width behavior remains unverified.
pub fn hex_string(value: i64) -> Option<String> {
    let value = i32::try_from(value).ok()?;
    if value < 0 {
        return None;
    }
    Some(crate::core::numeric::hex(value, None))
}

/// RND(1) candidate; [0,1) and ignored argument follow only 3.0 base semantics.
pub fn rnd(argument: f64) -> Option<f64> {
    argument.is_finite().then(random_unit)
}

/// INT subset for finite non-negative values; negative rounding is unverified.
pub fn int(value: f64) -> Option<i64> {
    if !value.is_finite() || value < 0.0 || value >= i64::MAX as f64 {
        return None;
    }
    Some(value.trunc() as i64)
}

/// SQR subset for finite non-negative inputs; negative-input behavior is unverified.
pub fn sqr(value: f64) -> Option<f64> {
    if !value.is_finite() || value < 0.0 {
        return None;
    }
    let result = value.sqrt();
    result.is_finite().then_some(result)
}

#[cfg(test)]
mod tests {
    use super::{hex_string, int, rand_integer, repeat_string, rnd, sqr, succ_integer};

    #[test]
    fn successor_checks_overflow() {
        assert_eq!(succ_integer(2), Some(3));
        assert_eq!(succ_integer(i64::MAX), None);
    }

    #[test]
    fn string_repetition_uses_the_evidenced_string_argument_form() {
        assert_eq!(repeat_string(3, b"-"), Some(b"---".to_vec()));
        assert_eq!(repeat_string(0, b"x"), Some(Vec::new()));
        assert_eq!(repeat_string(-1, b"x"), None);
    }

    #[test]
    fn hexadecimal_conversion_covers_positive_one_argument_form() {
        assert_eq!(hex_string(47).as_deref(), Some("2F"));
        assert_eq!(hex_string(-1), None);
    }

    #[test]
    fn random_candidates_stay_within_provisional_ranges() {
        assert_eq!(rand_integer(1), Some(0));
        assert_eq!(rand_integer(0), None);
        assert!(rand_integer(10).is_some_and(|v| (0..10).contains(&v)));
        assert!(rnd(1.0).is_some_and(|v| (0.0..1.0).contains(&v)));
        assert_eq!(rnd(f64::NAN), None);
    }

    #[test]
    fn integer_conversion_is_limited_to_non_negative_finite_values() {
        assert_eq!(int(5.9), Some(5));
        assert_eq!(int(-0.1), None);
        assert_eq!(int(f64::INFINITY), None);
    }

    #[test]
    fn square_root_is_limited_to_non_negative_finite_values() {
        assert_eq!(sqr(9.0), Some(3.0));
        assert_eq!(sqr(-1.0), None);
    }
}
