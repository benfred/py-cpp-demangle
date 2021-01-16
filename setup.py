from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

extras = {}
extras["testing"] = ["pytest"]

setup(name='svm2csr',
      author="Vladimir Feinberg",
      author_email="vladimir.feinberg@gmail.com",
      url='https://github.com/vlad17/svm2csr',
      description="Convert SVMlight text files to scipy CSR matrices",
      long_description=open("README.md").read(),
      long_description_content_type="text/markdown",
      extras_require=extras,
      version="0.0.1",
      # TODO debug=false
      rust_extensions=[RustExtension('py_svm2cs4_rs', 'Cargo.toml',  binding=Binding.PyO3)],
      test_suite="tests",
      packages=find_packages(),
      license="Apache Software License",
      classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: Apache Software License",
        "Topic :: Software Development :: Libraries",
        "Topic :: Utilities"],
      zip_safe=False)
