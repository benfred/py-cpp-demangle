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
fn load(py: Python, _fname: String) -> PyResult<PyObject> {
    let data = vec![1.0f64, 10.0f64, 0.1f64];
    let indices = vec![1u64, 1u64, 3u64];
    let indptr = vec![0u64, 1, 3];

    let data = data.as_byte_slice();
    let indices = indices.as_byte_slice();
    let indptr = indptr.as_byte_slice();

    Ok((data, indices, indptr).into_py(py))
}
