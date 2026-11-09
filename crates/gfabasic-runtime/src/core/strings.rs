//! Byte-oriented string functions from the Amiga GFA BASIC reference manual.
//!
//! The original interpreter uses byte strings. These helpers therefore operate
//! on byte slices rather than Rust UTF-8 character boundaries. String positions
//! accepted by GFA BASIC functions are one-based.

use std::error::Error;
use std::fmt;

/// Errors while constructing repeated GFA BASIC strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringError {
    RepeatCountOutOfRange { count: usize },
    LengthOverflow,
    AllocationFailed,
}

impl fmt::Display for StringError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RepeatCountOutOfRange { count } => {
                write!(formatter, "STRING$ repeat count {count} is outside 0..=32767")
            }
            Self::LengthOverflow => write!(formatter, "requested string length overflows usize"),
            Self::AllocationFailed => write!(formatter, "could not allocate requested string"),
        }
    }
}

impl Error for StringError {}

/// Repeat a byte string (Amiga GFA BASIC STRING$(count, expression$)).
pub fn string_repeat(input: &[u8], count: usize) -> Result<Vec<u8>, StringError> {
    if count > 32_767 {
        return Err(StringError::RepeatCountOutOfRange { count });
    }
    let length = input
        .len()
        .checked_mul(count)
        .ok_or(StringError::LengthOverflow)?;
    let mut output = Vec::new();
    output
        .try_reserve_exact(length)
        .map_err(|_| StringError::AllocationFailed)?;
    for _ in 0..count {
        output.extend_from_slice(input);
    }
    Ok(output)
}

/// Repeat the low byte of an integer expression (STRING$(count, code)).
pub fn string_repeat_code(code: i64, count: usize) -> Result<Vec<u8>, StringError> {
    string_repeat(&[code as u8], count)
}

/// Create a string containing count ASCII spaces (Amiga GFA BASIC SPACE$).
pub fn spaces(count: usize) -> Result<Vec<u8>, StringError> {
    let mut output = Vec::new();
    output
        .try_reserve_exact(count)
        .map_err(|_| StringError::AllocationFailed)?;
    output.resize(count, b' ');
    Ok(output)
}

/// Return the ASCII value of the first byte, or zero for an empty string.
pub fn asc(input: &[u8]) -> u8 {
    input.first().copied().unwrap_or(0)
}

/// Return the low byte of an integer expression (Amiga GFA BASIC CHR$).
pub fn chr(code: i64) -> u8 {
    code as u8
}

/// Return the preceding ASCII byte of the first character, when representable.
pub fn pred(input: &[u8]) -> Option<u8> {
    input.first().and_then(|character| character.checked_sub(1))
}

/// Return the succeeding ASCII byte of the first character, when representable.
pub fn succ(input: &[u8]) -> Option<u8> {
    input.first().and_then(|character| character.checked_add(1))
}

/// Return the leftmost count bytes, or the first byte when count is omitted.
pub fn left(input: &[u8], count: Option<usize>) -> &[u8] {
    let length = count.unwrap_or(1).min(input.len());
    &input[..length]
}

/// Return the rightmost count bytes, or the last byte when count is omitted.
pub fn right(input: &[u8], count: Option<usize>) -> &[u8] {
    let length = count.unwrap_or(1).min(input.len());
    &input[input.len() - length..]
}

/// Return bytes starting at the one-based start position.
///
/// A start position of zero or a position beyond the end returns an empty
/// slice. When count is omitted, all remaining bytes are returned.
pub fn mid(input: &[u8], start: usize, count: Option<usize>) -> &[u8] {
    if start == 0 || start > input.len() {
        return &input[input.len()..];
    }
    let offset = start - 1;
    let length = count
        .unwrap_or(input.len() - offset)
        .min(input.len() - offset);
    &input[offset..offset + length]
}

/// Search for a byte substring and return its one-based starting position.
///
/// An omitted start position begins at the first byte. A missing substring,
/// empty search string, or invalid zero start returns zero.
pub fn instr(input: &[u8], needle: &[u8], start: Option<usize>) -> usize {
    let start = start.unwrap_or(1);
    if start == 0 || start > input.len() || needle.is_empty() {
        return 0;
    }
    let offset = start - 1;
    input[offset..]
        .windows(needle.len())
        .position(|window| window == needle)
        .map_or(0, |position| offset + position + 1)
}

/// Search backward and return the one-based starting position of the last match.
///
/// An omitted start position begins at the rightmost byte. When supplied, the
/// position limits the search to that prefix of the input.
pub fn rinstr(input: &[u8], needle: &[u8], start: Option<usize>) -> usize {
    let end = start.unwrap_or(input.len()).min(input.len());
    if end == 0 || needle.is_empty() || needle.len() > end {
        return 0;
    }
    input[..end]
        .windows(needle.len())
        .rposition(|window| window == needle)
        .map_or(0, |position| position + 1)
}

