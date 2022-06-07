use pyo3::prelude::*;

use anyhow::anyhow;
use crate::{core_runtime, components::macros::cvt};

#[pyclass]
#[derive(Debug,Clone,Copy)]
pub enum SignalType {
    Analog,
    Digital
}

impl Default for SignalType {
    fn default() -> Self {
        SignalType::Analog
    }
}

cvt!(SignalType => core_runtime::SignalType,
    Analog => SignalType_Signal_Analog,
    Digital => SignalType_Signal_Digital
);
