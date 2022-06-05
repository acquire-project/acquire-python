

pub(crate) mod core_runtime;
pub(crate) mod device_manager;
pub(crate) mod components;
pub(crate) mod camera;
pub mod runtime;

use std::ffi::CStr;
use pyo3::prelude::*;
use anyhow::{anyhow,Result};

trait Status: Sized {
    fn ok(&self) -> Result<Self>;
}

impl Status for core_runtime::DeviceStatusCode {
    fn ok(&self) -> Result<Self> {
        if *self == core_runtime::CoreStatusCode_CoreStatus_Ok {
            Ok(*self)
        } else {
            Err(anyhow!("Failed status check"))
        }
    }
}

#[pyfunction]
fn core_api_version() -> PyResult<String> {
    let ptr = unsafe { core_runtime::core_api_version_string() };
    Ok(unsafe { CStr::from_ptr(ptr) }.to_str()?.to_owned())
}

#[pymodule]
fn demo_python_api(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<runtime::Runtime>()?;
    m.add_class::<camera::SampleType>()?;
    m.add_class::<camera::CameraProperties>()?;
    m.add_function(wrap_pyfunction!(core_api_version, m)?)?;
    Ok(())
}
