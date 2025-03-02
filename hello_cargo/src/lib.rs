use pyo3::prelude::*;
use std::cmp::Ordering;
use std::io;
use std::sync::Mutex;
use std::fs;
use std::path::PathBuf;

struct DirReader {
    stack: Vec<PathBuf>
}

impl DirReader {
    fn new(d: String) -> DirReader {
        DirReader {
            stack: vec!(d.into())
        }
    }
}

impl Iterator for DirReader {
    type Item = (String, Vec<String>, Vec<String>);

    fn next(&mut self) -> Option<Self::Item> {
        let mut target: PathBuf;
        let dir_reader = loop {
            target = self.stack.pop()?;
            match fs::read_dir(&target) {
                Ok(reader) => break reader,
                Err(_) => (),
            }
        };
        let mut dirs = Vec::<String>::new();
        let mut files = Vec::<String>::new();

        for entry in dir_reader.filter_map(|e| e.ok()) {
            let filetype = entry.file_type().ok()?;
            let file_name = entry.file_name().into_string().ok()?;
            let path = entry.path();
            if filetype.is_dir() {
                self.stack.push(path);
                dirs.push(file_name);
            } else {
                files.push(file_name)
            }
        }
        Some((target.into_os_string().into_string().ok()?, dirs, files))
    }
}

#[pyclass]
struct PyDirReader {
    inner: DirReader
}


#[pymethods]
impl PyDirReader {
    #[new]
    fn new(d: String) -> PyDirReader {
        PyDirReader {
            inner: DirReader::new(d)
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<(String, Vec<String>, Vec<String>)> {
        slf.inner.next()
    }
}




#[pyfunction]
fn read_dir(d: &str) -> io::Result<(String, Vec<String>, Vec<String>)> {
    let mut dirs = Vec::<String>::new();
    let mut files = Vec::<String>::new();
    let root = String::from(d);

    for entry in fs::read_dir(d)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name().into_string().expect("Oops");
        if path.is_dir() {
            dirs.push(file_name)
        } else {
            files.push(file_name)
        }
    }
    Ok((root, dirs, files))
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

use std::iter::FilterMap;
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

// A Python module implemented in Rust. The name of this function must match
// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to

#[pymodule]
fn hello_cargo(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_dir, m)?)?;
    m.add_class::<PyClassIter>()?;
    m.add_class::<PyDirReader>()?;
    Ok(())
}
