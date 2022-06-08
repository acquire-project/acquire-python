use pyo3::prelude::*;

use anyhow::anyhow;
use crate::{capi, components::macros::cvt};

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

cvt!(SignalType => capi::SignalType,
    Analog => SignalType_Signal_Analog,
    Digital => SignalType_Signal_Digital
);
