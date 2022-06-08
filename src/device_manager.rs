use std::{ffi::CString, ptr::NonNull, sync::Arc};

use crate::{
    core_runtime,
    device::{DeviceIdentifier, DeviceKind},
    runtime::RawRuntime,
    Status,
};

use anyhow::Result;
use pyo3::prelude::*;

#[pyclass]
pub struct DeviceManager {
    pub(crate) _runtime: Arc<RawRuntime>,
    pub(crate) inner: NonNull<core_runtime::DeviceManager>,
}

unsafe impl Send for DeviceManager {}

#[pymethods]
impl DeviceManager {
    fn devices(&self) -> PyResult<Vec<DeviceIdentifier>> {
        fn get_ident(
            dm: NonNull<core_runtime::DeviceManager>,
            idevice: u32,
        ) -> Result<DeviceIdentifier, PyErr> {
            let mut ident = core_runtime::DeviceIdentifier {
                driver_id: 0,
                device_id: 0,
                kind: core_runtime::DeviceKind_DeviceKind_Unknown,
                name: [0; 256],
            };
            unsafe { core_runtime::device_manager_get(&mut ident, dm.as_ptr(), idevice) }.ok()?;
            Ok(ident.try_into()?)
        }

        let count = unsafe { core_runtime::device_manager_count(self.inner.as_ptr()) };
        (0..count)
            .map(|idevice| get_ident(self.inner, idevice))
            .collect()
    }

    fn select_device(&self, kind: DeviceKind, name: &str) -> PyResult<Option<DeviceIdentifier>> {
        let name = CString::new(name)?;
        let (status,ident) = unsafe {
            let mut ident: core_runtime::DeviceIdentifier = std::mem::zeroed();
            let status=core_runtime::device_manager_select(
                self.inner.as_ptr(),
                kind.into(),
                name.as_ptr(),
                name.as_bytes().len() as _,
                &mut ident,
            ).is_ok();
            (status,ident)
        };
        if status {
            Ok(Some(ident.try_into()?))
        } else {
            Ok(None)
        }
    }
}
