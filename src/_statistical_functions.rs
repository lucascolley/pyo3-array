use numpy::ndarray::{arr0};
use numpy::{PyUntypedArray, PyReadwriteArrayDyn, ToPyArray};
use pyo3::prelude::*;

#[pyfunction]
// XXX: keepdims=False
#[pyo3(signature = (x, /, *, _axis=None, correction=0.0, _keepdims=None))]
pub fn std<'py>(
    py: Python<'py>,
    x: Bound<'py, PyUntypedArray>,
    _axis: Option<String>,
    correction: f64,
    _keepdims: Option<bool>,
) -> PyResult<Bound<'py, PyAny>> {
    // let dtype = x.dtype();
    let x_borrowed = x.extract::<PyReadwriteArrayDyn<f64>>()?;
    let res = arr0(x_borrowed.as_array().std(correction));
    let res_py = res.to_pyarray_bound(py).into_any();

    Ok(res_py)
}
