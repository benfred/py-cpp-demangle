
import numpy as np

from svm2csr import load_svmlight_file

def test_load_binding(tmp_path):
    """
    Validates a single simple load case.
    """

    X = load_svmlight_file(str(tmp_path))
    X = np.asarray(X.todense())
    np.testing.assert_array_equal(X, [[0, 1.0, 0, 0], [0, 10.0, 0, 0.1]])

def test_useless():
    pass
