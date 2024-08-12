use pyo3::prelude::*;
use pyo3::types::PyNone;
pub use std::f64::consts::E as e;
pub use std::f64::consts::PI as pi;
pub use std::f64::INFINITY as inf;
pub use std::f64::NAN as nan;

pub fn newaxis(py: Python) -> &PyNone {
    PyNone::get(py)
}
