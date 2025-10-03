//! Cursor for efficient input traversal with SIMD optimizations
//!
//! This module provides a cursor implementation optimized for parsing,
//! with SIMD-accelerated whitespace skipping and boundary checking.

use std::{ascii::escape_default, fmt::Display};

/// Cursor for traversing input text with SIMD optimizations
#[derive(Debug, Clone)]
pub struct Cursor<'a> {
    /// The input we're scanning
    pub input: &'a [u8],

    /// The location of the cursor in the input
    pub pos: usize,

    /// Is the cursor at the start of the input
    pub at_start: bool,

    /// Is the cursor at the end of the input
    pub at_end: bool,

    /// The previously consumed character
    /// If `at_start` is true, this will be NUL
    pub prev: u8,

    /// The current character
    pub curr: u8,

    /// The upcoming character (if any)
    /// If `at_end` is true, this will be NUL
    pub next: u8,
}

impl<'a> Cursor<'a> {
    /// Create a new cursor for the given input
    pub fn new(input: &'a [u8]) -> Self {
        let mut cursor = Self {
            input,
            pos: 0,
            at_start: true,
            at_end: false,
            prev: 0x00,
            curr: 0x00,
            next: 0x00,
        };
        cursor.move_to(0);
        cursor
    }

    /// Advance the cursor by one character
    #[inline(always)]
    pub fn advance(&mut self) {
        self.pos += 1;

        self.prev = self.curr;
        self.curr = self.next;
        self.next = *self
            .input
            .get(self.pos.saturating_add(1))
            .unwrap_or(&0x00u8);
    }

    /// Advance the cursor by two characters
    #[inline(always)]
    pub fn advance_twice(&mut self) {
        self.pos += 2;

        self.prev = self.next;
        self.curr = *self.input.get(self.pos).unwrap_or(&0x00u8);
        self.next = *self
            .input
            .get(self.pos.saturating_add(1))
            .unwrap_or(&0x00u8);
    }

    /// Advance the cursor by the specified number of characters
    #[inline(always)]
    pub fn advance_by(&mut self, amount: usize) {
        self.move_to(self.pos.saturating_add(amount));
    }

    /// Move cursor to a specific position
    pub fn move_to(&mut self, pos: usize) {
        let len = self.input.len();
        let pos = pos.clamp(0, len);

        self.pos = pos;
        self.at_start = pos == 0;
        self.at_end = pos + 1 >= len;

        self.prev = *self.input.get(pos.wrapping_sub(1)).unwrap_or(&0x00u8);
        self.curr = *self.input.get(pos).unwrap_or(&0x00u8);
        self.next = *self.input.get(pos.saturating_add(1)).unwrap_or(&0x00u8);
    }

    /// Skip whitespace using SIMD-accelerated operations when possible
    #[inline(always)]
    pub fn skip_whitespace_simd(&mut self) -> bool {
        if let Some(new_pos) = fast_skip::fast_skip_whitespace(self) {
            self.move_to(new_pos);
            true
        } else {
            false
        }
    }

    /// Skip whitespace using traditional byte-by-byte approach
    pub fn skip_whitespace_fallback(&mut self) {
        while self.pos < self.input.len() && self.curr.is_ascii_whitespace() {
            self.advance();
        }
    }

    /// Skip whitespace (uses SIMD when available, falls back to byte-by-byte)
    pub fn skip_whitespace(&mut self) {
        if !self.skip_whitespace_simd() {
            self.skip_whitespace_fallback();
        }
    }

    /// Check if we're at the end of input
    #[inline(always)]
    pub fn is_at_end(&self) -> bool {
        self.at_end
    }

    /// Get the current character as a char (unsafe for performance)
    #[inline(always)]
    pub unsafe fn current_char_unchecked(&self) -> char {
        char::from_u32_unchecked(self.curr as u32)
    }

    /// Get the current character as a char (safe)
    #[inline(always)]
    pub fn current_char(&self) -> Option<char> {
        char::from_u32(self.curr as u32)
    }

    /// Peek ahead by n characters
    pub fn peek(&self, n: usize) -> Option<u8> {
        self.input.get(self.pos + n).copied()
    }

    /// Get remaining input from current position
    #[inline(always)]
    pub fn remaining(&self) -> &[u8] {
        &self.input[self.pos..]
    }

    /// Get a slice from current position to end
    #[inline(always)]
    pub fn slice_from_current(&self) -> &[u8] {
        self.remaining()
    }

    /// Get a slice from start position to current position
    #[inline(always)]
    pub fn slice_to_current(&self, start: usize) -> &[u8] {
        &self.input[start..self.pos]
    }
}

