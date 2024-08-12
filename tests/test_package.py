from __future__ import annotations

import importlib.metadata

import pyo3_array as m


def test_version():
    assert importlib.metadata.version("pyo3_array") == m.__version__
