"""
Copyright (c) 2024 Lucas Colley. All rights reserved.

pyo3-array: An experimental Python API for rust-ndarray, following the Python array API standard.
"""

from __future__ import annotations

__version__ = "0.1.0"

__all__ = ["__version__", "eye", "e", "inf", "nan", "newaxis", "pi"]

from ._constants import e, inf, nan, newaxis, pi
from .xp import eye
