use crate::{
    capi,
    components::{macros::impl_plain_old_dict, Channel, Timing, Trigger},
};
use anyhow::anyhow;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalProperties {
    channels: Vec<Py<Channel>>, // FIXME: should by Py<PyList>
    timing: Py<Timing>,
    triggers: Vec<Py<Trigger>>, // FIXME: should by Py<PyList>
}

impl_plain_old_dict!(SignalProperties);

impl Default for SignalProperties {
    fn default() -> Self {
        Self {
            channels: Default::default(),
            timing: Python::with_gil(|py| Py::new(py, Timing::default())).unwrap(),
            triggers: Default::default(),
        }
    }
}

impl TryFrom<capi::SignalProperties> for SignalProperties {
    type Error = anyhow::Error;

    fn try_from(value: capi::SignalProperties) -> Result<Self, Self::Error> {
        let channels = (0..value.channels.line_count as usize)
            .map(|i| value.channels.lines[i].try_into())
            .collect::<Result<_, _>>()?;
        let triggers = (0..value.triggers.line_count as usize)
            .map(|i| value.triggers.lines[i].try_into())
            .collect::<Result<_, _>>()?;
        Ok(Self {
            channels,
            timing: value.timing.try_into()?,
            triggers,
        })
    }
}

impl TryFrom<&SignalProperties> for capi::SignalProperties {
    type Error = anyhow::Error;

    fn try_from(value: &SignalProperties) -> Result<Self, Self::Error> {
        let mut triggers: capi::SignalProperties_signal_properties_triggers_s =
            unsafe { std::mem::zeroed() };

        let mut channels: capi::SignalProperties_signal_properties_channels_s =
            unsafe { std::mem::zeroed() };

        if value.triggers.len() > triggers.lines.len() {
            return Err(anyhow!(
                "Expected fewer trigger lines. Require {}<{}",
                value.triggers.len(),
                triggers.lines.len()
            ));
        }

        if value.channels.len() > channels.lines.len() {
            return Err(anyhow!(
                "Expected fewer channels. Require {}<{}",
                value.channels.len(),
                channels.lines.len()
            ));
        }

        for (src, dst) in value.triggers.iter().zip(triggers.lines.iter_mut()) {
            *dst = src.into()
        }

        for (src, dst) in value.channels.iter().zip(channels.lines.iter_mut()) {
            *dst = src.into()
        }

        Ok(Self {
            channels,
            timing: (&value.timing).try_into()?,
            triggers,
        })
    }
}
