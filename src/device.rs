use pyo3::prelude::*;
use std::ffi::CStr;

use crate::{components::macros::cvt, core_runtime};
use anyhow::{anyhow, Result};

impl core_runtime::DeviceIdentifier {
    pub(crate) fn name_as_string(&self) -> Result<String> {
        Ok(unsafe { CStr::from_ptr(self.name.as_ptr()) }
            .to_str()?
            .to_owned())
    }
}

#[pyclass]
#[derive(Debug, Clone, Copy)]
enum DeviceKind {
    Camera,
    Storage,
    StageAxis,
    Signals,
}

cvt!(DeviceKind=>core_runtime::DeviceKind,
    Camera    => DeviceKind_DeviceKind_Camera,
    Storage   => DeviceKind_DeviceKind_Storage,
    StageAxis => DeviceKind_DeviceKind_StageAxis,
    Signals   => DeviceKind_DeviceKind_Signals
);

#[pyclass]
#[derive(Debug, Clone)]
struct DeviceIdentifier {
    #[pyo3(get)]
    id: (u8, u8),

    #[pyo3(get)]
    kind: DeviceKind,

    #[pyo3(get)]
    name: String,
}

impl TryFrom<core_runtime::DeviceIdentifier> for DeviceIdentifier {
    type Error = anyhow::Error;

    fn try_from(value: core_runtime::DeviceIdentifier) -> Result<Self, Self::Error> {
        Ok(Self {
            id: (value.driver_id, value.device_id),
            kind: value.kind.try_into()?,
            name: value.name_as_string()?,
        })
    }
}

impl TryFrom<DeviceIdentifier> for core_runtime::DeviceIdentifier {
    type Error = anyhow::Error;

    fn try_from(value: DeviceIdentifier) -> Result<Self, Self::Error> {
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
            for (src,dst) in value.name.bytes().zip(out.name.iter_mut()) {
                *dst=src as _;
            }
            Ok(out)
        }
    }
}
