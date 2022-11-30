use crate::{capi, components::macros::impl_plain_old_dict, Status};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    ffi::{CStr, CString},
    fmt::{Debug, Display},
    ptr::{null, null_mut},
};

#[pyclass]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StorageProperties {
    #[pyo3(get, set)]
    #[serde(default)]
    pub(crate) filename: Option<String>,

    #[pyo3(get, set)]
    #[serde(default)]
    pub(crate) external_metadata_json: Option<String>,

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
        let external_metadata_json = if value.external_metadata_json.nbytes == 0 {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(value.external_metadata_json.str_) }
                    .to_str()?
                    .to_owned(),
            )
        };
        Ok(Self {
            filename,
            first_frame_id: value.first_frame_id,
            external_metadata_json,
        })
    }
}

impl TryFrom<&StorageProperties> for capi::StorageProperties {
    type Error = anyhow::Error;

    fn try_from(value: &StorageProperties) -> Result<Self, Self::Error> {
        let mut out: capi::StorageProperties = unsafe { std::mem::zeroed() };
        // Careful: x needs to live long enough
        let x = if let Some(filename) = &value.filename {
            Some(CString::new(filename.as_str())?)
        } else {
            None
        };
        let (filename, bytes_of_filename) = if let Some(ref x) = x {
            (x.as_ptr(), x.to_bytes_with_nul().len())
        } else {
            (null(), 0)
        };

        // Careful: y needs to live long enough
        let y = if let Some(metadata) = &value.external_metadata_json {
            Some(CString::new(metadata.as_str())?)
        } else {
            None
        };
        let (metadata, bytes_of_metadata) = if let Some(ref y) = y {
            (y.as_ptr(), y.to_bytes_with_nul().len())
        } else {
            (null(), 0)
        };

        // This copies the string into a buffer owned by the return value.
        unsafe {
            capi::storage_properties_init(
                &mut out,
                value.first_frame_id,
                filename,
                bytes_of_filename as _,
                metadata,
                bytes_of_metadata as _,
            )
            .ok()?;
        }
        Ok(out)
    }
}

impl Default for capi::StorageProperties {
    fn default() -> Self {
        Self {
            filename: Default::default(),
            first_frame_id: Default::default(),
            external_metadata_json: Default::default(),
        }
    }
}

impl Default for capi::String {
    fn default() -> Self {
        Self {
            str_: null_mut(),
            nbytes: Default::default(),
            is_ref: Default::default(),
        }
    }
}

impl Display for capi::String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = unsafe { CStr::from_ptr(self.str_) }.to_string_lossy();
        write!(f, "{}", s)
    }
}

//     }
// }
