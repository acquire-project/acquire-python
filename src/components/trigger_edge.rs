use pyo3::prelude::*;

use crate::{core_runtime, components::macros::cvt};
use anyhow::anyhow;


#[pyclass]
#[derive(Debug, Clone, Copy)]
pub enum TriggerEdge {
    Rising,
    Falling,
    NotApplicable
}

impl Default for TriggerEdge {
    fn default() -> Self {
        TriggerEdge::Rising
    }
}

cvt!( TriggerEdge => core_runtime::TriggerEdge,
    Rising => TriggerEdge_TriggerEdge_Rising,
    Falling => TriggerEdge_TriggerEdge_Falling,
    NotApplicable => TriggerEdge_TriggerEdge_NotApplicable
);