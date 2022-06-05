use crate::{
    components::{SampleType, Trigger},
    core_runtime,
};
use anyhow::anyhow;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Default)]
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
    triggers: Vec<Trigger>,
}

#[pymethods]
impl CameraProperties {
    #[new]
    fn new() -> Self {
        Default::default()
    }
}

impl TryFrom<core_runtime::CameraProperties> for CameraProperties {
    type Error = anyhow::Error;

    fn try_from(value: core_runtime::CameraProperties) -> Result<Self, Self::Error> {
        let triggers = (0..value.triggers.line_count as usize)
            .map(|i| value.triggers.lines[i].try_into())
            .collect::<Result<Vec<Trigger>, anyhow::Error>>()?;
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

impl TryInto<core_runtime::CameraProperties> for CameraProperties {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<core_runtime::CameraProperties, Self::Error> {
        let mut triggers: core_runtime::CameraProperties_camera_properties_triggers_s =
            unsafe { std::mem::zeroed() };
        if self.triggers.len() > triggers.lines.len() {
            Err(anyhow!(
                "Expected fewer trigger lines. Require {}<{}",
                self.triggers.len(),
                triggers.lines.len()
            ))
        } else {
            let offset = core_runtime::CameraProperties_camera_properties_offset_s {
                x: self.offset.0,
                y: self.offset.1,
            };
            let shape = core_runtime::CameraProperties_camera_properties_shape_s {
                x: self.shape.0,
                y: self.shape.1,
            };
            for (src, dst) in self.triggers.into_iter().zip(triggers.lines.iter_mut()) {
                *dst = src.into()
            }
            Ok(core_runtime::CameraProperties {
                gain_dB: self.gain_db,
                exposure_time_us: self.exposure_time_us,
                binning: self.binning,
                pixel_type: self.pixel_type.into(),
                offset,
                shape,
                triggers,
            })
        }
    }
}
