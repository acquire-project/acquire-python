use pyo3::prelude::*;

use crate::{capi, components::macros::cvt};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SampleType {
    U8,
    U16,
    I8,
    I16,
    F32,
}

impl Default for SampleType {
    fn default() -> Self {
        SampleType::U8
    }
}

cvt!(SampleType => capi::SampleType,
    U8 => SampleType_SampleType_u8,
    U16 => SampleType_SampleType_u16,
    I8 => SampleType_SampleType_i8,
    I16 => SampleType_SampleType_i16,
    F32 => SampleType_SampleType_f32
);
