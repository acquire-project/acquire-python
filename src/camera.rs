use crate::{
    capi,
    components::{macros::impl_plain_old_dict, Direction, SampleType, Trigger},
};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct InputTriggers {
    #[pyo3(get, set)]
    acquisition_start: Trigger,
    #[pyo3(get, set)]
    frame_start: Trigger,
    #[pyo3(get, set)]
    exposure: Trigger,
}

impl_plain_old_dict!(InputTriggers);

impl AsRef<InputTriggers> for InputTriggers {
    fn as_ref(&self) -> &InputTriggers {
        self
    }
}

impl TryFrom<capi::CameraProperties_camera_properties_input_triggers_s> for InputTriggers {
    type Error = anyhow::Error;

    fn try_from(
        value: capi::CameraProperties_camera_properties_input_triggers_s,
    ) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|_| -> PyResult<_> {
            Ok(Self {
                acquisition_start: value.acquisition_start.try_into()?,
                frame_start: value.frame_start.try_into()?,
                exposure: value.exposure.try_into()?,
            })
        })?)
    }
}

impl TryFrom<&InputTriggers> for capi::CameraProperties_camera_properties_input_triggers_s {
    type Error = anyhow::Error;

    fn try_from(value: &InputTriggers) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|_| -> PyResult<_> {
            Ok(Self {
                acquisition_start: (&value.acquisition_start).into(),
                frame_start: (&value.frame_start).into(),
                exposure: (&value.exposure).into(),
            })
        })?)
    }
}

#[pyclass]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OutputTriggers {
    #[pyo3(get, set)]
    exposure: Trigger,
    #[pyo3(get, set)]
    frame_start: Trigger,
    #[pyo3(get, set)]
    trigger_wait: Trigger,
}

impl_plain_old_dict!(OutputTriggers);

impl AsRef<OutputTriggers> for OutputTriggers {
    fn as_ref(&self) -> &OutputTriggers {
        self
    }
}

impl TryFrom<capi::CameraProperties_camera_properties_output_triggers_s> for OutputTriggers {
    type Error = anyhow::Error;

    fn try_from(
        value: capi::CameraProperties_camera_properties_output_triggers_s,
    ) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|_| -> PyResult<_> {
            Ok(Self {
                exposure: value.exposure.try_into()?,
                frame_start: value.frame_start.try_into()?,
                trigger_wait: value.trigger_wait.try_into()?,
            })
        })?)
    }
}

impl TryFrom<&OutputTriggers> for capi::CameraProperties_camera_properties_output_triggers_s {
    type Error = anyhow::Error;

    fn try_from(value: &OutputTriggers) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|_| -> PyResult<_> {
            Ok(Self {
                exposure: (&value.exposure).into(),
                frame_start: (&value.frame_start).into(),
                trigger_wait: (&value.trigger_wait).into(),
            })
        })?)
    }
}

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraProperties {
    #[pyo3(get, set)]
    exposure_time_us: f32,

    #[pyo3(get, set)]
    line_interval_us: f32,

    #[pyo3(get, set)]
    readout_direction: Direction,

    #[pyo3(get, set)]
    binning: u8,

    #[pyo3(get, set)]
    pixel_type: SampleType,

    #[pyo3(get, set)]
    offset: (u32, u32),

    #[pyo3(get, set)]
    shape: (u32, u32),

    #[pyo3(get, set)]
    input_triggers: Py<InputTriggers>,

    #[pyo3(get, set)]
    output_triggers: Py<OutputTriggers>,
}

impl_plain_old_dict!(CameraProperties);

impl Default for CameraProperties {
    fn default() -> Self {
        let (input_triggers, output_triggers) = Python::with_gil(|py| {
            (
                Py::new(py, InputTriggers::default()).unwrap(),
                Py::new(py, OutputTriggers::default()).unwrap(),
            )
        });
        Self {
            exposure_time_us: Default::default(),
            line_interval_us: Default::default(),
            readout_direction: Default::default(),
            binning: Default::default(),
            pixel_type: Default::default(),
            offset: Default::default(),
            shape: (1920, 1080),
            input_triggers,
            output_triggers,
        }
    }
}

impl TryFrom<capi::CameraProperties> for CameraProperties {
    type Error = anyhow::Error;

    fn try_from(value: capi::CameraProperties) -> Result<Self, Self::Error> {
        let (input_triggers, output_triggers) = Python::with_gil(|py| -> PyResult<_> {
            let tr_in: InputTriggers = value.input_triggers.try_into()?;
            let tr_out: OutputTriggers = value.output_triggers.try_into()?;
            Ok((Py::new(py, tr_in)?, Py::new(py, tr_out)?))
        })?;
        Ok(CameraProperties {
            exposure_time_us: value.exposure_time_us,
            line_interval_us: value.line_interval_us,
            readout_direction: value.readout_direction.try_into()?,
            binning: value.binning,
            pixel_type: value.pixel_type.try_into()?,
            offset: (value.offset.x, value.offset.y),
            shape: (value.shape.x, value.shape.y),
            input_triggers,
            output_triggers,
        })
    }
}

impl TryFrom<&CameraProperties> for capi::CameraProperties {
    type Error = anyhow::Error;

    fn try_from(src: &CameraProperties) -> Result<Self, Self::Error> {
        let offset = capi::CameraProperties_camera_properties_offset_s {
            x: src.offset.0,
            y: src.offset.1,
        };
        let shape = capi::CameraProperties_camera_properties_shape_s {
            x: src.shape.0,
            y: src.shape.1,
        };
        let (input_triggers, output_triggers) = Python::with_gil(|py| -> PyResult<_> {
            let input_triggers: InputTriggers = src.input_triggers.extract(py)?;
            let output_triggers: OutputTriggers = src.output_triggers.extract(py)?;
            Ok((input_triggers, output_triggers))
        })?;
        Ok(capi::CameraProperties {
            exposure_time_us: src.exposure_time_us,
            line_interval_us: src.line_interval_us,
            readout_direction: src.readout_direction.into(),
            binning: src.binning,
            pixel_type: src.pixel_type.into(),
            offset,
            shape,
            input_triggers: (&input_triggers).try_into()?,
            output_triggers: (&output_triggers).try_into()?,
        })
    }
}

impl Default for capi::CameraProperties_camera_properties_input_triggers_s {
    fn default() -> Self {
        Self {
            acquisition_start: Default::default(),
            exposure: Default::default(),
            frame_start: Default::default(),
        }
    }
}

impl Default for capi::CameraProperties_camera_properties_output_triggers_s {
    fn default() -> Self {
        Self {
            exposure: Default::default(),
            frame_start: Default::default(),
            trigger_wait: Default::default(),
        }
    }
}

impl Default for capi::CameraProperties {
    fn default() -> Self {
        Self {
            exposure_time_us: Default::default(),
            line_interval_us: Default::default(),
            readout_direction: capi::Direction_Direction_Forward,
            binning: 1,
            pixel_type: capi::SampleType_SampleType_u16,
            offset: Default::default(),
            shape: Default::default(),
            input_triggers: Default::default(),
            output_triggers: Default::default(),
        }
    }
}

impl Default for capi::CameraProperties_camera_properties_offset_s {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl Default for capi::CameraProperties_camera_properties_shape_s {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}
