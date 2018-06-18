from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(name='cpp-demangle',
      author="Ben Frederickson",
      author_email="ben@benfrederickson.com",
      url='http://github.com/benfred/py-cpp-demangle/',
      description="A package for demangling C++ linker symbols",
      long_description=open("README.rst").read(),
      version="0.0.1",
      rust_extensions=[RustExtension('cpp_demangle', 'Cargo.toml',  binding=Binding.PyO3)],
      test_suite="tests",
      license="MIT",
      classifiers=[
        "Development Status :: 3 - Alpha",
        "Programming Language :: Python :: 3",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Topic :: Software Development :: Libraries",
        "Topic :: Utilities"],
      zip_safe=False)
