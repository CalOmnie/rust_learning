use pyo3::prelude::*;
use std::cmp::Ordering;
use std::io;
use walkdir::WalkDir;
use walkdir::DirEntry;
use walkdir::Result;
use std::sync::Mutex;

#[pyfunction]
fn walk(d: &str) -> i32 {
    let mut res = 0;
    for entry in WalkDir::new(d).into_iter().filter_map(|e| e.ok()) {
        let _unwrapped = entry.path().display();
        res += 1;
    }
    res
}

#[pyclass]
struct PyClassIter {
    count: usize,
}

#[pymethods]
impl PyClassIter {
    #[new]
    fn new(value: usize) -> Self {
        PyClassIter {count: value}
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(&mut self) -> Option<usize> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}


#[pyclass]
struct Iter {
    inner: <WalkDir as IntoIterator>::IntoIter,
}

#[pymethods]
impl Iter {
    #[new]
    fn new(d: String) -> Iter {
        Iter {
            inner: WalkDir::new(d).into_iter()
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<String> {
        let res = slf.inner.next();
        match res {
            Some(Ok(a)) => a.into_path().into_os_string().into_string().ok(),
            Some(Err(a)) => Some(String::from("permission denied")),
            None => None
        }
    }
}

// #[pyclass]
// struct Container {
//     iter: Vec<String>,
// }

// #[pymethods]
// impl Container {
//     fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<Iter>> {
//         let iter = Iter {
//             inner: slf.iter.clone().into_iter(),
//         };
//         Py::new(slf.py(), iter)
//     }
// }

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn hello_cargo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(walk, m)?)?;
    m.add_class::<PyClassIter>()?;
    m.add_class::<Iter>()?;
    Ok(())
}
