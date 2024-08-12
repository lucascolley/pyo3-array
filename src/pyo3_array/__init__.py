"""
Copyright (c) 2024 Lucas Colley. All rights reserved.

pyo3-array: An experimental Python API for rust-ndarray, following the Python array API standard.
"""

from __future__ import annotations

__version__ = "0.1.0"

__all__ = ["__version__", "eye"]

from .xp import eye  # type: ignore[import-not-found]
