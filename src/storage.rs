use pyo3::prelude::*;

use crate::{core_runtime, Status};
use std::ffi::{CStr, CString};

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct StorageProperties {
    #[pyo3(get, set)]
    filename: String,

    #[pyo3(get, set)]
    first_frame_id: u32,
}

impl TryFrom<core_runtime::StorageProperties> for StorageProperties {
    type Error = anyhow::Error;

    fn try_from(value: core_runtime::StorageProperties) -> Result<Self, Self::Error> {
        let filename = unsafe { CStr::from_ptr(value.filename.str_) }
            .to_str()?
            .to_owned();
        Ok(Self {
            filename,
            first_frame_id: value.first_frame_id,
        })
    }
}

impl TryFrom<StorageProperties> for core_runtime::StorageProperties2k {
    type Error = anyhow::Error;

    fn try_from(value: StorageProperties) -> Result<Self, Self::Error> {
        let mut out: core_runtime::StorageProperties2k = unsafe { std::mem::zeroed() };
        let x = CString::new(value.filename)?;
        let x = x.as_c_str();
        unsafe {
            core_runtime::storage_properties_init(
                out.as_mut() as *mut core_runtime::StoragePropertiesOwned,
                std::mem::size_of_val(&out) as _,
                value.first_frame_id,
                x.as_ptr(),
                x.to_bytes_with_nul().len() as _,
            )
            .ok()?;
        }
        Ok(out)
    }


}

impl AsRef<core_runtime::StorageProperties> for core_runtime::StorageProperties2k {
    fn as_ref(&self) -> &core_runtime::StorageProperties {
        unsafe { *(self as *const core_runtime::StorageProperties2k as *const _) }
    }
}


impl AsMut<core_runtime::StoragePropertiesOwned> for core_runtime::StorageProperties2k {
    fn as_mut(&mut self) -> &mut core_runtime::StoragePropertiesOwned {
        unsafe { &mut *(self as *mut core_runtime::StorageProperties2k as *mut _) }
    }
}