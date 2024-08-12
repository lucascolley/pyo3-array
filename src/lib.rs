use numpy::{PyArray2, ToPyArray};
use numpy::ndarray::Array;
use pyo3::prelude::*;


#[pyfunction]
fn eye(n_rows: usize, py: Python) -> Py<PyArray2<f64>> {
    Array::eye(n_rows).to_pyarray(py).to_owned()
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn xp(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(eye, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
