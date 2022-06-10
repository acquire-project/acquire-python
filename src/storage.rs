use crate::{capi, components::macros::impl_plain_old_dict, Status};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    ffi::{CStr, CString},
    ptr::null,
};

#[pyclass]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageProperties {
    #[pyo3(get, set)]
    #[serde(default)]
    pub(crate) filename: Option<String>,

    /// Doesn't do anything right now. One day could be used for file-rollover.
    #[pyo3(get, set)]
    #[serde(default)]
    pub(crate) first_frame_id: u32,
}

impl_plain_old_dict!(StorageProperties);

impl TryFrom<capi::StorageProperties> for StorageProperties {
    type Error = anyhow::Error;

    fn try_from(value: capi::StorageProperties) -> Result<Self, Self::Error> {
        let filename = if value.filename.nbytes == 0 {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(value.filename.str_) }
                    .to_str()?
                    .to_owned(),
            )
        };
        Ok(Self {
            filename,
            first_frame_id: value.first_frame_id,
        })
    }
}

impl TryFrom<&StorageProperties> for capi::StorageProperties {
    type Error = anyhow::Error;

    fn try_from(value: &StorageProperties) -> Result<Self, Self::Error> {
        let mut out: capi::StorageProperties = unsafe { std::mem::zeroed() };
        let x = if let Some(filename) = &value.filename {
            Some(CString::new(filename.as_str())?)
        } else {
            None
        };
        let (filename, nbytes) = if let Some(x) = x {
            (x.as_c_str().as_ptr(), x.to_bytes_with_nul().len())
        } else {
            (null(), 0)
        };
        // This copies the string into a buffer owned by the return value.
        unsafe {
            capi::storage_properties_init(&mut out, value.first_frame_id, filename, nbytes as _)
                .ok()?;
        }
        Ok(out)
    }
}