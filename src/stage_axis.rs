use pyo3::prelude::*;

use crate::{components::PID, core_runtime};

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct StageAxisState {
    #[pyo3(get, set)]
    position: f32,

    #[pyo3(get, set)]
    velocity: f32,
}

impl From<core_runtime::StageAxisProperties_stage_axis_properties_state_s> for StageAxisState {
    fn from(value: core_runtime::StageAxisProperties_stage_axis_properties_state_s) -> Self {
        Self {
            position: value.position,
            velocity: value.velocity,
        }
    }
}

impl From<StageAxisState> for core_runtime::StageAxisProperties_stage_axis_properties_state_s {
    fn from(value: StageAxisState) -> Self {
        Self {
            position: value.position,
            velocity: value.velocity,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct StageAxisProperties {
    #[pyo3(get, set)]
    target: StageAxisState,

    #[pyo3(get, set)]
    immediate: StageAxisState,

    #[pyo3(get, set)]
    feedback: PID,
}

impl From<core_runtime::StageAxisProperties> for StageAxisProperties {
    fn from(value: core_runtime::StageAxisProperties) -> Self {
        Self {
            target: value.target.into(),
            immediate: value.immediate.into(),
            feedback: value.feedback.into(),
        }
    }
}

impl From<StageAxisProperties> for core_runtime::StageAxisProperties {
    fn from(value: StageAxisProperties) -> Self {
        Self {
            target: value.target.into(),
            immediate: value.immediate.into(),
            feedback: value.feedback.into(),
        }
    }
}