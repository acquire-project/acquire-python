mod direction;
pub(crate) mod macros;
mod sample_type;
mod signal_io_kind;
mod signal_type;
mod trigger_edge;

use anyhow::Result;
use pyo3::prelude::*;

use serde::{Deserialize, Serialize};

// Exports
pub use direction::Direction;
pub use sample_type::SampleType;
pub use signal_io_kind::SignalIOKind;
pub use signal_type::SignalType;
pub use trigger_edge::TriggerEdge;

use crate::{capi, components::macros::impl_plain_old_dict};

#[pyclass]
#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize)]
pub struct Trigger {
    #[pyo3(get, set)]
    #[serde(default)]
    enable: bool,

    #[pyo3(get, set)]
    #[serde(default)]
    line: u8,

    #[pyo3(get, set)]
    #[serde(default)]
    kind: SignalIOKind,

    #[pyo3(get, set)]
    #[serde(default)]
    edge: TriggerEdge,
}

impl_plain_old_dict!(Trigger);

impl AsRef<Trigger> for Trigger {
    fn as_ref(&self) -> &Trigger {
        self
    }
}

impl TryFrom<capi::Trigger> for Trigger {
    type Error = anyhow::Error;

    fn try_from(value: capi::Trigger) -> Result<Self, Self::Error> {
        Ok(Trigger {
            enable: value.enable > 0,
            line: value.line,
            kind: value.kind.try_into()?,
            edge: value.edge.try_into()?,
        })
    }
}

impl From<&Trigger> for capi::Trigger {
    fn from(value: &Trigger) -> Self {
        Self {
            enable: value.enable as _,
            line: value.line,
            kind: value.kind.into(),
            edge: value.edge.into(),
        }
    }
}

impl Default for capi::Trigger {
    fn default() -> Self {
        Self {
            enable: Default::default(),
            line: Default::default(),
            kind: Default::default(),
            edge: Default::default(),
        }
    }
}

#[pyclass]
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct PID {
    #[pyo3(get, set)]
    #[serde(default)]
    proportional: f32,

    #[pyo3(get, set)]
    #[serde(default)]
    integral: f32,

    #[pyo3(get, set)]
    #[serde(default)]
    derivative: f32,
}

impl_plain_old_dict!(PID);

impl From<capi::PID> for PID {
    fn from(value: capi::PID) -> Self {
        Self {
            proportional: value.proportional,
            integral: value.integral,
            derivative: value.derivative,
        }
    }
}

impl From<PID> for capi::PID {
    fn from(value: PID) -> Self {
        Self {
            proportional: value.proportional,
            integral: value.integral,
            derivative: value.derivative,
        }
    }
}

impl Default for capi::PID {
    fn default() -> Self {
        Self {
            proportional: Default::default(),
            integral: Default::default(),
            derivative: Default::default(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SampleRateHz {
    #[pyo3(get, set)]
    #[serde(default)]
    numerator: u64,

    #[pyo3(get, set)]
    #[serde(default)]
    denominator: u64,
}

impl_plain_old_dict!(SampleRateHz);

impl Default for SampleRateHz {
    fn default() -> Self {
        Self {
            numerator: 1,
            denominator: 1,
        }
    }
}

impl From<capi::SampleRateHz> for SampleRateHz {
    fn from(value: capi::SampleRateHz) -> Self {
        SampleRateHz {
            numerator: value.numerator,
            denominator: value.denominator,
        }
    }
}

impl From<SampleRateHz> for capi::SampleRateHz {
    fn from(value: SampleRateHz) -> Self {
        Self {
            numerator: value.numerator,
            denominator: value.denominator,
        }
    }
}

impl Default for capi::SampleRateHz {
    fn default() -> Self {
        Self {
            numerator: 1,
            denominator: 1,
        }
    }
}

/// Voltage range
#[pyclass]
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub struct VoltageRange {
    /// Minimum voltage
    #[pyo3(get, set)]
    #[serde(default)]
    mn: f32,

    /// Maximum voltage
    #[pyo3(get, set)]
    #[serde(default)]
    mx: f32,
}

impl_plain_old_dict!(VoltageRange);

impl From<capi::VoltageRange> for VoltageRange {
    fn from(value: capi::VoltageRange) -> Self {
        VoltageRange {
            mn: value.mn,
            mx: value.mx,
        }
    }
}

impl From<VoltageRange> for capi::VoltageRange {
    fn from(value: VoltageRange) -> Self {
        Self {
            mn: value.mn,
            mx: value.mx,
        }
    }
}

impl Default for capi::VoltageRange {
    fn default() -> Self {
        Self { mn: -1.0, mx: 1.0 }
    }
}
