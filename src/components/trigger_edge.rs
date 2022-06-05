use pyo3::prelude::*;

use crate::{core_runtime, components::macros::cvt};
use anyhow::anyhow;
use std::os::raw::c_uint;

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

cvt!( TriggerEdge => c_uint,
    Rising => TriggerEdge_TriggerEdge_Rising,
    Falling => TriggerEdge_TriggerEdge_Falling,
    NotApplicable => TriggerEdge_TriggerEdge_NotApplicable
);