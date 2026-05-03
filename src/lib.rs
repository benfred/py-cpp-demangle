use pyo3::exceptions;
use pyo3::prelude::*;

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
/// >>> demangle_itanium('_ZN7mangled3fooEd')
/// 'mangled::foo(double)'
///
/// >>> demangle_msvc('??_0klass@@QEAAHH@Z')
/// 'public: int __cdecl klass::operator/=(int)'
///
/// Passing an invalid identifier will throw a ValueError:
///
/// >>> demangle_itanium('invalid c++ symbol')
/// Traceback (most recent call last):
/// ...
/// ValueError: ('Could not demangle symbol', 'mangled symbol is not well-formed')
///
/// >>> demangle_msvc('invalid C++ symbol')
/// Traceback (most recent call last):
/// ...
/// ValueError: ('Could not format demangled name as string', 'does not start with b\'?\' (offset: 0, remaining: "invalid C++ symbol")')
#[pymodule]
fn cpp_demangle(_py: Python, m: &PyModule) -> PyResult<()> {
    // This adds a function to the python module:
    /// Demangles a mangled Itanium ABI C++ linker symbol name and returns it as a string
    #[pyfn(m)]
    fn demangle_itanium(mangled: String) -> PyResult<String> {
        let symbol = ::cpp_demangle::Symbol::new(&mangled[..]).map_err(|error| {
            exceptions::PyValueError::new_err(("Could not demangle symbol", error.to_string()))
        })?;
        let demangled = symbol.demangle(&Default::default()).map_err(|error| {
            exceptions::PyValueError::new_err((
                "Could not format demangled name as string",
                error.to_string(),
            ))
        })?;

        Ok(demangled)
    }

    /// Demangles a mangled MSVC C++ linker symbol name and returns it as a string
    #[pyfn(m)]
    fn demangle_msvc(mangled: String) -> PyResult<String> {
        let flags = ::msvc_demangler::DemangleFlags::llvm();
        let demangled = ::msvc_demangler::demangle(&mangled[..], flags).map_err(|error| {
            exceptions::PyValueError::new_err((
                "Could not format demangled name as string",
                error.to_string(),
            ))
        })?;

        Ok(demangled)
    }

    Ok(())
}
