use pyo3::prelude::*;

use anyhow::anyhow;
use std::os::raw::c_uint;
use crate::{core_runtime, components::macros::cvt};

#[pyclass]
#[derive(Debug,Clone,Copy)]
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

cvt!(SampleType => c_uint,
    U8 => SampleType_SampleType_u8,
    U16 => SampleType_SampleType_u16,
    I8 => SampleType_SampleType_i8,
    I16 => SampleType_SampleType_i16,
    F32 => SampleType_SampleType_f32
);
