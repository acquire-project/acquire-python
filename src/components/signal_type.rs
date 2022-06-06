use pyo3::prelude::*;

use anyhow::anyhow;
use std::os::raw::c_uint;
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

cvt!(SignalType => c_uint,
    Analog => SignalType_Signal_Analog,
    Digital => SignalType_Signal_Digital
);
