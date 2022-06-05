pub(crate) mod macros;
mod sample_type;
mod trigger_event;
mod trigger_edge;
mod signal_io_kind;

pub use sample_type::SampleType;
pub use trigger_event::TriggerEvent;
pub use trigger_edge::TriggerEdge;
pub use signal_io_kind::SignalIOKind;

use crate::core_runtime;

use pyo3::prelude::*;

#[pyclass]
#[derive(Debug,Copy,Clone,Default)]
pub struct Trigger {
    #[pyo3(get,set)]
    enable: bool,

    #[pyo3(get,set)]
    line: u8,

    #[pyo3(get,set)]
    event: TriggerEvent,

    #[pyo3(get,set)]
    kind: SignalIOKind,

    #[pyo3(get,set)]
    edge: TriggerEdge
}

impl TryFrom<core_runtime::Trigger> for Trigger {
    type Error=anyhow::Error;

    fn try_from(value: core_runtime::Trigger) -> Result<Self, Self::Error> {
        Ok(Trigger{
            enable: value.enable>0,
            line: value.line,
            event: value.event.try_into()?,
            kind: value.kind.try_into()?,
            edge: value.edge.try_into()?,
        })
    }
}

impl Into<core_runtime::Trigger> for Trigger {
    fn into(self) -> core_runtime::Trigger {
        core_runtime::Trigger {
            enable: self.enable as _,
            line: self.line,
            event: self.event.into(),
            kind: self.kind.into(),
            edge: self.edge.into(),
        }
    }
}