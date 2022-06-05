use pyo3::prelude::*;

use anyhow::anyhow;
use std::os::raw::c_uint;
use crate::core_runtime;

#[pyclass]
#[derive(Debug,Clone,Copy)]
pub enum SampleType {
    U8,
    U16,
    I8,
    I16,
    F32
}

impl Default for SampleType {
    fn default() -> Self {
        SampleType::U8
    }
}

// Doing these as a macro just in case.
// FIXME: remove this later if it's not needed, unnecessarily complex
macro_rules! sample_type_conversions{
    ($($T:ty),+)=>{
        $(
            impl TryFrom<$T> for SampleType {
                type Error=anyhow::Error;
            
                fn try_from(value: $T) -> Result<Self, Self::Error> {
                    match value as c_uint {
                        core_runtime::SampleType_SampleType_u8 => Ok(SampleType::U8),
                        core_runtime::SampleType_SampleType_u16 => Ok(SampleType::U16),
                        core_runtime::SampleType_SampleType_i8 => Ok(SampleType::I8),
                        core_runtime::SampleType_SampleType_i16 => Ok(SampleType::I16),
                        core_runtime::SampleType_SampleType_f32 => Ok(SampleType::F32),
                        _ => Err(anyhow!("Unknown SampleType: {}",value))
                    }
                }
            }
            
            impl Into<$T> for SampleType {
                fn into(self) -> $T {
                    match self {
                        SampleType::U8 =>  core_runtime::SampleType_SampleType_u8 as $T,
                        SampleType::U16 => core_runtime::SampleType_SampleType_u16 as $T,
                        SampleType::I8 =>  core_runtime::SampleType_SampleType_i8 as $T,
                        SampleType::I16 => core_runtime::SampleType_SampleType_i16 as $T,
                        SampleType::F32 => core_runtime::SampleType_SampleType_f32 as $T,
                    } 
                }
            }
        )+
    }
}

sample_type_conversions!(c_uint);
