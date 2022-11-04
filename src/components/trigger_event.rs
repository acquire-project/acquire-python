use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{capi, components::macros::cvt};
use anyhow::anyhow;

#[pyclass]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum TriggerEvent {
    AcquisitionStart,
    FrameStart,
    Exposure,
    FrameTriggerWait,
    Unknown,
}

impl Default for TriggerEvent {
    fn default() -> Self {
        TriggerEvent::AcquisitionStart
    }
}

cvt!( TriggerEvent => capi::TriggerEvent,
    AcquisitionStart => TriggerEvent_TriggerEvent_AcquisitionStart,
    FrameStart => TriggerEvent_TriggerEvent_FrameStart,
    Exposure => TriggerEvent_TriggerEvent_Exposure,
    FrameTriggerWait => TriggerEvent_TriggerEvent_FrameTriggerWait,
    Unknown => TriggerEvent_TriggerEvent_Unknown
);
