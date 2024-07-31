pub(crate) mod camera;
pub(crate) mod capabilities;
pub(crate) mod capi;
pub(crate) mod components;
pub(crate) mod core_properties;
pub(crate) mod device;
pub(crate) mod device_manager;
pub(crate) mod runtime;
pub(crate) mod storage;

use anyhow::{anyhow, Result};
use device_manager::DeviceManager;
use pyo3::prelude::*;
use std::ffi::CStr;

use crate::runtime::{
    AvailableData, AvailableDataContext, VideoFrame, VideoFrameMetadata, VideoFrameTimestamps,
};

trait Status: Copy + Sized {
    fn is_ok(&self) -> bool;

    fn ok(&self) -> Result<Self> {
        if self.is_ok() {
            Ok(*self)
        } else {
            Err(anyhow!("Failed acquire api status check")) // TODO: (nclack) adapt to thiserror maybe, need to know which api called, params etc
        }
    }
}

impl Status for capi::DeviceStatusCode {
    fn is_ok(&self) -> bool {
        *self == capi::AcquireStatusCode_AcquireStatus_Ok
    }
}

impl Status for bool {
    fn is_ok(&self) -> bool {
        *self
    }
}

#[pyfunction]
fn core_api_version() -> PyResult<String> {
    let ptr = unsafe { capi::acquire_api_version_string() };
    Ok(unsafe { CStr::from_ptr(ptr) }.to_str()?.to_owned())
}

#[pymodule]
fn acquire(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::Logger::new(py, pyo3_log::Caching::LoggersAndLevels)?
        .filter(log::LevelFilter::Trace)
        .install()
        .expect("Failed to init logger");
    log::trace!("Log: trace enabled");
    log::debug!("Log: debug enabled");
    log::info!("Log: info enabled");
    log::warn!("Log: warn enabled");
    log::error!("Log: error enabled");

    m.add_class::<runtime::Runtime>()?;
    m.add_class::<DeviceManager>()?;
    m.add_class::<AvailableData>()?;
    m.add_class::<AvailableDataContext>()?;
    m.add_class::<VideoFrame>()?;
    m.add_class::<VideoFrameMetadata>()?;
    m.add_class::<VideoFrameTimestamps>()?;

    m.add_class::<core_properties::Properties>()?;
    m.add_class::<core_properties::VideoStream>()?;
    m.add_class::<core_properties::Camera>()?;
    m.add_class::<core_properties::Storage>()?;

    m.add_class::<capabilities::Capabilities>()?;
    m.add_class::<capabilities::VideoStreamCapabilities>()?;
    m.add_class::<camera::CameraCapabilities>()?;
    m.add_class::<camera::DigitalLineCapabilities>()?;
    m.add_class::<camera::TriggerCapabilities>()?;
    m.add_class::<camera::TriggerInputOutputCapabilities>()?;
    m.add_class::<camera::OffsetCapabilities>()?;
    m.add_class::<camera::ShapeCapabilities>()?;
    m.add_class::<storage::StorageCapabilities>()?;
    m.add_class::<components::Property>()?;
    m.add_class::<components::PropertyType>()?;

    m.add_class::<device::DeviceIdentifier>()?;
    m.add_class::<device::DeviceState>()?;
    m.add_class::<camera::CameraProperties>()?;
    m.add_class::<camera::InputTriggers>()?;
    m.add_class::<camera::OutputTriggers>()?;
    m.add_class::<storage::DimensionType>()?;
    m.add_class::<storage::StorageDimension>()?;
    m.add_class::<storage::StorageProperties>()?;

    m.add_class::<components::Direction>()?;
    m.add_class::<components::PID>()?;
    m.add_class::<components::SampleRateHz>()?;
    m.add_class::<components::SampleType>()?;
    m.add_class::<components::SignalIOKind>()?;
    m.add_class::<components::SignalType>()?;
    m.add_class::<components::Trigger>()?;
    m.add_class::<components::TriggerEdge>()?;
    m.add_class::<components::VoltageRange>()?;

    m.add_class::<device::DeviceKind>()?;

    m.add_function(wrap_pyfunction!(core_api_version, m)?)?;
    Ok(())
}

// TODO: consider replacing anyhow with thiserror/eyre for errors
