use pyo3::prelude::*;

use crate::{capi, components::macros::cvt};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::slice::Iter;

#[pyclass]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SampleType {
    U8,
    U16,
    I8,
    I16,
    F32,
    U10,
    U12,
    U14,
}

impl Default for SampleType {
    fn default() -> Self {
        SampleType::U8
    }
}

impl SampleType {
    pub fn iter() -> Iter<'static, SampleType> {
        static SAMPLE_TYPES: [SampleType; 8] = [
            SampleType::U8,
            SampleType::U16,
            SampleType::I8,
            SampleType::I16,
            SampleType::F32,
            SampleType::U10,
            SampleType::U12,
            SampleType::U14,
        ];
        SAMPLE_TYPES.iter()
    }
}

cvt!(SampleType => capi::SampleType,
    U8 => SampleType_SampleType_u8,
    U16 => SampleType_SampleType_u16,
    I8 => SampleType_SampleType_i8,
    I16 => SampleType_SampleType_i16,
    F32 => SampleType_SampleType_f32,
    U10 => SampleType_SampleType_u10,
    U12 => SampleType_SampleType_u12,
    U14 => SampleType_SampleType_u14
);
