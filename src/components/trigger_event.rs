use pyo3::prelude::*;

use crate::{core_runtime, components::macros::cvt};
use anyhow::anyhow;
use std::os::raw::c_uint;

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

cvt!( TriggerEvent => c_uint,
    AcquisitionStart => TriggerEvent_TriggerEvent_AcquisitionStart,
    FrameStart => TriggerEvent_TriggerEvent_FrameStart,
    Exposure => TriggerEvent_TriggerEvent_Exposure,
    FrameTriggerWait => TriggerEvent_TriggerEvent_FrameTriggerWait
);