use pyo3::{prelude::*, pyclass::CompareOp};
use serde::{Deserialize, Serialize};
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

impl Default for capi::DeviceIdentifier {
    fn default() -> Self {
        Self {
            driver_id: Default::default(),
            device_id: Default::default(),
            kind: capi::DeviceKind_DeviceKind_None,
            name: [0; 256],
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DeviceState {
    Closed,
    AwaitingConfiguration,
    Armed,
    Running,
}

impl Default for DeviceState {
    fn default() -> Self {
        DeviceState::Closed
    }
}

cvt!(DeviceState => capi::DeviceState,
    Closed => DeviceState_DeviceState_Closed,
    AwaitingConfiguration => DeviceState_DeviceState_AwaitingConfiguration,
    Armed => DeviceState_DeviceState_Armed,
    Running => DeviceState_DeviceState_Running
);

#[pyclass]
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceKind {
    NONE,
    Camera,
    Storage,
    StageAxis,
    Signals,
}

impl Default for DeviceKind {
    fn default() -> Self {
        DeviceKind::NONE
    }
}

cvt!(DeviceKind=>capi::DeviceKind,
    NONE      => DeviceKind_DeviceKind_None,
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
            "None" | "NONE" => Ok(DeviceKind::NONE),
            "Camera" => Ok(DeviceKind::Camera),
            "Storage" => Ok(DeviceKind::Storage),
            "StageAxis" => Ok(DeviceKind::StageAxis),
            "Signals" => Ok(DeviceKind::Signals),
            _ => Err(anyhow!("Did not recognize {} as a valid DeviceKind", value)),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct DeviceIdentifier {
    #[pyo3(get)]
    id: (u8, u8),

    #[pyo3(get)]
    kind: DeviceKind,

    #[pyo3(get)]
    name: String,
}

// FIXME: (nclack) don't want to serialize the id field.  It's unstable,
//                 only makes sense in context of an active runtime.
//                 Should probably drop the id's altogether except maybe for
//                 internal debugging.

// TODO: (nclack) maybe use impl_plain_old_dict for device identifier. caveat ^

#[pymethods]
impl DeviceIdentifier {
    fn __repr__(&self) -> String {
        format!("<DeviceIdentifier {:?} \"{}\">", self.kind, self.name)
    }

    fn dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(pythonize::pythonize(py, self)?)
    }

    fn __richcmp__(&self, other: &DeviceIdentifier, op: CompareOp) -> Py<PyAny> {
        Python::with_gil(|py| match op {
            CompareOp::Eq => (self == other).into_py(py),
            CompareOp::Ne => (self != other).into_py(py),
            _ => py.NotImplemented(),
        })
    }
}

impl TryFrom<capi::DeviceIdentifier> for DeviceIdentifier {
    type Error = anyhow::Error;

    fn try_from(value: capi::DeviceIdentifier) -> Result<Self, Self::Error> {
        if value.kind == capi::DeviceKind_DeviceKind_None {
            return Ok(DeviceIdentifier::default());
        }

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
