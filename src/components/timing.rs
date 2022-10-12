use crate::capi;
use crate::components::{SampleRateHz, TriggerEdge};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use super::macros::impl_plain_old_dict;

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timing {
    #[pyo3(get, set)]
    #[serde(default)]
    terminal: u8,

    #[pyo3(get, set)]
    #[serde(default)]
    edge: TriggerEdge,

    #[pyo3(get, set)]
    samples_per_second: Py<SampleRateHz>,
}

impl_plain_old_dict!(Timing);

impl Default for Timing {
    fn default() -> Self {
        let samples_per_second =
            Python::with_gil(|py| Py::new(py, SampleRateHz::default())).unwrap();
        Self {
            terminal: Default::default(),
            edge: Default::default(),
            samples_per_second,
        }
    }
}

impl TryFrom<capi::SignalProperties_signal_properties_timing_s> for Timing {
    type Error = anyhow::Error;

    fn try_from(
        value: capi::SignalProperties_signal_properties_timing_s,
    ) -> Result<Self, Self::Error> {
        let samples_per_second: SampleRateHz = value.samples_per_second.into();
        let samples_per_second = Python::with_gil(|py| Py::new(py, samples_per_second))?;
        let edge = value.edge.try_into()?;
        Ok(Self {
            terminal: value.terminal,
            edge,
            samples_per_second,
        })
    }
}

impl From<&Timing> for capi::SignalProperties_signal_properties_timing_s {
    fn from(value: &Timing) -> Self {
        let samples_per_second: capi::SampleRateHz = Python::with_gil(|py| {
            value
                .samples_per_second
                .extract::<SampleRateHz>(py)
                .unwrap()
                .into()
        });
        Self {
            terminal: value.terminal,
            edge: value.edge.into(),
            samples_per_second,
        }
    }
}
