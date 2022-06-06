use std::ffi::CStr;

use log::debug;
use pyo3::prelude::*;

use crate::core_runtime;
use anyhow::Result;

impl core_runtime::DeviceIdentifier {
    pub(crate) fn name_as_string(&self)->Result<String> {
        debug!("Converting device identifier name to string.");
        Ok(unsafe{CStr::from_ptr(self.name.as_ptr())}.to_str()?.to_owned())
    }
}

// #[pyclass]
// #[derive(Debug, Clone, Default)]
// struct DeviceIdentifier {

// }