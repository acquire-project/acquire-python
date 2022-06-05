use pyo3::prelude::*;

use anyhow::anyhow;
use std::os::raw::c_uint;
use crate::{core_runtime, components::macros::cvt};

#[pyclass]
#[derive(Debug,Clone,Copy)]
pub enum SignalIOKind {
    Input,
    Output
}

impl Default for SignalIOKind {
    fn default() -> Self {
        SignalIOKind::Input
    }
}

cvt!(SignalIOKind => c_uint,
    Input => SignalIOKind_Signal_Input,
    Output => SignalIOKind_Signal_Output
);
