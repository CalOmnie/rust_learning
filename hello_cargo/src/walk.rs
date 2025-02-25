use pyo3::prelude::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;


#[pyfunction]
fn walk(d: &str) -> i32 {
    let mut res = 0;
    for entry in WalkDir::new("/dev").into_iter().filter_map(|e| e.ok()) {
        let _unwrapped = entry.path().display();
        res += 1;
    }
    res
}


/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn hello_cargo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(walk, m)?)?;

    Ok(())
}
