py-cpp-demangle: Demangles C++ linker symbols
============================================================

.. image:: https://github.com/benfred/py-cpp-demangle/workflows/Build/badge.svg?branch=master
    :target: https://github.com/benfred/py-cpp-demangle/actions?query=branch%3Amaster

A package for demangling C++ linker symbol strings

This package provides Python bindings for the Rust crate `cpp_demangle
<http://github.com/gimli-rs/cpp_demangle>`_ and for the Rust crate
`msvc_demangler <https://github.com/mstange/msvc-demangler-rust>`_ by building
a native Python extension using `PyO3 <https://github.com/pyO3/pyO3>`_.

This is mainly an experiment in creating Python extensions in Rust.
`A blog post about this is here.
<https://www.benfrederickson.com/writing-python-extensions-in-rust-using-pyo3/>`_

Usage
-------------------

To install

.. code-block:: python

    pip install cpp-demangle


Building from source requires the nightly version of the Rust compiler.

This module exposes a two functions (one for Itanium and one for MSVC) which
transform C++ linker symbols to a human readable representation.

.. code-block:: python

    from cpp_demangle import demangle_itanium

    print(demangle('_ZN7mangled3fooEd'))
    # prints 'mangled::foo(double)'


.. code-block:: python

    from cpp_demangle import demangle_msvc

    print(demangle_msvc('??_0klass@@QEAAHH@Z'))
    # prints 'public: int __cdecl klass::operator/=(int)'

Released under the MIT License
