use numpy::ndarray::{arr0};
use numpy::{PyUntypedArray, PyReadwriteArrayDyn, ToPyArray};
use pyo3::prelude::*;

#[derive(FromPyObject)]
enum TypedArray<'py> {
    F64Array(PyReadwriteArrayDyn<'py, f64>),
    F32Array(PyReadwriteArrayDyn<'py, f32>),
}

#[pyfunction]
// XXX: keepdims=False
#[pyo3(signature = (x, /, *, _axis=None, _correction=0.0, _keepdims=None))]
pub fn std<'py>(
    py: Python<'py>,
    x: Bound<'py, PyUntypedArray>,
    _axis: Option<String>,
    _correction: f64,
    _keepdims: Option<bool>,
) -> PyResult<Bound<'py, PyAny>> {
    let res = match x.extract::<TypedArray>()? {
        TypedArray::F64Array(x) => arr0(x.as_array().std(0.0)).to_pyarray_bound(py).into_any(),
        TypedArray::F32Array(x) => arr0(x.as_array().std(0.0)).to_pyarray_bound(py).into_any(),
    };

    Ok(res)
}
