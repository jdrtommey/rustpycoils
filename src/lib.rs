use pyo3::exceptions::{PyKeyError, PyTypeError};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::PyObjectProtocol;
use pyo3::{PyErr, PyResult};
use std::fmt;

struct AxialErrorWrap {
    error: rustycoils::AxialError,
}
impl std::convert::From<rustycoils::AxialError> for AxialErrorWrap {
    fn from(err: rustycoils::AxialError) -> AxialErrorWrap {
        AxialErrorWrap { error: err }
    }
}
impl std::fmt::Display for AxialErrorWrap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error.to_string())
    }
}
impl std::convert::From<AxialErrorWrap> for PyErr {
    fn from(err: AxialErrorWrap) -> PyErr {
        match err.error {
            rustycoils::AxialError::KeyMissingError(_) => {
                PyKeyError::new_err(err.error.to_string())
            }
            rustycoils::AxialError::KeyDuplicateError(_) => {
                PyKeyError::new_err(err.error.to_string())
            }
            rustycoils::AxialError::IncompatiblePrimitiveError(_, _) => {
                PyTypeError::new_err(err.error.to_string())
            }
            rustycoils::AxialError::ReservedWordError(_) => {
                PyKeyError::new_err(err.error.to_string())
            }
        }
    }
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn rustycoils_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<AxialSystemWrapper>()?;
    Ok(())
}

#[pyclass(name = "AxialSystem", module = "rustycoils_py")]
struct AxialSystemWrapper {
    axialsystem: rustycoils::AxialSystem,
}
#[pymethods]
impl AxialSystemWrapper {
    #[new]
    fn new() -> Self {
        AxialSystemWrapper {
            axialsystem: rustycoils::AxialSystem::default(),
        }
    }
    pub fn transform_x(&mut self) -> PyResult<()> {
        self.axialsystem.transform_x();
        Ok(())
    }
    pub fn transform_y(&mut self) -> PyResult<()> {
        self.axialsystem.transform_y();
        Ok(())
    }
    pub fn transform_z(&mut self) -> PyResult<()> {
        self.axialsystem.transform_z();
        Ok(())
    }
    pub fn add_loop(
        &mut self,
        id: String,
        radius: f64,
        position: f64,
        current: f64,
    ) -> PyResult<()> {
        let res = match self.axialsystem.add_loop(id, radius, position, current) {
            Ok(_) => Ok(()),
            Err(e) => Err(AxialErrorWrap::from(e)),
        };
        Ok(res?)
    }
    pub fn add_annular(
        &mut self,
        id: String,
        radius: f64,
        thickness: f64,
        position: f64,
        current: f64,
    ) -> PyResult<()> {
        let res = match self
            .axialsystem
            .add_annular(id, radius, thickness, position, current)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(AxialErrorWrap::from(e)),
        };
        Ok(res?)
    }
    pub fn add_solenoid(
        &mut self,
        id: String,
        radius: f64,
        length: f64,
        position: f64,
        current: f64,
    ) -> PyResult<()> {
        let res = match self
            .axialsystem
            .add_thin_solenoid(id, radius, length, position, current)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(AxialErrorWrap::from(e)),
        };
        Ok(res?)
    }
    pub fn add_coil(
        &mut self,
        id: String,
        radius: f64,
        thickness: f64,
        length: f64,
        position: f64,
        current: f64,
    ) -> PyResult<()> {
        let res = match self
            .axialsystem
            .add_coil_solenoid(id, radius, thickness, length, position, current)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(AxialErrorWrap::from(e)),
        };
        Ok(res?)
    }
    pub fn modify_radius(&mut self, id: &str, radius: f64) -> PyResult<()> {
        let res = match self.axialsystem.modify_radius(id, radius) {
            Ok(_) => Ok(()),
            Err(e) => Err(AxialErrorWrap::from(e)),
        };
        Ok(res?)
    }
    pub fn modify_current(&mut self, id: &str, current: f64) -> PyResult<()> {
        let res = match self.axialsystem.modify_current(id, current) {
            Ok(_) => Ok(()),
            Err(e) => Err(AxialErrorWrap::from(e)),
        };
        Ok(res?)
    }
    pub fn modify_thickness(&mut self, id: &str, thickness: f64) -> PyResult<()> {
        let res = match self.axialsystem.modify_thickness(id, thickness) {
            Ok(_) => Ok(()),
            Err(e) => Err(AxialErrorWrap::from(e)),
        };
        Ok(res?)
    }
    pub fn modify_length(&mut self, id: &str, length: f64) -> PyResult<()> {
        let res = match self.axialsystem.modify_length(id, length) {
            Ok(_) => Ok(()),
            Err(e) => Err(AxialErrorWrap::from(e)),
        };
        Ok(res?)
    }
    pub fn modify_position(&mut self, id: &str, position: f64) -> PyResult<()> {
        let res = match self.axialsystem.modify_position(id, position) {
            Ok(_) => Ok(()),
            Err(e) => Err(AxialErrorWrap::from(e)),
        };
        Ok(res?)
    }
    pub fn get_field(&self, coordinates: (f64, f64, f64), tol: f64) -> PyResult<Vec<f64>> {
        let (x, y, z) = self.axialsystem.get_field(&coordinates, &tol);
        Ok(vec![x, y, z])
    }
    pub fn get_field_axial(&mut self, z: f64, r: f64, tol: f64) -> PyResult<Vec<f64>> {
        let (z, r) = self.axialsystem.get_field_axial(&z, &r, &tol);
        Ok(vec![z, r])
    }

    pub fn view(&self, id: &str) -> PyResult<String> {
        //  let foo = self.axialsystem.view(id);

        //    Ok("dj".to_string())
        let i = self.axialsystem.view(id);
        let res = match i {
            Ok(string) => Ok(string),
            Err(e) => Err(AxialErrorWrap::from(e)),
        };
        Ok(res?)
    }
}

#[pyproto]
impl PyObjectProtocol for AxialSystemWrapper {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.axialsystem))
    }
}

#[cfg(test)]
mod tests {}
