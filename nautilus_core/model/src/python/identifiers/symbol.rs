// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2024 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

use std::{
    collections::hash_map::DefaultHasher,
    ffi::CString,
    hash::{Hash, Hasher},
    str::FromStr,
};

use nautilus_core::{correctness::check_in_range_inclusive_usize, python::to_pyvalue_err};
use pyo3::{
    exceptions::PyValueError,
    prelude::*,
    pyclass::CompareOp,
    types::{PyString, PyTuple},
};

use crate::identifiers::symbol::Symbol;

#[pymethods]
impl Symbol {
    #[new]
    fn py_new(value: &str) -> PyResult<Self> {
        Self::new_checked(value).map_err(to_pyvalue_err)
    }

    #[staticmethod]
    fn _safe_constructor() -> Self {
        Self::from("NULL")
    }

    fn __setstate__(&mut self, py: Python, state: PyObject) -> PyResult<()> {
        let value: (&PyString,) = state.extract(py)?;
        let value: &str = value.0.extract()?;
        self.set_inner(value);
        Ok(())
    }

    fn __getstate__(&self, py: Python) -> PyResult<PyObject> {
        Ok((self.to_string(),).to_object(py))
    }

    fn __reduce__(&self, py: Python) -> PyResult<PyObject> {
        let safe_constructor = py.get_type::<Self>().getattr("_safe_constructor")?;
        let state = self.__getstate__(py)?;
        Ok((safe_constructor, PyTuple::empty(py), state).to_object(py))
    }

    fn __richcmp__(&self, other: PyObject, op: CompareOp, py: Python<'_>) -> Py<PyAny> {
        if let Ok(other) = other.extract::<Self>(py) {
            match op {
                CompareOp::Eq => self.eq(&other).into_py(py),
                CompareOp::Ne => self.ne(&other).into_py(py),
                _ => py.NotImplemented(),
            }
        } else {
            py.NotImplemented()
        }
    }

    fn __hash__(&self) -> isize {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish() as isize
    }

    fn __repr__(&self) -> String {
        format!("{}('{}')", stringify!(Symbol), self)
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    #[staticmethod]
    #[pyo3(name = "from_str")]
    fn py_from_str(value: &str) -> PyResult<Self> {
        Self::new_checked(value).map_err(to_pyvalue_err)
    }

    #[getter]
    #[pyo3(name = "value")]
    fn py_value(&self) -> String {
        self.to_string()
    }

    #[getter]
    #[pyo3(name = "is_composite")]
    fn py_is_composite(&self) -> bool {
        self.is_composite()
    }

    #[getter]
    #[pyo3(name = "root")]
    fn py_root(&self) -> &str {
        self.root()
    }

    #[getter]
    #[pyo3(name = "topic")]
    fn py_topic(&self) -> String {
        self.topic()
    }
}

impl ToPyObject for Symbol {
    fn to_object(&self, py: Python) -> PyObject {
        self.into_py(py)
    }
}
