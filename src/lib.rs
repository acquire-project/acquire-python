pub(crate) mod camera;
pub(crate) mod capi;
pub(crate) mod components;
pub(crate) mod core_properties;
pub(crate) mod device;
pub(crate) mod device_manager;
pub(crate) mod runtime;
pub(crate) mod signals;
pub(crate) mod stage_axis;
pub(crate) mod storage;

use anyhow::{anyhow, Result};
use device_manager::DeviceManager;
use pyo3::prelude::*;
use std::ffi::CStr;

trait Status: Copy + Sized {
    fn is_ok(&self) -> bool;

    fn ok(&self) -> Result<Self> {
        if self.is_ok() {
            Ok(*self)
        } else {
            Err(anyhow!("Failed cpx api status check")) // TODO: (nclack) adapt to thiserror maybe, need to know which api called, params etc
        }
    }
}

impl Status for capi::DeviceStatusCode {
    fn is_ok(&self) -> bool {
        *self == capi::CpxStatusCode_CpxStatus_Ok
    }
}

#[pyfunction]
fn core_api_version() -> PyResult<String> {
    let ptr = unsafe { capi::cpx_api_version_string() };
    Ok(unsafe { CStr::from_ptr(ptr) }.to_str()?.to_owned())
}

#[pymodule]
fn calliphlox(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::Logger::new(py, pyo3_log::Caching::LoggersAndLevels)?
        .filter(log::LevelFilter::Debug)
        .install()
        .expect("Failed to init logger");

    m.add_class::<runtime::Runtime>()?;
    m.add_class::<DeviceManager>()?;
    m.add_class::<core_properties::Properties>()?;
    m.add_class::<core_properties::Camera>()?;
    m.add_class::<core_properties::Signals>()?;
    m.add_class::<core_properties::StageAxis>()?;
    m.add_class::<core_properties::Storage>()?;

    m.add_class::<device::DeviceIdentifier>()?;
    m.add_class::<camera::CameraProperties>()?;
    m.add_class::<signals::SignalProperties>()?;
    m.add_class::<storage::StorageProperties>()?;
    m.add_class::<stage_axis::StageAxisProperties>()?;
    m.add_class::<stage_axis::StageAxisState>()?;
    m.add_class::<signals::SignalProperties>()?;

    m.add_class::<components::Channel>()?;
    m.add_class::<components::PID>()?;
    m.add_class::<components::SampleRateHz>()?;
    m.add_class::<components::SampleType>()?;
    m.add_class::<components::SignalIOKind>()?;
    m.add_class::<components::SignalType>()?;
    m.add_class::<components::Timing>()?;
    m.add_class::<components::Trigger>()?;
    m.add_class::<components::TriggerEdge>()?;
    m.add_class::<components::TriggerEvent>()?;
    m.add_class::<components::VoltageRange>()?;

    m.add_class::<device::DeviceKind>()?;

    m.add_function(wrap_pyfunction!(core_api_version, m)?)?;
    Ok(())
}

// TODO: consider replacing anyhow with thiserror/eyre for errors
