//! Integer-to-string radix conversions from the Amiga GFA BASIC reference manual.
//!
//! The API models a 32-bit integer expression. Negative values are interpreted
//! as their 32-bit two's-complement bit pattern; the original manual's cited
//! examples use non-negative values and do not specify negative formatting.

use std::error::Error;
use std::fmt;

/// Radix format validation failures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RadixFormatError {
    BinaryWidthOutOfRange { width: usize },
    OctalWidthOutOfRange { width: usize },
}

impl fmt::Display for RadixFormatError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BinaryWidthOutOfRange { width } => write!(
                formatter,
                "BIN$ width {width} is outside the documented range 1..=32"
            ),
            Self::OctalWidthOutOfRange { width } => write!(
                formatter,
                "OCT$ width {width} is outside the documented range 0..=11"
            ),
        }
    }
}

impl Error for RadixFormatError {}

/// Parse the numeric prefix of a byte string (Amiga GFA BASIC VAL).
///
/// Decimal input is read until the first byte that cannot continue a number.
/// The Amiga reference also specifies &H hexadecimal, &X binary, and &O octal
/// prefixes. If no numeric prefix is present, zero is returned.
pub fn val(input: &[u8]) -> f64 {
    val_with_length(input).0
}

/// Return the number of bytes consumed by VAL (Amiga GFA BASIC VAL?).
///
/// Leading ASCII whitespace is skipped but not counted. The returned length
/// includes an optional sign or radix prefix and the parsed numeric digits.
pub fn val_length(input: &[u8]) -> usize {
    val_with_length(input).1
}

/// Parse a numeric prefix and return its value and consumed byte length.
pub fn val_with_length(input: &[u8]) -> (f64, usize) {
    let mut start = input
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(input.len());
    if start == input.len() {
        return (0.0, 0);
    }
    let token_start = start;

    let sign = match input[start] {
        b'-' => {
            start += 1;
            -1.0
        }
        b'+' => {
            start += 1;
            1.0
        }
        _ => 1.0,
    };
    if start >= input.len() {
        return (0.0, 0);
    }

    // The Amiga base manual documents both ampersand radix prefixes
    // (&H, &X, &O) and the shorthand hexadecimal/binary forms ($ and %).
    // These inherited forms remain 3.0-source candidates unless separately
    // confirmed for the selected 3.5 interpreter.
    let (radix, prefix_length) = match input[start] {
        b'&' if start + 1 < input.len() => (
            match input[start + 1].to_ascii_uppercase() {
                b'H' => 16_u32,
                b'X' => 2_u32,
                b'O' => 8_u32,
                _ => 0,
            },
            2,
        ),
        b'$' => (16_u32, 1),
        b'%' => (2_u32, 1),
        _ => (0, 0),
    };
    if radix != 0 {
        let mut index = start + prefix_length;
        let mut value = 0.0;
        let mut digits = 0;
        while index < input.len() {
            let Some(digit) = digit_value(input[index]) else {
                break;
            };
            if digit >= radix {
                break;
            }
            value = value * f64::from(radix) + f64::from(digit);
            digits += 1;
            index += 1;
        }
        return if digits == 0 {
            (0.0, 0)
        } else {
            (sign * value, index - token_start)
        };
    }

    let number_start = start;
    let mut index = start;
    let mut digits = 0;
    while index < input.len() && input[index].is_ascii_digit() {
        digits += 1;
        index += 1;
    }
    if index < input.len() && input[index] == b'.' {
        index += 1;
        while index < input.len() && input[index].is_ascii_digit() {
            digits += 1;
            index += 1;
        }
    }
    if digits == 0 {
        return (0.0, 0);
    }

    if index < input.len() && matches!(input[index], b'e' | b'E') {
        let mut exponent_end = index + 1;
        if exponent_end < input.len() && matches!(input[exponent_end], b'+' | b'-') {
            exponent_end += 1;
        }
        let exponent_start = exponent_end;
        while exponent_end < input.len() && input[exponent_end].is_ascii_digit() {
            exponent_end += 1;
        }
        if exponent_end > exponent_start {
            index = exponent_end;
        }
    }

    let token = &input[number_start..index];
    let value = std::str::from_utf8(token)
        .ok()
        .and_then(|number| number.parse::<f64>().ok())
        .map_or(0.0, |value| sign * value);
    (value, index - token_start)
}

fn digit_value(byte: u8) -> Option<u32> {
    match byte {
        b'0'..=b'9' => Some(u32::from(byte - b'0')),
        b'a'..=b'f' => Some(u32::from(byte - b'a' + 10)),
        b'A'..=b'F' => Some(u32::from(byte - b'A' + 10)),
        _ => None,
    }
}

