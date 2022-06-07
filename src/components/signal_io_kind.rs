use pyo3::prelude::*;

use anyhow::anyhow;
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

cvt!(SignalIOKind => core_runtime::SignalIOKind,
    Input => SignalIOKind_Signal_Input,
    Output => SignalIOKind_Signal_Output
);
