import doctest
import cpp_demangle


def load_tests(loader, tests, ignore):
    tests.addTests(doctest.DocTestSuite(cpp_demangle))
    return tests