/// Convert an integer expression to binary (Amiga GFA BASIC BIN$).
///
/// An optional width pads the result with leading zeroes and must be 1..=32.
pub fn bin(value: i32, width: Option<usize>) -> Result<String, RadixFormatError> {
    if let Some(width) = width.filter(|width| !(1..=32).contains(width)) {
        return Err(RadixFormatError::BinaryWidthOutOfRange { width });
    }
    Ok(format!(
        "{:0width$b}",
        value as u32,
        width = width.unwrap_or(0)
    ))
}

/// Convert an integer expression to octal (Amiga GFA BASIC OCT$).
///
/// An optional width pads the result with leading zeroes and must be 0..=11.
pub fn oct(value: i32, width: Option<usize>) -> Result<String, RadixFormatError> {
    if let Some(width) = width.filter(|width| *width > 11) {
        return Err(RadixFormatError::OctalWidthOutOfRange { width });
    }
    Ok(format!(
        "{:0width$o}",
        value as u32,
        width = width.unwrap_or(0)
    ))
}

/// Convert an integer expression to uppercase hexadecimal (Amiga GFA BASIC HEX$).
///
/// An optional width pads the result with leading zeroes.
pub fn hex(value: i32, width: Option<usize>) -> String {
    format!("{:0width$X}", value as u32, width = width.unwrap_or(0))
}

#[cfg(test)]
mod tests {
    use super::{RadixFormatError, bin, hex, oct, val, val_length, val_with_length};

    #[test]
    fn val_parses_decimal_prefix_and_stops_at_first_invalid_byte() {
        assert_eq!(val(b"123abc"), 123.0);
        assert_eq!(val(b"  -12.5xyz"), -12.5);
        assert_eq!(val(b".75tail"), 0.75);
        assert_eq!(val(b"1.25e2rest"), 125.0);
        assert_eq!(val(b"1e+"), 1.0);
    }

    #[test]
    fn val_parses_documented_hex_binary_and_octal_prefixes() {
        assert_eq!(val(b"&HFFtail"), 255.0);
        assert_eq!(val(b"&x101"), 5.0);
        assert_eq!(val(b"-&O17rest"), -15.0);
    }

    #[test]
    fn val_question_reports_consumed_numeric_prefix_length() {
        assert_eq!(val_length(b"3.00 dollars"), 4);
        assert_eq!(val_length(b"  -12.5xyz"), 5);
        assert_eq!(val_length(b"&HFFtail"), 4);
        assert_eq!(val_length(b"word123"), 0);
        assert_eq!(val_with_length(b"3.00 dollars"), (3.0, 4));
    }

    #[test]
    fn val_accepts_documented_shorthand_hexadecimal_and_binary_prefixes() {
        // These examples are present in the 3.0 base-manual VAL section.
        // They are implemented pragmatically, not asserted as 3.5 conformance.
        assert_eq!(val(b"$AA"), 170.0);
        assert_eq!(val(b"%10101010"), 170.0);
        assert_eq!(val_with_length(b"$AA-tail"), (170.0, 3));
        assert_eq!(val_with_length(b"%10101010tail"), (170.0, 9));
        assert_eq!(val(b"$"), 0.0);
        assert_eq!(val(b"%"), 0.0);
    }

    #[test]
    fn val_returns_zero_when_no_numeric_prefix_exists() {
        assert_eq!(val(b""), 0.0);
        assert_eq!(val(b"  "), 0.0);
        assert_eq!(val(b"word123"), 0.0);
        assert_eq!(val(b"&H"), 0.0);
    }

    #[test]
    fn radix_conversions_match_reference_examples() {
        assert_eq!(hex(47, None), "2F");
        assert_eq!(oct(23, Some(4)).unwrap(), "0027");
        assert_eq!(bin(85, Some(8)).unwrap(), "01010101");
    }

    #[test]
    fn radix_widths_pad_but_never_truncate_values() {
        assert_eq!(bin(255, Some(4)).unwrap(), "11111111");
        assert_eq!(oct(8, Some(0)).unwrap(), "10");
        assert_eq!(hex(255, Some(4)), "00FF");
    }

    #[test]
    fn binary_and_octal_widths_enforce_documented_ranges() {
        assert_eq!(
            bin(1, Some(0)),
            Err(RadixFormatError::BinaryWidthOutOfRange { width: 0 })
        );
        assert_eq!(
            bin(1, Some(33)),
            Err(RadixFormatError::BinaryWidthOutOfRange { width: 33 })
        );
        assert_eq!(
            oct(1, Some(12)),
            Err(RadixFormatError::OctalWidthOutOfRange { width: 12 })
        );
    }
}
