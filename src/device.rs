use pyo3::prelude::*;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::ffi::CStr;

use crate::{capi, components::macros::cvt};
use anyhow::{anyhow, Result};

impl capi::DeviceIdentifier {
    pub(crate) fn name_as_string(&self) -> Result<String> {
        Ok(unsafe { CStr::from_ptr(self.name.as_ptr()) }
            .to_str()?
            .to_owned())
    }
}

#[pyclass]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeviceKind {
    Camera,
    Storage,
    StageAxis,
    Signals,
}

cvt!(DeviceKind=>capi::DeviceKind,
    Camera    => DeviceKind_DeviceKind_Camera,
    Storage   => DeviceKind_DeviceKind_Storage,
    StageAxis => DeviceKind_DeviceKind_StageAxis,
    Signals   => DeviceKind_DeviceKind_Signals
);

// TODO: (nclack) Allow DeviceManager::select to accept strings for device kind
// TODO: (nclack) Is there a way to automatically extend python's usual string to enum conversion?
impl TryFrom<&str> for DeviceKind {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Camera" => Ok(DeviceKind::Camera),
            "Storage" => Ok(DeviceKind::Storage),
            "StageAxis" => Ok(DeviceKind::StageAxis),
            "Signals" => Ok(DeviceKind::Signals),
            _ => Err(anyhow!("Did not recognize {} as a valid DeviceKind", value)),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub(crate) struct DeviceIdentifier {
    #[pyo3(get)]
    id: (u8, u8),

    #[pyo3(get)]
    kind: DeviceKind,

    #[pyo3(get)]
    name: String,
}

impl Serialize for DeviceIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DeviceIdentifier", 2)?;
        state.serialize_field("kind", &self.kind)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

#[pymethods]
impl DeviceIdentifier {
    fn __repr__(&self) -> String {
        format!("<DeviceIdentifier {:?} \"{}\">", self.kind, self.name)
    }

    fn dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(pythonize::pythonize(py, self)?)
    }
}

impl TryFrom<capi::DeviceIdentifier> for DeviceIdentifier {
    type Error = anyhow::Error;

    fn try_from(value: capi::DeviceIdentifier) -> Result<Self, Self::Error> {
        if value.name[0] == 0 {
            Err(anyhow!("Invalid device identifier (empty name)."))
        } else {
            Ok(Self {
                id: (value.driver_id, value.device_id),
                kind: value.kind.try_into()?,
                name: value.name_as_string()?,
            })
        }
    }
}

impl TryFrom<&DeviceIdentifier> for capi::DeviceIdentifier {
    type Error = anyhow::Error;

    fn try_from(value: &DeviceIdentifier) -> Result<Self, Self::Error> {
        let mut out = Self {
            driver_id: value.id.0,
            device_id: value.id.1,
            kind: value.kind.into(),
            name: [0; 256],
        };
        if value.name.as_bytes().len() >= out.name.len() - 1 {
            Err(anyhow!(
                "Device name is too long. Names are limited to {} bytes. Got {}.",
                out.name.len() - 1,
                value.name.as_bytes().len()
            ))
        } else {
            for (src, dst) in value.name.bytes().zip(out.name.iter_mut()) {
                *dst = src as _;
            }
            Ok(out)
        }
    }
}

impl TryFrom<DeviceIdentifier> for capi::DeviceIdentifier {
    type Error = anyhow::Error;

    fn try_from(value: DeviceIdentifier) -> Result<Self, Self::Error> {
        value.try_into()
    }
}