use numpy::ndarray::Array;
use numpy::{PyArray2, ToPyArray};
use pyo3::exceptions::PyNotImplementedError;
use pyo3::prelude::*;

mod _statistical_functions;
use _statistical_functions::std;


#[pyfunction]
#[pyo3(signature = (n_rows, n_cols=None, /, *, k=0, dtype=None, device=None))]
fn eye(
    py: Python,
    n_rows: usize,
    n_cols: Option<usize>,
    k: usize,
    dtype: Option<String>,
    device: Option<String>,
) -> PyResult<Py<PyArray2<f64>>> {
    if n_cols.is_some() || k != 0 || dtype.is_some() || device.is_some() {
        let message = "Only the `n_rows` parameter is implemented for `eye`.";
        Err(PyNotImplementedError::new_err(message))
    } else {
        Ok(Array::eye(n_rows).to_pyarray_bound(py).unbind())
    }
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn xp(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // functions
    m.add_function(wrap_pyfunction!(eye, m)?)?;
    // statistical functions
    m.add_function(wrap_pyfunction!(std, m)?)?;
    // version
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
