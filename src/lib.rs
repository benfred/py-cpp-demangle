#![feature(specialization)]

extern crate cpp_demangle;
extern crate pyo3;

use pyo3::py::modinit;
use pyo3::{Python, PyResult, PyModule, exc};

// This defines a python module. pyo3 will copy the rust doc comment
// below into a python docstring

/// A package for demangling C++ linker symbols
///
/// This package provides python bindings for the rust crate
/// [cpp_demangle](http://github.com/gimli-rs/cpp_demangle) by building
/// a native Python extension using [PyO3](https://github.com/pyO3/pyO3)
///
/// Basic usage:
///
/// >>> demangle('_ZN7mangled3fooEd')
/// 'mangled::foo(double)'
///
/// Passing an invalid identifier will throw a ValueError:
///
/// >>> demangle('invalid c++ symbol')
/// Traceback (most recent call last):
/// ...
/// ValueError: mangled symbol is not well-formed
#[modinit(cpp_demangle)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    // This adds a function to the python module:
    /// Demangles a mangled c++ linker symbol name and returns it as a string
    #[pyfn(m, "demangle")]
    fn demangle(mangled: String) -> PyResult<String> {
        match cpp_demangle::Symbol::new(&mangled[..]) {
            // Return the output as a string to Python
            Ok(sym) => Ok(sym.to_string()),

            // on an error, this will raise a python ValueError exception!
            Err(error) => return Err(exc::ValueError::new(error.to_string()))
        }
    }

    Ok(())
}
