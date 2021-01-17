import numpy as np
import scipy.sparse as sps
from sklearn.datasets import dump_svmlight_file, load_svmlight_file as skload
import pytest

from svm2csr import load_svmlight_file as myload

def eval_custom(tmp_path, contents, chunk_sizes):
    fn = tmp_path / 'testfile'
    with open(fn, 'w') as f:
        print(contents, file=f, end='')

    fn = str(fn)
    check_file_loaded_equally(fn, chunk_sizes)

def check_file_loaded_equally(fn, chunk_sizes):
    expected_X, expected_y = skload(fn)
    for min_chunk in chunk_sizes:
        kwargs = {'min_chunk_size': min_chunk} if min_chunk is not None else {}
        actual_X, actual_y = myload(fn, **kwargs)

        assert expected_y.dtype == actual_y.dtype
        assert expected_X.dtype == actual_X.dtype

        np.testing.assert_array_equal(actual_y, expected_y)
        expected_X.sort_indices()
        actual_X.sort_indices()
        for attr in ['data', 'indices', 'indptr']:
            np.testing.assert_array_equal(getattr(expected_X, attr), getattr(actual_X, attr), err_msg=attr)

CASES = {
    'empty': '',
    'one line': '1.5 3:0 5:0.0 32:2.1 341:321\n',
    'two line':
    '-1.5 3:0 5:0.0 32:2.1 341:321\n' +
    '-0.0 5:2 10001:-0.001\n',
    'one line no features': '1.5\n',
    'two line no features':
    '-1.5    \n' +
    '-0.0\n'
}

for k in list(CASES):
    no_newline = CASES[k].strip()
    if no_newline != CASES[k]:
        CASES[k + ' no final newline'] = no_newline


@pytest.mark.parametrize("case", list(CASES))
def test_const_file(tmp_path, case):
    chunk_sizes = [1, 2, 3, 5, 10, 100, None]
    eval_custom(tmp_path, CASES[case], chunk_sizes)

def weird_floats(n):
    y = np.random.choice([-1.0, 0.0, 1.0, 2.0], size=n)
    y[y == 2.0] = np.random.randn(np.sum(y == 2.0))
    return y

def test_1000_lines(tmp_path):
    np.random.seed(1234)

    n = 1000
    y = weird_floats(n)

    d = 100 * 1000
    row_sizes = [0, 1, 2, 10, 10 * 1000]
    row_p = [0.2, 0.1, 0.25, 0.25, 0.2]
    sampled_row_sizes = np.random.choice(row_sizes, size=n, p=row_p)
    rows = []
    cols = []
    for i, rowlen in enumerate(sampled_row_sizes):
        cols.append(np.random.choice(d, size=rowlen, replace=False))
        cols[-1].sort()
        rows.append([i] * rowlen)
    rows = np.concatenate(rows).astype(int)
    cols = np.concatenate(cols)
    vals = weird_floats(len(cols))
    vals[vals == 0.0] = -2.0

    X = sps.coo_matrix((vals, (rows, cols)))

    path = tmp_path / "svmfile"
    fn = str(path)
    dump_svmlight_file(X, y, fn)

    sz = path.stat().st_size

    chunk_sizes = [sz // 2 - 1, sz // 2, sz // 2 + 1, sz - 1, sz, sz + 1, None]

    check_file_loaded_equally(fn, chunk_sizes)
