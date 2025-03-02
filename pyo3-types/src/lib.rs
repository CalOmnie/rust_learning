use pyo3::prelude::*;
use pyo3::types::PyList;
use std::path::PathBuf;
use std::fs;

#[pyfunction]
fn empty_list(py: Python) -> Option<(String, Bound<'_, PyList>, Bound<'_, PyList>)> {
    let res1 = PyList::empty(py);
    let _ = res1.append(1);
    let res2 = PyList::empty(py);
    let _ = res2.append(1);
    Some((String::from("toto"), res1, res2))
}

#[pyclass]
pub struct DirReader {
    stack: Vec<PathBuf>
}

#[pymethods]
impl DirReader {
    #[new]
    pub fn new(d: String) -> DirReader {
        DirReader {
            stack: vec!(d.into())
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__<'a>(&mut self) -> Option<(String, Vec<String>, Vec<String>)> {
        let target = self.stack.pop()?;
        let dir_reader = fs::read_dir(&target).ok()?;
        let (dirs, files): (_, Vec<_>) = dir_reader
            .into_iter()
            .map(|entry| entry.ok().expect("oops").path())
            .partition(|x| x.is_dir());

        let str_dirs = dirs.iter().map(|x| x.file_name().expect("ok").to_str().expect("ok").to_string()).collect();
        let str_files = files.into_iter().map(|x| x.file_name().expect("ok").to_str().expect("ok").to_string()).collect();

        self.stack.extend(dirs);
        Some((target.into_os_string().into_string().ok()?, str_dirs, str_files))
    }

}


/// A Python module implemented in Rust.
#[pymodule]
fn pyo3_types(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(empty_list, m)?)?;
    m.add_class::<DirReader>()?;
    Ok(())
}
