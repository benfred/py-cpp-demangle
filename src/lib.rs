#![feature(proc_macro, specialization)]

extern crate cpp_demangle;
extern crate pyo3;

use pyo3::py::modinit;
use pyo3::{Python, PyResult, PyModule, exc};

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
    #[pyfn(m, "demangle")]
    /// Demangles a mangled c++ linker symbol name and returns it as a string
    fn demangle(mangled: String) -> PyResult<String> {
        let sym = match cpp_demangle::Symbol::new(&mangled[..]) {
            Ok(s) => s,
            Err(error) => return Err(exc::ValueError::new(error.to_string()))
        };

        let demangled = sym.to_string();
        Ok(demangled)
    }

    Ok(())
}