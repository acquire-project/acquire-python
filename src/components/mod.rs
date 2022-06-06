pub(crate) mod macros;
mod sample_type;
mod signal_io_kind;
mod signal_type;
mod trigger_edge;
mod trigger_event;

pub use sample_type::SampleType;
pub use signal_io_kind::SignalIOKind;
pub use signal_type::SignalType;
pub use trigger_edge::TriggerEdge;
pub use trigger_event::TriggerEvent;

use crate::core_runtime;

use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Copy, Clone, Default)]
pub struct Trigger {
    #[pyo3(get, set)]
    enable: bool,

    #[pyo3(get, set)]
    line: u8,

    #[pyo3(get, set)]
    event: TriggerEvent,

    #[pyo3(get, set)]
    kind: SignalIOKind,

    #[pyo3(get, set)]
    edge: TriggerEdge,
}

impl TryFrom<core_runtime::Trigger> for Trigger {
    type Error = anyhow::Error;

    fn try_from(value: core_runtime::Trigger) -> Result<Self, Self::Error> {
        Ok(Trigger {
            enable: value.enable > 0,
            line: value.line,
            event: value.event.try_into()?,
            kind: value.kind.try_into()?,
            edge: value.edge.try_into()?,
        })
    }
}

impl From<Trigger> for core_runtime::Trigger {
    fn from(value: Trigger) -> Self {
        Self {
            enable: value.enable as _,
            line: value.line,
            event: value.event.into(),
            kind: value.kind.into(),
            edge: value.edge.into(),
        }
    }
}

#[pyclass]
#[derive(Debug, Default, Clone, Copy)]
pub struct PID {
    #[pyo3(get, set)]
    proportional: f32,

    #[pyo3(get, set)]
    integral: f32,

    #[pyo3(get, set)]
    derivative: f32,
}

impl From<core_runtime::PID> for PID {
    fn from(value: core_runtime::PID) -> Self {
        Self {
            proportional: value.proportional,
            integral: value.integral,
            derivative: value.derivative,
        }
    }
}

impl From<PID> for core_runtime::PID {
    fn from(value: PID) -> Self {
        Self {
            proportional: value.proportional,
            integral: value.integral,
            derivative: value.derivative,
        }
    }
}

#[pyclass]
#[derive(Debug, Default, Clone, Copy)]
pub struct SampleRateHz {
    #[pyo3(get, set)]
    numerator: u64,

    #[pyo3(get, set)]
    denominator: u64,
}

impl From<core_runtime::SampleRateHz> for SampleRateHz {
    fn from(value: core_runtime::SampleRateHz) -> Self {
        SampleRateHz {
            numerator: value.numerator,
            denominator: value.denominator,
        }
    }
}

impl From<SampleRateHz> for core_runtime::SampleRateHz {
    fn from(value: SampleRateHz) -> Self {
        Self {
            numerator: value.numerator,
            denominator: value.denominator,
        }
    }
}

#[pyclass]
#[derive(Debug, Default, Clone, Copy)]
pub struct VoltageRange {
    #[pyo3(get, set)]
    mn: f32,

    #[pyo3(get, set)]
    mx: f32,
}

impl From<core_runtime::VoltageRange> for VoltageRange {
    fn from(value: core_runtime::VoltageRange) -> Self {
        VoltageRange {
            mn: value.mn,
            mx: value.mx,
        }
    }
}

impl From<VoltageRange> for core_runtime::VoltageRange {
    fn from(value: VoltageRange) -> Self {
        Self {
            mn: value.mn,
            mx: value.mx,
        }
    }
}
