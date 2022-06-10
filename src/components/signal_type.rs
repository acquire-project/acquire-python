use pyo3::prelude::*;

use crate::{capi, components::macros::cvt};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SignalType {
    Analog,
    Digital,
}

impl Default for SignalType {
    fn default() -> Self {
        SignalType::Analog
    }
}

cvt!(SignalType => capi::SignalType,
    Analog => SignalType_Signal_Analog,
    Digital => SignalType_Signal_Digital
);
