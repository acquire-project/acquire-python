use pyo3::{Py, Python};
use crate::capi;
use crate::components::{SampleType, SignalIOKind, SignalType, VoltageRange};

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
        }
    }
}

impl TryFrom<capi::Channel> for Channel {
    type Error = anyhow::Error;

    fn try_from(value: capi::Channel) -> Result<Self, Self::Error> {
        let voltage_range: VoltageRange = value.voltage_range.into();
        let voltage_range = Python::with_gil(|py| Py::new(py, voltage_range)).unwrap();
        Ok(Self {
            sample_type: value.sample_type.try_into()?,
            signal_type: value.signal_type.try_into()?,
            signal_io_kind: value.signal_io_kind.try_into()?,
            voltage_range,
            line: value.line,
        })
    }
}

impl From<&Channel> for capi::Channel {
    fn from(value: &Channel) -> Self {
        let voltage_range: capi::VoltageRange = Python::with_gil(|py| {
            value
                .voltage_range
                .extract::<VoltageRange>(py)
                .unwrap()
                .into()
        });
        Self {
            sample_type: value.sample_type.into(),
            signal_type: value.signal_type.into(),
            signal_io_kind: value.signal_io_kind.into(),
            voltage_range,
            line: value.line,
        }
    }
}

impl From<Channel> for capi::Channel {
    fn from(value: Channel) -> Self {
        value.into()
    }
}