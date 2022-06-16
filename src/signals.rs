use crate::{
    capi,
    components::{macros::impl_plain_old_dict, Channel, Timing, Trigger},
};
use anyhow::anyhow;
use pyo3::{prelude::*, types::PyList};
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone)]
pub struct SignalProperties {
    channels: Py<PyList>,
    timing: Py<Timing>,
    triggers: Py<PyList>,
}

impl_plain_old_dict!(SignalProperties);

impl Default for SignalProperties {
    fn default() -> Self {
        let (channels, triggers) =
            Python::with_gil(|py| (PyList::empty(py).into(), PyList::empty(py).into()));
        Self {
            channels,
            timing: Python::with_gil(|py| Py::new(py, Timing::default())).unwrap(),
            triggers,
        }
    }
}

impl TryFrom<capi::SignalProperties> for SignalProperties {
    type Error = anyhow::Error;

    fn try_from(value: capi::SignalProperties) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let triggers = {
                let v: Vec<_> = (0..value.triggers.line_count as usize)
                    .map(|i| {
                        let t: Trigger = value.triggers.lines[i].try_into()?;
                        Py::new(py, t)
                    })
                    .collect::<Result<_, _>>()?;
                PyList::new(py, v).into()
            };
            let channels = {
                let v: Vec<_> = (0..value.channels.line_count as usize)
                    .map(|i| {
                        let t: Channel = value.channels.lines[i].try_into()?;
                        Py::new(py, t)
                    })
                    .collect::<Result<_, _>>()?;
                PyList::new(py, v).into()
            };
            let timing: Timing = value.timing.try_into()?;
            let timing = Py::new(py, timing)?;
            Ok(Self {
                channels,
                timing,
                triggers,
            })
        })?)
    }
}

impl TryFrom<&SignalProperties> for capi::SignalProperties {
    type Error = anyhow::Error;

    fn try_from(src: &SignalProperties) -> Result<Self, Self::Error> {
        let mut dst_channels: capi::SignalProperties_signal_properties_channels_s =
            unsafe { std::mem::zeroed() };
        let mut dst_triggers: capi::SignalProperties_signal_properties_triggers_s =
            unsafe { std::mem::zeroed() };

        let (src_channel_count, src_trigger_count) = Python::with_gil(|py| {
            let src_channel_count = src.channels.as_ref(py).len();
            let src_trigger_count = src.triggers.as_ref(py).len();
            (src_channel_count, src_trigger_count)
        });

        if src_channel_count > dst_channels.lines.len() {
            return Err(anyhow!(
                "Expected fewer channel lines. Require {}<{}",
                src_channel_count,
                dst_channels.lines.len()
            ));
        }

        if src_trigger_count > dst_triggers.lines.len() {
            return Err(anyhow!(
                "Expected fewer trigger lines. Require {}<{}",
                src_trigger_count,
                dst_triggers.lines.len()
            ));
        }

        Ok(Python::with_gil(|py| -> PyResult<_> {
            for (src, dst) in src
                .channels
                .as_ref(py)
                .iter()
                .zip(dst_channels.lines.iter_mut())
            {
                *dst = src.extract::<Channel>()?.into()
            }

            for (src, dst) in src
                .triggers
                .as_ref(py)
                .iter()
                .zip(dst_triggers.lines.iter_mut())
            {
                *dst = src.extract::<Trigger>()?.into()
            }

            let timing:Timing=src.timing.extract(py)?;
            Ok(Self {
                channels: dst_channels,
                timing: timing.try_into()?,
                triggers: dst_triggers,
            })
        })?)
    }
}
