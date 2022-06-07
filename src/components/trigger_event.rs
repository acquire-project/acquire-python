use pyo3::prelude::*;

use crate::{core_runtime, components::macros::cvt};
use anyhow::anyhow;

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub enum TriggerEvent {
    AcquisitionStart,
    FrameStart,
    Exposure,
    FrameTriggerWait,
}

impl Default for TriggerEvent {
    fn default() -> Self {
        TriggerEvent::AcquisitionStart
    }
}

cvt!( TriggerEvent => core_runtime::TriggerEvent,
    AcquisitionStart => TriggerEvent_TriggerEvent_AcquisitionStart,
    FrameStart => TriggerEvent_TriggerEvent_FrameStart,
    Exposure => TriggerEvent_TriggerEvent_Exposure,
    FrameTriggerWait => TriggerEvent_TriggerEvent_FrameTriggerWait
);