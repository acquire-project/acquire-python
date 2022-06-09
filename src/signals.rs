use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    capi,
    components::{
        SampleRateHz, SampleType, SignalIOKind, SignalType, Trigger, TriggerEdge, VoltageRange,
        macros::impl_plain_old_dict
    },
};
use anyhow::anyhow;

#[pyclass]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    #[serde(default)]
    voltage_range: VoltageRange,

    #[pyo3(get, set)]
    #[serde(default)]
    line: u8,
}

impl_plain_old_dict!(Channel);

impl TryFrom<capi::Channel> for Channel {
    type Error = anyhow::Error;

    fn try_from(value: capi::Channel) -> Result<Self, Self::Error> {
        Ok(Self {
            sample_type: value.sample_type.try_into()?,
            signal_type: value.signal_type.try_into()?,
            signal_io_kind: value.signal_io_kind.try_into()?,
            voltage_range: value.voltage_range.into(),
            line: value.line,
        })
    }
}

impl From<&Channel> for capi::Channel {
    fn from(value: &Channel) -> Self {
        Self {
            sample_type: value.sample_type.into(),
            signal_type: value.signal_type.into(),
            signal_io_kind: value.signal_io_kind.into(),
            voltage_range: value.voltage_range.into(),
            line: value.line,
        }
    }
}

impl From<Channel> for capi::Channel {
    fn from(value: Channel) -> Self {
        value.into()
    }
}

#[pyclass]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Timing {
    #[pyo3(get, set)]
    #[serde(default)]
    terminal: u8,

    #[pyo3(get, set)]
    #[serde(default)]
    edge: TriggerEdge,

    #[pyo3(get, set)]
    #[serde(default)]
    samples_per_second: SampleRateHz,
}

impl_plain_old_dict!(Timing);

impl TryFrom<capi::SignalProperties_signal_properties_timing_s> for Timing {
    type Error = anyhow::Error;

    fn try_from(
        value: capi::SignalProperties_signal_properties_timing_s,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            terminal: value.terminal,
            edge: value.edge.try_into()?,
            samples_per_second: value.samples_per_second.into(),
        })
    }
}

impl From<&Timing> for capi::SignalProperties_signal_properties_timing_s {
    fn from(value: &Timing) -> Self {
        Self {
            terminal: value.terminal,
            edge: value.edge.into(),
            samples_per_second: value.samples_per_second.into(),
        }
    }
}

impl From<Timing> for capi::SignalProperties_signal_properties_timing_s {
    fn from(value: Timing) -> Self {
        value.into()
    }
}

#[pyclass]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SignalProperties {
    channels: Vec<Channel>,
    timing: Timing,
    triggers: Vec<Trigger>,
}

impl_plain_old_dict!(SignalProperties);

impl TryFrom<capi::SignalProperties> for SignalProperties {
    type Error = anyhow::Error;

    fn try_from(value: capi::SignalProperties) -> Result<Self, Self::Error> {
        Ok(Self {
            channels: (0..value.channels.line_count as usize)
                .map(|i| value.channels.lines[i].try_into())
                .collect::<Result<_, _>>()?,
            timing: value.timing.try_into()?,
            triggers: (0..value.triggers.line_count as usize)
                .map(|i| value.triggers.lines[i].try_into())
                .collect::<Result<_, _>>()?,
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

impl TryFrom<SignalProperties> for capi::SignalProperties {
    type Error = anyhow::Error;

    fn try_from(value: SignalProperties) -> Result<Self, Self::Error> {
        value.try_into()
    }
}