use pyo3::prelude::*;

use crate::{capi, components::macros::cvt};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Direction {
    Forward,
    Backward,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Forward
    }
}

cvt!(Direction => capi::Direction,
    Forward => Direction_Direction_Forward,
    Backward => Direction_Direction_Backward
);
