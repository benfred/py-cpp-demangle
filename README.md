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

# Unsupported Features

* `dtype` (currently only doubles supported)
* an svmlight ranking mode where query ids are identified with `qid`
* comments in svmlight files (start with `#`)
* empty lines
* lines larger than 16 KiB
* multilabel [extension](https://www.csie.ntu.edu.tw/~cjlin/libsvmtools/datasets/multilabel.html)
* reading from compressed files
* reading from multiple files and stacking
* reading from streams
* writing SVMlight files
* `n_features` option
* `zero_based` option
* windows, mac, and non-Travis-default python wheels

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