/// Return the number of bytes in a GFA BASIC byte string.
pub fn len(input: &[u8]) -> usize {
    input.len()
}

/// Remove ASCII space bytes from both ends of a string.
pub fn trim(input: &[u8]) -> &[u8] {
    let start = input
        .iter()
        .position(|byte| *byte != b' ')
        .unwrap_or(input.len());
    let end = input
        .iter()
        .rposition(|byte| *byte != b' ')
        .map_or(start, |position| position + 1);
    &input[start..end]
}

/// Convert ASCII lowercase letters to uppercase, preserving all other bytes.
pub fn upper(input: &[u8]) -> Vec<u8> {
    input.iter().map(u8::to_ascii_uppercase).collect()
}

#[cfg(test)]
mod tests {
    use super::{
        asc, chr, instr, left, len, mid, pred, right, rinstr, spaces, string_repeat,
        string_repeat_code, succ, trim, upper, StringError,
    };

    #[test]
    fn asc_and_chr_use_first_byte_and_low_byte_conversion() {
        assert_eq!(asc(b"Amiga"), b'A');
        assert_eq!(asc(b""), 0);
        assert_eq!(chr(65), b'A');
        assert_eq!(chr(256), 0);
        assert_eq!(chr(-1), 255);
    }

    #[test]
    fn pred_and_succ_change_only_the_first_byte() {
        assert_eq!(pred(b"Basic"), Some(b'A'));
        assert_eq!(succ(b"Basic"), Some(b'C'));
        assert_eq!(pred(b""), None);
        assert_eq!(pred(&[0]), None);
        assert_eq!(succ(&[255]), None);
    }

    #[test]
    fn string_repeat_and_spaces_construct_expected_bytes() {
        assert_eq!(string_repeat(b"ab", 3), Ok(b"ababab".to_vec()));
        assert_eq!(string_repeat_code(65, 3), Ok(b"AAA".to_vec()));
        assert_eq!(string_repeat(b"x", 0), Ok(Vec::new()));
        assert_eq!(spaces(3), Ok(b"   ".to_vec()));
        assert_eq!(spaces(0), Ok(Vec::new()));
    }

    #[test]
    fn string_repeat_rejects_counts_above_documented_limit() {
        assert_eq!(
            string_repeat(b"x", 32_768),
            Err(StringError::RepeatCountOutOfRange { count: 32_768 })
        );
    }

    #[test]
    fn left_and_right_use_first_or_last_byte_when_count_is_omitted() {
        assert_eq!(left(b"Handbook", None), b"H");
        assert_eq!(right(b"Handbook", None), b"k");
    }

    #[test]
    fn left_and_right_clamp_counts_to_input_length() {
        assert_eq!(left(b"abc", Some(2)), b"ab");
        assert_eq!(right(b"abc", Some(2)), b"bc");
        assert_eq!(left(b"abc", Some(9)), b"abc");
        assert_eq!(right(b"abc", Some(9)), b"abc");
        assert_eq!(left(b"abc", Some(0)), b"");
        assert_eq!(right(b"abc", Some(0)), b"");
    }

    #[test]
    fn mid_uses_one_based_positions_and_optional_length() {
        assert_eq!(mid(b"Handbook", 3, Some(4)), b"ndbo");
        assert_eq!(mid(b"Handbook", 5, None), b"book");
        assert_eq!(mid(b"Handbook", 9, None), b"");
        assert_eq!(mid(b"Handbook", 0, Some(2)), b"");
        assert_eq!(mid(b"abc", 2, Some(0)), b"");
    }

    #[test]
    fn instr_returns_one_based_position_and_honors_start() {
        assert_eq!(instr(b"GFA-Systemtechnik", b"System", None), 5);
        assert_eq!(instr(b"GFA-BASIC", b"BASIC", Some(6)), 0);
        assert_eq!(instr(b"  ", b" ", None), 1);
        assert_eq!(instr(b"abc", b"z", None), 0);
    }

    #[test]
    fn rinstr_searches_from_the_right_or_a_given_prefix() {
        assert_eq!(rinstr(b"a/b/c", b"/", None), 4);
        assert_eq!(rinstr(b"a/b/c", b"/", Some(3)), 2);
        assert_eq!(rinstr(b"a/b/c", b"z", None), 0);
    }

    #[test]
    fn len_counts_bytes_and_trim_removes_only_ascii_spaces() {
        assert_eq!(len(b"Test"), 4);
        assert_eq!(trim(b"  test  "), b"test");
        assert_eq!(trim(b"   "), b"");
        assert_eq!(trim(b"\t test \t"), b"\t test \t");
    }

    #[test]
    fn upper_changes_ascii_lowercase_only() {
        assert_eq!(upper(b"GFA-Basic 3.5!"), b"GFA-BASIC 3.5!");
        assert_eq!(upper(&[b'a', 0xe4, b'z']), [b'A', 0xe4, b'Z']);
    }
}
