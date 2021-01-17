//! Load a file as CSR end-to-end.

use std::convert::TryInto;
use std::fs;
use std::path::Path;
use std::str;

use rayon;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::ParallelSlice;

use crate::delim_iter::DelimIter;
use crate::fileblocks;

#[derive(Clone, Default)]
pub struct CsrMatrix {
    pub(crate) y: Vec<f64>,
    pub(crate) data: Vec<f64>,
    pub(crate) indices: Vec<u64>,
    pub(crate) indptr: Vec<u64>,
}

/// Parse svmlight in parallel, relying on a minimum chunk size
pub fn svmlight_to_csr(fname: &Path, min_chunk_size: usize) -> CsrMatrix {
    // use a few more chunks than threads to allow rayon to load balance naturally
    let max_chunks = rayon::current_num_threads() * 4;
    let metadata = fs::metadata(fname).unwrap();
    let size: usize = metadata.len().try_into().unwrap();
    let max_chunks = max_chunks.min(size / min_chunk_size).max(1);
    let chunks = fileblocks::chunkify(&fname, max_chunks);

    let folds: Vec<_> = chunks
        .par_iter()
        .map(|chunk| {
            chunk.lines().fold(CsrMatrix::default(), |mut acc, line| {
                let start = acc.indices.len();
                let words = DelimIter::new(&line, b' ');
                let svm_line = parse(words);
                acc.y.push(svm_line.target());
                svm_line.for_each(|(feature, value)| {
                    acc.indices.push(feature);
                    acc.data.push(value);
                });
                assert!(acc.indices[start..].windows(2).all(|s| s[0] < s[1]));
                acc.indptr.push(start.try_into().unwrap());
                acc
            })
        })
        .collect();

    // interesting extension here would be to figure out the sizes
    // of all the folds, then allocate final result vectors, and then
    // have rayon threads write in parallel using the wicked cool slice mut
    // technique here:
    // https://github.com/sisudata/chromatic-encoding/blob/18728fce08fa3abbbb189964dd52c783bd48800a/crank/src/graphio.rs#L200-L217
    let nrows = folds.iter().map(|csr| csr.indptr.len()).sum();
    let ndata = folds.iter().map(|csr| csr.indices.len()).sum();
    let init = CsrMatrix {
        y: Vec::with_capacity(nrows),
        data: Vec::with_capacity(ndata),
        indices: Vec::with_capacity(ndata),
        indptr: Vec::with_capacity(nrows + 1),
    };
    let mut stacked = folds.into_iter().fold(init, |mut acc, x| {
        let offset: u64 = acc.indices.len().try_into().unwrap();
        acc.y.extend(x.y.into_iter());
        acc.indices.extend(x.indices.into_iter());
        acc.data.extend(x.data.into_iter());
        acc.indptr.extend(x.indptr.into_iter().map(|i| i + offset));
        acc
    });
    assert!(stacked.y.len() == stacked.indptr.len());
    assert!(stacked.indptr.par_windows(2).all(|s| s[0] <= s[1]));
    assert!(stacked
        .indptr
        .last()
        .iter()
        .all(|&&i| i <= stacked.indices.len().try_into().unwrap()));
    stacked
        .indptr
        .push(stacked.indices.len().try_into().unwrap());

    stacked
}

/// Given a [`DelimIter`] pointing to the front of a line in a
/// simsvm file, this wrapper is a convenient iterator over
/// just the features in that line.
#[derive(Clone)]
pub struct SvmlightLineIter<'a> {
    target: &'a [u8],
    iter: DelimIter<'a>,
}

pub fn parse(mut iter: DelimIter<'_>) -> SvmlightLineIter<'_> {
    let target = iter.next().expect("target");
    SvmlightLineIter { target, iter }
}

impl<'a> Iterator for SvmlightLineIter<'a> {
    type Item = (u64, f64);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(word) = self.iter.next() {
            if word.is_empty() {
                continue;
            }
            let string = str::from_utf8(word).expect("utf-8");
            let (feature, value) = string
                .rfind(':')
                .map(|pos| (&string[..pos], &string[pos + 1..]))
                .expect("feature-value pair");
            return Some((
                feature.parse().expect("parse feature"),
                value.parse().expect("parse value"),
            ));
        }
        None
    }
}

impl<'a> SvmlightLineIter<'a> {
    pub fn target(&self) -> f64 {
        let string = str::from_utf8(self.target).expect("utf-8");
        string.parse().expect("target parse")
    }
}

// tests:
//
// simple line-with-target nd line-with-two features parse line test
//
// svmlight_to_csr effectively tested by python, no need to repeat here b/c binding
// is a thin wrapper
