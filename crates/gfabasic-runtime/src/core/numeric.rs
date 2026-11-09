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

/// Convert an integer expression to binary (Amiga GFA BASIC BIN$).
///
/// An optional width pads the result with leading zeroes and must be 1..=32.
pub fn bin(value: i32, width: Option<usize>) -> Result<String, RadixFormatError> {
    if let Some(width) = width {
        if !(1..=32).contains(&width) {
            return Err(RadixFormatError::BinaryWidthOutOfRange { width });
        }
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
    if let Some(width) = width {
        if width > 11 {
            return Err(RadixFormatError::OctalWidthOutOfRange { width });
        }
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
    use super::{RadixFormatError, bin, hex, oct};

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
