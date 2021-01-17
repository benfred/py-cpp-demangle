"""
SVMlight loading API.
"""

import numpy as np
import scipy.sparse as sps

from .svm2csr import rs_load

def load_svmlight_file(fname, zero_based="auto", min_chunk_size=(16 * 1024)):
    """
    Loads an SVMlight file into a CSR matrix.

    fname (str): the file name of the file to load.
    zero_based ("auto" or bool): whether the corresponding svmlight file uses
        zero based indexing; if false or all indices are nonzero, then
        shifts indices down uniformly by 1 for python's zero indexing.
    min_chunk_size (int): minimum chunk size in bytes per
        parallel processing task
    """
    assert min_chunk_size > 0
    y, data, indices, indptr = rs_load(fname, min_chunk_size)

    y = np.frombuffer(y, dtype=np.float64)
    data = np.frombuffer(data, dtype=np.float64)
    indices = np.frombuffer(indices, dtype=np.uint64)
    indptr = np.frombuffer(indptr, dtype=np.uint64)

    assert indptr.size
    assert len(y) == len(indptr) - 1

    if not indices.size:
        return sps.csr_matrix((len(indptr) - 1, 0), dtype=np.float64), y

    if not zero_based or (zero_based == "auto" and indices.min() > 0):
        indices -= 1

    return sps.csr_matrix((data, indices, indptr)), y
