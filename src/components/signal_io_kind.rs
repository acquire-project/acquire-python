use pyo3::prelude::*;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use crate::{capi, components::macros::cvt};

#[pyclass]
#[derive(Debug,Clone,Copy, Deserialize, Serialize)]
pub enum SignalIOKind {
    Input,
    Output
}

impl Default for SignalIOKind {
    fn default() -> Self {
        SignalIOKind::Input
    }
}

cvt!(SignalIOKind => capi::SignalIOKind,
    Input => SignalIOKind_Signal_Input,
    Output => SignalIOKind_Signal_Output
);
