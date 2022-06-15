use crate::{
    capi,
    components::{macros::impl_plain_old_dict, SampleType, Trigger},
};
use anyhow::anyhow;
use pyo3::prelude::*;
use pyo3::types::PyList;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Clone)]
pub struct CameraProperties {
    #[pyo3(get, set)]
    gain_db: f32,

    #[pyo3(get, set)]
    exposure_time_us: f32,

    #[pyo3(get, set)]
    binning: u8,

    #[pyo3(get, set)]
    pixel_type: SampleType,

    #[pyo3(get, set)]
    offset: (u32, u32),

    #[pyo3(get, set)]
    shape: (u32, u32),

    #[pyo3(get, set)]
    triggers: Py<PyList>
    // triggers: Vec<Trigger>, // FIXME: Should be Py<PyList>
}

impl_plain_old_dict!(CameraProperties);

impl TryFrom<capi::CameraProperties> for CameraProperties {
    type Error = anyhow::Error;

    fn try_from(value: capi::CameraProperties) -> Result<Self, Self::Error> {
        let triggers = Python::with_gil(|py|->PyResult<_> {
            let v:Vec<_>=(0..value.triggers.line_count as usize)
                .map(|i| {
                    let t:Trigger=value.triggers.lines[i].try_into()?;
                    Py::new(py,t)
                }) 
                .collect::<Result<_,_>>()?;
            Ok(PyList::new(py,v).into())
        })?;

        Ok(CameraProperties {
            gain_db: value.gain_dB,
            exposure_time_us: value.exposure_time_us,
            binning: value.binning,
            pixel_type: value.pixel_type.try_into()?,
            offset: (value.offset.x, value.offset.y),
            shape: (value.shape.x, value.shape.y),
            triggers,
        })
    }
}

impl TryFrom<&CameraProperties> for capi::CameraProperties {
    type Error = anyhow::Error;

    fn try_from(src: &CameraProperties) -> Result<Self, Self::Error> {
        let mut dst_triggers: capi::CameraProperties_camera_properties_triggers_s =
            unsafe { std::mem::zeroed() };
        let src_trigger_count=Python::with_gil(|py| {
            src.triggers.as_ref(py).len()
        });
        if src_trigger_count > dst_triggers.lines.len() {
            Err(anyhow!(
                "Expected fewer trigger lines. Require {}<{}",
                src_trigger_count,
                dst_triggers.lines.len()
            ))
        } else {
            let offset = capi::CameraProperties_camera_properties_offset_s {
                x: src.offset.0,
                y: src.offset.1,
            };
            let shape = capi::CameraProperties_camera_properties_shape_s {
                x: src.shape.0,
                y: src.shape.1,
            };
            Python::with_gil(|py| -> PyResult<()> {
                for (src, dst) in src.triggers.as_ref(py).iter().zip(dst_triggers.lines.iter_mut()) {
                    *dst = src.extract::<Trigger>()?.into()
                }
                Ok(())
            })?;
            Ok(capi::CameraProperties {
                gain_dB: src.gain_db,
                exposure_time_us: src.exposure_time_us,
                binning: src.binning,
                pixel_type: src.pixel_type.into(),
                offset,
                shape,
                triggers: dst_triggers,
            })
        }
    }
}

impl Serialize for CameraProperties {

}