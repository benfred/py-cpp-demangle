import numpy as np
from sklearn.datasets import dump_svmlight_file, load_svmlight_file as skload

from svm2csr import load_svmlight_file as myload

def eval_custom(tmp_path, contents):
    fn = tmp_path / 'testfile'
    with open(fn, 'w') as f:
        print(contents, file=f, end='')

    fn = str(fn)
    expected_X, expected_y = skload(fn)
    actual_X, actual_y = myload(fn)

    assert expected_y.dtype == actual_y.dtype
    assert expected_X.dtype == actual_X.dtype

    np.testing.assert_array_equal(actual_y, expected_y)
    expected_X.sort_indices()
    actual_X.sort_indices()
    for attr in ['data', 'indices', 'indptr']:
        np.testing.assert_array_equal(getattr(expected_X, attr), getattr(actual_X, attr), err_msg=attr)

def test_empty_file(tmp_path):
    eval_custom(tmp_path, "")

def test_one_line_file(tmp_path):
    eval_custom(tmp_path, "1.5 3:0 5:0.0 32:2.1 341:321\n")

def test_two_line_file(tmp_path):
    pass # make manually and test

def test_one_line_no_newline_file(tmp_path):
    pass # make manually and test

def test_one_line_no_features_file(tmp_path):
    pass # make manually and test

def test_one_line_no_features_no_newline_file(tmp_path):
    pass # make manually and test

def test_1000_lines(tmp_path):
    # generate randomly over
    # label (-1, 0, +1, randn)
    # features (zero, one, moderate (10), many (10000)) valued also at (-1, 0, +1, 10000)
    # + with guaranteed dups
    # try mix of chunksizes (slightly under/exact/over
    # Full file, slightly under 1/2, 1/2 exactly, slightly over 1/2)
    pass
