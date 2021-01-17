//! SVMLight reader crate with python bindings via PyO3.

mod delim_iter;
mod fileblocks;
mod svmlight;

use byte_slice_cast::AsByteSlice;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// SVMlight loader bindings from Rust.
///
/// Not intended to be called directly. Instead, see :func:`~svm2csr.load_svmlight_file`.
#[pymodule]
fn py_svm2csr_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load, m)?)?;

    Ok(())
}

/// Returns a three-tuple of bytes containing (data, indices, indptr) with types
/// (f8, u8, u8) in native endianness.
#[pyfunction]
fn load(py: Python, fname: String) -> PyResult<PyObject> {
    let min_chunk_size = 16 * 1024;
    let csr = svmlight::svmlight_to_csr(fname.as_ref(), min_chunk_size);

    let data = csr.data.as_byte_slice();
    let indices = csr.indices.as_byte_slice();
    let indptr = csr.indptr.as_byte_slice();

    Ok((data, indices, indptr).into_py(py))
}
