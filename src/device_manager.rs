use std::{ffi::CString, ptr::{NonNull, null}, sync::Arc};

use crate::{
    capi,
    device::{DeviceIdentifier, DeviceKind},
    runtime::RawRuntime,
    Status,
};

use anyhow::Result;
use pyo3::prelude::*;

#[pyclass]
pub struct DeviceManager {
    pub(crate) _runtime: Arc<RawRuntime>,
    pub(crate) inner: NonNull<capi::DeviceManager>,
}

unsafe impl Send for DeviceManager {}

#[pymethods]
impl DeviceManager {
    fn devices(&self) -> PyResult<Vec<DeviceIdentifier>> {
        fn get_ident(
            dm: NonNull<capi::DeviceManager>,
            idevice: u32,
        ) -> Result<DeviceIdentifier, PyErr> {
            let mut ident = capi::DeviceIdentifier {
                driver_id: 0,
                device_id: 0,
                kind: capi::DeviceKind_DeviceKind_Unknown,
                name: [0; 256],
            };
            unsafe { capi::device_manager_get(&mut ident, dm.as_ptr(), idevice) }.ok()?;
            Ok(ident.try_into()?)
        }

        let count = unsafe { capi::device_manager_count(self.inner.as_ptr()) };
        (0..count)
            .map(|idevice| get_ident(self.inner, idevice))
            .collect()
    }

    fn select(&self, kind: DeviceKind, name: Option<&str>) -> PyResult<Option<DeviceIdentifier>> {
        let (name,bytes,_owned_name)=if let Some(name)=name {
            let name_ = CString::new(name)?;
            (name_.as_ptr(),name_.as_bytes().len(),Some(name_))
        } else {
            (null(),0,None)
        };
        let (status, ident) = unsafe {
            let mut ident: capi::DeviceIdentifier = std::mem::zeroed();
            let status = capi::device_manager_select(
                self.inner.as_ptr(),
                kind.into(),
                name,
                bytes as _,
                &mut ident,
            )
            .is_ok();
            (status, ident)
        };
        if status {
            Ok(Some(ident.try_into()?))
        } else {
            Ok(None)
        }
    }
}
