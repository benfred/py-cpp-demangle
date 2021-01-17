//! SVMLight reader crate with python bindings via PyO3.

mod delim_iter;
mod fileblocks;
mod svmlight;

use byte_slice_cast::AsByteSlice;
use pyo3::prelude::*;
use pyo3::types::PyByteArray;
use pyo3::wrap_pyfunction;

/// SVMlight loader bindings from Rust.
///
/// Not intended to be called directly.
#[pymodule]
fn svm2csr(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rs_load, m)?)?;

    Ok(())
}

/// Rust svmlight loader binding.
///
/// Not intended to be called directly. Instead, see :func:`~svm2csr.load_svmlight_file`.
///
/// Returns a four-tuple of bytes containing (y, data, indices, indptr) with types
/// (f8, f8, u8, u8) in native endianness.
#[pyfunction]
fn rs_load(py: Python, fname: String, min_chunk_size: usize) -> PyResult<PyObject> {
    let csr = svmlight::svmlight_to_csr(fname.as_ref(), min_chunk_size);

    // If we just shipped over bytes, which is immutable, rather than
    // a bytearray, we'd have to copy twice: once here for the py-owned
    // object and once in python as numpy would only allow readonly access to
    // bytes-backed numpy buffers.
    let y = PyByteArray::new(py, csr.y.as_byte_slice());
    let data = PyByteArray::new(py, csr.data.as_byte_slice());
    let indices = PyByteArray::new(py, csr.indices.as_byte_slice());
    let indptr = PyByteArray::new(py, csr.indptr.as_byte_slice());

    Ok((y, data, indices, indptr).into_py(py))
}
