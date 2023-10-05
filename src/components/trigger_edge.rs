use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{capi, components::macros::cvt};
use anyhow::anyhow;

#[pyclass]
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum TriggerEdge {
    Rising,
    Falling,
    NotApplicable,
    AnyEdge,
    LevelHigh,
    LevelLow,
}

impl Default for TriggerEdge {
    fn default() -> Self {
        TriggerEdge::Rising
    }
}

cvt!( TriggerEdge => capi::TriggerEdge,
    Rising => TriggerEdge_TriggerEdge_Rising,
    Falling => TriggerEdge_TriggerEdge_Falling,
    NotApplicable => TriggerEdge_TriggerEdge_NotApplicable,
    AnyEdge => TriggerEdge_TriggerEdge_AnyEdge,
    LevelHigh => TriggerEdge_TriggerEdge_LevelHigh,
    LevelLow => TriggerEdge_TriggerEdge_LevelLow
);
