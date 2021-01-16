"""
SVMlight loading API.
"""

import numpy as np
import scipy.sparse as sps

import py_svm2csr_rs

def load_svmlight_file(fname):
    """
    Loads an SVMlight file into a CSR matrix.

    fname (str): the file name of the file to load.
    """
    data, indices, indptr = py_svm2csr_rs.load(fname)
    print(data, indices, indptr)

    data = np.frombuffer(data, dtype=np.float64)
    indices = np.frombuffer(indices, dtype=np.uint64)
    indptr = np.frombuffer(indptr, dtype=np.uint64)

    return sps.csr_matrix((data, indices, indptr))
