# `svm2csr`: convert svmlight files into CSR representation

Many sparse datasets are distributed in a lightweight text format called [svmlight](http://svmlight.joachims.org/). While simple and familiar, it's terribly slow to read in python even with C++ solutions. Note this is Python 3.5+.

```
from sklearn.datasets import load_svmlight_file
%timeit load_svmlight_file('kdda')
# ...

# https://github.com/mblondel/svmlight-loader
# beware this is a pain to install
from svmlight_loader import load
%timeit load()
# ...

from svm2csr import load
%timeit load()
# ...
```

# Install

```
pip install svm2csr
```

Note this package is only available for pythons, operating systems, and machine architecture targets I can build wheels for. Right now, that makes it linux-only.

* `cp36-cp39, manylinux2010, x86_64`

# Unsupported Features

* `dtype` (currently only doubles supported)
* an svmlight ranking mode where query ids are identified with `qid`
* comments in svmlight files (start with `#`)
* empty lines
* multilabel [extension](https://www.csie.ntu.edu.tw/~cjlin/libsvmtools/datasets/multilabel.html)
* reading from compressed files
* reading from multiple files and stacking
* reading from streams
* writing SVMlight files
* `n_features` option
* `zero_based` option
* graceful client `multiprocessing`
* mac and windows wheels

All of these are fixable (even stream reading with parallel bridge). Let me know if you'd like to make PR.

# Documentation

TODO

# Dev Info

Install maturin and pytest first.

```
pip install maturin pytest
```

Local development.

```
cargo test # test rust only
maturin develop # create py bindings for rust code
pytest # test python bindings
```

[![travis build](https://travis-ci.org/vlad17/svm2csr.svg?branch=master](https://travis-ci.org/vlad17/svm2csr)

# Publishing

Maturin doesn't prepare a `setup.py` when publishing. For this reason, a source distribution doesn't make sense, as a client machine's `pip` would not know how to install this package. For this reason, only wheels are published.

A new set of wheels can be built and published for supported OSes and pythons with the following steps for a repository administrator:

1. Fetch the most recent master.
1. Bump the version in `Cargo.toml` appropriately if needed (else wheel names will clash with previous ones in pypi, though PRs should be bumping this already). Commit these changes.
1. Tag the release. `git tag -a -m "v<CURRENT VERSION>"`
1. Push to github, triggering a Travis build that tests, packages, and uploads to pypi. `git push --follow-tags`