impl Display for Cursor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.input.len().to_string();

        let pos = format!("{: >len_count$}", self.pos, len_count = len.len());
        write!(f, "{}/{} ", pos, len)?;

        if self.at_start {
            write!(f, "S ")?;
        } else if self.at_end {
            write!(f, "E ")?;
        } else {
            write!(f, "M ")?;
        }

        fn to_str(c: u8) -> String {
            if c == 0x00 {
                "NUL".into()
            } else {
                format!("{:?}", escape_default(c).to_string())
            }
        }

        write!(
            f,
            "[{} {} {}]",
            to_str(self.prev),
            to_str(self.curr),
            to_str(self.next)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cursor() {
        let mut cursor = Cursor::new(b"hello world");
        assert_eq!(cursor.pos, 0);
        assert!(cursor.at_start);
        assert!(!cursor.at_end);
        assert_eq!(cursor.prev, 0x00);
        assert_eq!(cursor.curr, b'h');
        assert_eq!(cursor.next, b'e');

        cursor.advance_by(1);
        assert_eq!(cursor.pos, 1);
        assert!(!cursor.at_start);
        assert!(!cursor.at_end);
        assert_eq!(cursor.prev, b'h');
        assert_eq!(cursor.curr, b'e');
        assert_eq!(cursor.next, b'l');

        // Advancing too far should stop at the end
        cursor.advance_by(10);
        assert_eq!(cursor.pos, 11);
        assert!(!cursor.at_start);
        assert!(cursor.at_end);
        assert_eq!(cursor.prev, b'd');
        assert_eq!(cursor.curr, 0x00);
        assert_eq!(cursor.next, 0x00);

        // Can't advance past the end
        cursor.advance_by(1);
        assert_eq!(cursor.pos, 11);
        assert!(!cursor.at_start);
        assert!(cursor.at_end);
        assert_eq!(cursor.prev, b'd');
        assert_eq!(cursor.curr, 0x00);
        assert_eq!(cursor.next, 0x00);
    }

    #[test]
    fn test_skip_whitespace() {
        let mut cursor = Cursor::new(b"   \t\n\r   hello");
        cursor.skip_whitespace();
        assert_eq!(cursor.pos, 8);
        assert_eq!(cursor.curr, b'h');
    }
}

/// SIMD-accelerated fast skipping operations
pub mod fast_skip {
    use super::Cursor;

    const STRIDE: usize = 16;
    type Mask = [bool; STRIDE];

    /// Fast skip whitespace using SIMD operations
    #[inline(always)]
    pub fn fast_skip_whitespace(cursor: &Cursor) -> Option<usize> {
        // If we don't have enough bytes left to check then bail early
        if cursor.pos + STRIDE >= cursor.input.len() {
            return None;
        }

        if !cursor.curr.is_ascii_whitespace() {
            return None;
        }

        let mut offset = 1;

        // SAFETY: We've already checked (indirectly) that this index is valid
        let remaining = unsafe { cursor.input.get_unchecked(cursor.pos..) };

        // NOTE: This loop uses primitives designed to be auto-vectorized
        // Do not change this loop without benchmarking the results
        // And checking the generated assembly using godbolt.org
        for (i, chunk) in remaining.chunks_exact(STRIDE).enumerate() {
            let value = load(chunk);
            let is_whitespace = is_ascii_whitespace(value);
            let is_all_whitespace = all_true(is_whitespace);

            if is_all_whitespace {
                offset = (i + 1) * STRIDE;
            } else {
                break;
            }
        }

        Some(cursor.pos + offset)
    }

    #[inline(always)]
    fn load(input: &[u8]) -> [u8; STRIDE] {
        let mut value = [0u8; STRIDE];
        value.copy_from_slice(input);
        value
    }

    #[inline(always)]
    fn eq(input: [u8; STRIDE], val: u8) -> Mask {
        let mut res = [false; STRIDE];
        for n in 0..STRIDE {
            res[n] = input[n] == val
        }
        res
    }

    #[inline(always)]
    fn or(a: Mask, b: Mask) -> Mask {
        let mut res = [false; STRIDE];
        for n in 0..STRIDE {
            res[n] = a[n] | b[n];
        }
        res
    }

    #[inline(always)]
    fn all_true(a: Mask) -> bool {
        let mut res = true;
        for item in a.iter().take(STRIDE) {
            res &= item;
        }
        res
    }

    #[inline(always)]
    fn is_ascii_whitespace(value: [u8; STRIDE]) -> Mask {
        let whitespace_1 = eq(value, b'\t');
        let whitespace_2 = eq(value, b'\n');
        let whitespace_3 = eq(value, b'\x0C');
        let whitespace_4 = eq(value, b'\r');
        let whitespace_5 = eq(value, b' ');

        or(
            or(
                or(or(whitespace_1, whitespace_2), whitespace_3),
                whitespace_4,
            ),
            whitespace_5,
        )
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::cursor::Cursor;

        #[test]
        fn test_fast_skip_whitespace() {
            let input = b"   \t\n\r   hello world";
            let cursor = Cursor::new(input);

            let skip_pos = fast_skip_whitespace(&cursor);
            assert_eq!(skip_pos, Some(8));
        }

        #[test]
        fn test_no_skip_when_not_whitespace() {
            let input = b"hello world";
            let cursor = Cursor::new(input);

            let skip_pos = fast_skip_whitespace(&cursor);
            assert_eq!(skip_pos, None);
        }

        #[test]
        fn test_skip_at_end() {
            let input = b"   "; // exactly at end
            let cursor = Cursor::new(input);
            let cursor_at_end = Cursor {
                pos: 3,
                at_end: true,
                ..cursor
            };

            let skip_pos = fast_skip_whitespace(&cursor_at_end);
            assert_eq!(skip_pos, None);
        }
    }
}
