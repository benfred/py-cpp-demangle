//! Utilities for converting files into blocks.

use std::convert::TryInto;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind};
use std::io::{Seek, SeekFrom};
use std::iter;
use std::path::Path;
use std::path::PathBuf;

use memchr;

pub struct FileChunk {
    path: PathBuf,
    start: usize,
    stop: usize,
}

impl FileChunk {
    /// Iterates over just those lines the file chunk refers to.
    pub fn lines(&self) -> impl Iterator<Item = Vec<u8>> {
        // interesting extension for this method would be to avoid
        // copies in the common case by relying on the BufReader's buffer
        // and returning an iterator into &[u8] slices to that directly
        // (with a fallback for larger-than-buffer lines, "simple" fallback
        // could just be to allocate a bigger bufreader and seek back).
        //
        // this could possibly require creating a separate BufReader owner
        // that generates an iterator, depending on from_fn lifetimes,
        // or just boxing the closure.
        let mut file = File::open(&self.path).expect("file available");
        file.seek(SeekFrom::Start(self.start.try_into().unwrap()))
            .expect("seek");
        let reader = BufReader::new(file);
        let mut current_byte = self.start;
        let stop_byte = self.stop;
        let mut split_it = reader.split(b'\n');
        iter::from_fn(move || {
            if current_byte >= stop_byte {
                return None;
            }
            assert!(
                current_byte < stop_byte + 1,
                "can only overshoot if non-newline split or eof with no newline"
            );
            let line = split_it.next().expect("should not have hit eof");
            let line = line.expect("line read");
            current_byte += line.len() + 1;
            Some(line)
        })
    }
}

/// Returns a list of up to `nchunks` file chunks splitting up the given
/// file, roughly of the same size, newline aligned.
///
/// Of course, the file is assumed to not be modified between the start of this method and the usage
/// of the corresponding file chunks, else someone will panic.
pub(crate) fn chunkify(path: &Path, max_chunks: usize) -> Vec<FileChunk> {
    assert!(max_chunks > 0);
    let metadata = fs::metadata(path).unwrap();
    let size: usize = metadata.len().try_into().unwrap();

    let mut file = File::open(path).unwrap();
    let mut chunks = Vec::with_capacity(max_chunks);
    let mut current_byte = 0;
    for i in 0..max_chunks {
        let stop = size * (i + 1) / max_chunks;

        // in the rare case when a line takes up a whole block, skip it
        if current_byte >= stop {
            continue;
        }

        file.seek(SeekFrom::Start(stop.try_into().unwrap()))
            .expect("seek");
        let mut reader = BufReader::new(&mut file);
        let stop = read_until(b'\n', &mut reader);

        chunks.push(FileChunk {
            path: path.to_owned(),
            start: current_byte,
            stop,
        });
        current_byte = stop;

        if stop == size {
            break;
        }
    }

    chunks
}

fn read_until<R: BufRead + ?Sized>(delim: u8, r: &mut R) -> usize {
    // from stdlib
    let mut read = 0;
    loop {
        let (done, used) = {
            let available = match r.fill_buf() {
                Ok(n) => n,
                Err(ref e) if e.kind() == ErrorKind::Interrupted => continue,
                x => x.unwrap(),
            };
            match memchr::memchr(delim, available) {
                Some(i) => (true, i + 1),
                None => (false, available.len()),
            }
        };
        r.consume(used);
        read += used;
        if done || used == 0 {
            return read;
        }
    }
}

// tests: chunkify().map(lines).join(b"\n") + ending newline if initially present
// should recover the original file
//
// test on empty file, one newline, two newlines, three newlines, 10, 100 newlines
// cross 1, 2, 3, 10 max chunks
// cross with/without trailing newline
// cross with/without lots of empty lines (else just mixed length cycle)
