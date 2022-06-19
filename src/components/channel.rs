use std::ffi::CStr;

use crate::capi;
use crate::components::{SampleType, SignalIOKind, SignalType, VoltageRange};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use super::macros::impl_plain_old_dict;

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    #[pyo3(get, set)]
    #[serde(default)]
    sample_type: SampleType,

    #[pyo3(get, set)]
    #[serde(default)]
    signal_type: SignalType,

    #[pyo3(get, set)]
    #[serde(default)]
    signal_io_kind: SignalIOKind,

    #[pyo3(get, set)]
    voltage_range: Py<VoltageRange>,

    #[pyo3(get, set)]
    #[serde(default)]
    line: u8,

    #[pyo3(get, set)]
    #[serde(default)]
    display_name: String,
}

impl_plain_old_dict!(Channel);

impl Default for Channel {
    fn default() -> Self {
        let voltage_range = Python::with_gil(|py| Py::new(py, VoltageRange::default())).unwrap();
        Self {
            sample_type: Default::default(),
            signal_type: Default::default(),
            signal_io_kind: Default::default(),
            voltage_range,
            line: Default::default(),
            display_name: Default::default(),
        }
    }
}

impl TryFrom<capi::Channel> for Channel {
    type Error = anyhow::Error;

    fn try_from(value: capi::Channel) -> Result<Self, Self::Error> {
        let voltage_range: VoltageRange = value.voltage_range.into();
        let voltage_range = Python::with_gil(|py| Py::new(py, voltage_range)).unwrap();
        let display_name = unsafe { CStr::from_ptr(value.display_name.as_ptr()) }.to_str()?;
        let display_name = display_name.to_owned();
        Ok(Self {
            sample_type: value.sample_type.try_into()?,
            signal_type: value.signal_type.try_into()?,
            signal_io_kind: value.signal_io_kind.try_into()?,
            voltage_range,
            line: value.line,
            display_name,
        })
    }
}

impl TryFrom<&Channel> for capi::Channel {
    type Error = anyhow::Error;
    fn try_from(value: &Channel) -> Result<Self, Self::Error> {
        let voltage_range: capi::VoltageRange = Python::with_gil(|py| {
            value
                .voltage_range
                .extract::<VoltageRange>(py)
                .unwrap()
                .into()
        });
        let mut out = Self {
            sample_type: value.sample_type.into(),
            signal_type: value.signal_type.into(),
            signal_io_kind: value.signal_io_kind.into(),
            voltage_range,
            line: value.line,
            display_name: [0; 64],
        };

        if value.display_name.as_bytes().len() >= out.display_name.len() - 1 {
            Err(anyhow::anyhow!(
                "Device display name is too long. Names are limited to {} bytes. Got {}.",
                out.display_name.len() - 1,
                value.display_name.as_bytes().len()
            ))
        } else {
            for (src, dst) in value.display_name.bytes().zip(out.display_name.iter_mut()) {
                *dst = src as _;
            }
            Ok(out)
        }
    }
}

impl From<Channel> for capi::Channel {
    fn from(value: Channel) -> Self {
        value.into()
    }
}

impl Default for capi::Channel {
    fn default() -> Self {
        Self {
            sample_type: Default::default(),
            signal_type: Default::default(),
            signal_io_kind: Default::default(),
            voltage_range: Default::default(),
            line: Default::default(),
            display_name: [0;64],
        }
    }
}
