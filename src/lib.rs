
pub(crate) mod core_runtime;
pub(crate) mod device_manager;
pub(crate) mod components;
pub(crate) mod camera;
pub(crate) mod storage;
pub(crate) mod stage_axis;
pub(crate) mod signals;
pub(crate) mod core_properties;
pub(crate) mod device;
pub(crate) mod runtime;


use std::ffi::CStr;
use pyo3::prelude::*;
use anyhow::{anyhow,Result};

trait Status: Copy+Sized {
    fn is_ok(&self) -> bool;

    fn ok(&self) -> Result<Self> {
        if self.is_ok() {
            Ok(*self)
        } else {
            Err(anyhow!("Failed core_runtime api status check"))
        }
    }

}

impl Status for core_runtime::DeviceStatusCode {
    fn is_ok(&self) -> bool {
        *self == core_runtime::CpxStatusCode_CpxStatus_Ok
    }
}

#[pyfunction]
fn core_api_version() -> PyResult<String> {
    let ptr = unsafe { core_runtime::cpx_api_version_string() };
    Ok(unsafe { CStr::from_ptr(ptr) }.to_str()?.to_owned())
}

#[pymodule]
fn demo_python_api(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<runtime::Runtime>()?;
    m.add_class::<core_properties::CoreProperties>()?;

    m.add_class::<camera::CameraProperties>()?;
    m.add_class::<storage::StorageProperties>()?;
    m.add_class::<stage_axis::StageAxisProperties>()?;
    m.add_class::<stage_axis::StageAxisState>()?;
    m.add_class::<signals::SignalProperties>()?;
    m.add_class::<signals::Channel>()?;

    m.add_class::<components::PID>()?;
    m.add_class::<components::SampleRateHz>()?;
    m.add_class::<components::SampleType>()?;
    m.add_class::<components::SignalIOKind>()?;
    m.add_class::<components::SignalType>()?;
    m.add_class::<components::Trigger>()?;
    m.add_class::<components::TriggerEdge>()?;
    m.add_class::<components::TriggerEvent>()?;
    m.add_class::<components::VoltageRange>()?;

    m.add_class::<device::DeviceKind>()?;

    m.add_function(wrap_pyfunction!(core_api_version, m)?)?;
    Ok(())
}

// TODO: consider replacing anyhow with thiserror for errors