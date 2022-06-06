use pyo3::prelude::*;

use crate::{
    components::{
        SampleRateHz, SampleType, SignalIOKind, SignalType, Trigger, TriggerEdge, VoltageRange,
    },
    core_runtime,
};
use anyhow::anyhow;

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct Channel {
    #[pyo3(get, set)]
    sample_type: SampleType,

    #[pyo3(get, set)]
    signal_type: SignalType,

    #[pyo3(get, set)]
    signal_io_kind: SignalIOKind,

    #[pyo3(get, set)]
    voltage_range: VoltageRange,

    #[pyo3(get, set)]
    line: u8,
}

impl TryFrom<core_runtime::Channel> for Channel {
    type Error = anyhow::Error;

    fn try_from(value: core_runtime::Channel) -> Result<Self, Self::Error> {
        Ok(Self {
            sample_type: value.sample_type.try_into()?,
            signal_type: value.signal_type.try_into()?,
            signal_io_kind: value.signal_io_kind.try_into()?,
            voltage_range: value.voltage_range.into(),
            line: value.line,
        })
    }
}

impl From<Channel> for core_runtime::Channel {
    fn from(value: Channel) -> Self {
        Self {
            sample_type: value.sample_type.into(),
            signal_type: value.signal_type.into(),
            signal_io_kind: value.signal_io_kind.into(),
            voltage_range: value.voltage_range.into(),
            line: value.line,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, Default)]
struct Timing {
    #[pyo3(get, set)]
    terminal: u8,

    #[pyo3(get, set)]
    edge: TriggerEdge,

    #[pyo3(get, set)]
    samples_per_second: SampleRateHz,
}

impl TryFrom<core_runtime::SignalProperties_signal_properties_timing_s> for Timing {
    type Error = anyhow::Error;

    fn try_from(
        value: core_runtime::SignalProperties_signal_properties_timing_s,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            terminal: value.terminal,
            edge: value.edge.try_into()?,
            samples_per_second: value.samples_per_second.into(),
        })
    }
}

impl From<Timing> for core_runtime::SignalProperties_signal_properties_timing_s {
    fn from(value: Timing) -> Self {
        Self {
            terminal: value.terminal,
            edge: value.edge.into(),
            samples_per_second: value.samples_per_second.into(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct SignalProperties {
    channels: Vec<Channel>,
    timing: Timing,
    triggers: Vec<Trigger>,
}

impl TryFrom<core_runtime::SignalProperties> for SignalProperties {
    type Error = anyhow::Error;

    fn try_from(value: core_runtime::SignalProperties) -> Result<Self, Self::Error> {
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

impl TryFrom<SignalProperties> for core_runtime::SignalProperties {
    type Error = anyhow::Error;

    fn try_from(value: SignalProperties) -> Result<Self, Self::Error> {
        let mut triggers: core_runtime::SignalProperties_signal_properties_triggers_s =
            unsafe { std::mem::zeroed() };

        let mut channels: core_runtime::SignalProperties_signal_properties_channels_s =
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

        for (src, dst) in value.triggers.into_iter().zip(triggers.lines.iter_mut()) {
            *dst = src.into()
        }

        for (src, dst) in value.channels.into_iter().zip(channels.lines.iter_mut()) {
            *dst = src.into()
        }

        Ok(Self {
            channels,
            timing: value.timing.try_into()?,
            triggers,
        })
    }
}
