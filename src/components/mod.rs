pub(crate) mod macros;
mod sample_type;
mod signal_io_kind;
mod signal_type;
mod trigger_edge;
mod trigger_event;

use pyo3::{prelude::*, types::PyDict};
use pythonize::{depythonize, pythonize};
use serde::{Deserialize, Serialize};
use anyhow::Result;

pub use sample_type::SampleType;
pub use signal_io_kind::SignalIOKind;
pub use signal_type::SignalType;
pub use trigger_edge::TriggerEdge;
pub use trigger_event::TriggerEvent;

use crate::capi;


#[pyclass]
#[derive(Debug, Copy, Clone, Default, Deserialize, Serialize)]
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

#[pymethods]
impl Trigger {
    #[new]
    #[args(kwargs="**")]
    fn __new__(kwargs:Option<&PyDict>)->Result<Self> {
        if let Some(kwargs)=kwargs {
            Ok(depythonize(kwargs)?)
        } else {
            Ok(Default::default())
        }
    }

    fn __repr__(&self,py:Python<'_>)->PyResult<String> {
        let obj=pythonize(py, self)?;
        let obj=obj.as_ref(py).downcast::<PyDict>()?;
        let args:String=obj
            .iter()
            .map(|(k,v)| format!("{}='{}'",k,v))
            .reduce(|acc,e| format!("{},{}",acc,e))
            .unwrap_or(String::new());

        Ok(format!("Trigger({})",args))
    }
}

impl TryFrom<capi::Trigger> for Trigger {
    type Error = anyhow::Error;

    fn try_from(value: capi::Trigger) -> Result<Self, Self::Error> {
        Ok(Trigger {
            enable: value.enable > 0,
            line: value.line,
            event: value.event.try_into()?,
            kind: value.kind.try_into()?,
            edge: value.edge.try_into()?,
        })
    }
}

impl From<Trigger> for capi::Trigger {
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

#[pyclass]
#[derive(Debug, Default, Clone, Copy)]
pub struct SampleRateHz {
    #[pyo3(get, set)]
    numerator: u64,

    #[pyo3(get, set)]
    denominator: u64,
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

#[pyclass]
#[derive(Debug, Default, Clone, Copy)]
pub struct VoltageRange {
    #[pyo3(get, set)]
    mn: f32,

    #[pyo3(get, set)]
    mx: f32,
}

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
