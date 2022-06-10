use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    capi,
    components::{macros::impl_plain_old_dict, PID},
};

#[pyclass]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct StageAxisState {
    #[pyo3(get, set)]
    position: f32,

    #[pyo3(get, set)]
    velocity: f32,
}

impl From<capi::StageAxisProperties_stage_axis_properties_state_s> for StageAxisState {
    fn from(value: capi::StageAxisProperties_stage_axis_properties_state_s) -> Self {
        Self {
            position: value.position,
            velocity: value.velocity,
        }
    }
}

impl From<StageAxisState> for capi::StageAxisProperties_stage_axis_properties_state_s {
    fn from(value: StageAxisState) -> Self {
        Self {
            position: value.position,
            velocity: value.velocity,
        }
    }
}

#[pyclass]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct StageAxisProperties {
    #[pyo3(get, set)]
    #[serde(default)]
    target: StageAxisState,

    #[pyo3(get, set)]
    #[serde(default)]
    immediate: StageAxisState,

    #[pyo3(get, set)]
    #[serde(default)]
    feedback: PID,
}

impl_plain_old_dict!(StageAxisProperties);

impl From<capi::StageAxisProperties> for StageAxisProperties {
    fn from(value: capi::StageAxisProperties) -> Self {
        Self {
            target: value.target.into(),
            immediate: value.immediate.into(),
            feedback: value.feedback.into(),
        }
    }
}

impl From<StageAxisProperties> for capi::StageAxisProperties {
    fn from(value: StageAxisProperties) -> Self {
        Self {
            target: value.target.into(),
            immediate: value.immediate.into(),
            feedback: value.feedback.into(),
        }
    }
}
