//! A delimited iterator over bytes.

use std::iter::FusedIterator;

use memchr;

/// An iterator over byte slices separated by a delimiter.
/// The iterated-over slices won't contain the delimiter, but may be empty.
#[derive(Clone)]
pub struct DelimIter<'a> {
    // possible simplification: no need for pos and bytes, just
    // mutate bytes to move array forward as next() is called
    bytes: &'a [u8],
    pos: usize,
    delim: u8,
}

impl<'a> DelimIter<'a> {
    pub fn new(bytes: &[u8], delim: u8) -> DelimIter<'_> {
        DelimIter {
            bytes,
            pos: 0,
            delim,
        }
    }
}

impl<'a> FusedIterator for DelimIter<'a> {}

impl<'a> Iterator for DelimIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<&'a [u8]> {
        if self.pos == self.bytes.len() {
            None
        } else {
            let start = self.pos;
            let bytes = &self.bytes[start..];
            let (end, new_pos) = match memchr::memchr(self.delim, bytes) {
                None => (bytes.len(), bytes.len()),
                Some(next_line) => (next_line, next_line + 1),
            };
            self.pos = start + new_pos;
            Some(&bytes[..end])
        }
    }
}

// tests: should match split values, on the cases below
//
// empty, all delimiter, no delimiter nonempty, two adjacent delimiter between normal worlds, all normal words, start and end with delim parse
