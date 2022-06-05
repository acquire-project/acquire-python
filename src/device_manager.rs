use std::{ffi::CStr, ptr::NonNull, sync::Arc};

use crate::{core_runtime, runtime::RawRuntime, Status};

use anyhow::{Result};
use pyo3::prelude::*;

#[pyclass]
pub struct DeviceManager {
    pub(crate) _runtime: Arc<RawRuntime>,
    pub(crate) inner: NonNull<core_runtime::DeviceManager>,
}

unsafe impl Send for DeviceManager {}

#[pymethods]
impl DeviceManager {
    fn devices(&self) -> PyResult<Vec<String>> {
        fn get_ident(
            dm: NonNull<core_runtime::DeviceManager>,
            idevice: u32,
        ) -> Result<String, PyErr> {
            let mut ident = core_runtime::DeviceIdentifier {
                driver_id: 0,
                device_id: 0,
                kind: core_runtime::DeviceKind_DeviceKind_Unknown,
                name: [0; 256],
            };
            unsafe { core_runtime::device_manager_get(&mut ident, dm.as_ptr(), idevice) }.ok()?;
            let name = unsafe { CStr::from_ptr(&ident.name[0] as *const i8) }
                .to_str()?
                .to_owned();
            Ok(name)
        }

        let count = unsafe { core_runtime::device_manager_count(self.inner.as_ptr()) };
        (0..count)
            .map(|idevice| get_ident(self.inner, idevice))
            .collect()
    }
}
