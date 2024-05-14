use crate::{
    capi,
    components::{macros::impl_plain_old_dict, Direction, Property, SampleType, Trigger},
};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::{c_char, c_void, CStr};

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

/// CameraCapabilities::OffsetCapabilities
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OffsetCapabilities {
    #[pyo3(get)]
    x: Property,

    #[pyo3(get)]
    y: Property,
}

impl_plain_old_dict!(OffsetCapabilities);

impl Default for OffsetCapabilities {
    fn default() -> Self {
        Self {
            x: Property::default(),
            y: Property::default(),
        }
    }
}

impl TryFrom<capi::CameraPropertyMetadata_camera_properties_metadata_offset_s>
    for OffsetCapabilities
{
    type Error = anyhow::Error;

    fn try_from(
        value: capi::CameraPropertyMetadata_camera_properties_metadata_offset_s,
    ) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|_| -> PyResult<_> {
            Ok(Self {
                x: value.x.try_into()?,
                y: value.y.try_into()?,
            })
        })?)
    }
}

/// CameraCapabilities::ShapeCapabilities
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeCapabilities {
    #[pyo3(get)]
    x: Property,

    #[pyo3(get)]
    y: Property,
}

impl_plain_old_dict!(ShapeCapabilities);

impl Default for ShapeCapabilities {
    fn default() -> Self {
        Self {
            x: Property::default(),
            y: Property::default(),
        }
    }
}

impl TryFrom<capi::CameraPropertyMetadata_camera_properties_metadata_shape_s>
    for ShapeCapabilities
{
    type Error = anyhow::Error;

    fn try_from(
        value: capi::CameraPropertyMetadata_camera_properties_metadata_shape_s,
    ) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|_| -> PyResult<_> {
            Ok(Self {
                x: value.x.try_into()?,
                y: value.y.try_into()?,
            })
        })?)
    }
}

/// CameraCapabilities::DigitalLineCapabilities
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalLineCapabilities {
    #[pyo3(get)]
    line_count: u8,

    #[pyo3(get)]
    names: [String; 8],
}

impl_plain_old_dict!(DigitalLineCapabilities);

impl Default for DigitalLineCapabilities {
    fn default() -> Self {
        Self {
            line_count: Default::default(),
            names: Default::default(),
        }
    }
}

impl TryFrom<capi::CameraPropertyMetadata_CameraPropertyMetadataDigitalLineMetadata>
    for DigitalLineCapabilities
{
    type Error = anyhow::Error;

    fn try_from(
        value: capi::CameraPropertyMetadata_CameraPropertyMetadataDigitalLineMetadata,
    ) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|_| -> PyResult<_> {
            let mut names: [String; 8] = Default::default();
            for (i, name) in value.names.iter().enumerate() {
                let name = unsafe { CStr::from_ptr(name.as_ptr()) }
                    .to_str()?
                    .to_owned();
                names[i] = name;
            }
            Ok(Self {
                line_count: value.line_count,
                names,
            })
        })?)
    }
}

/// CameraCapabilities::TriggerInputOutputCapabilities
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerInputOutputCapabilities {
    #[pyo3(get)]
    input: u8,

    #[pyo3(get)]
    output: u8,
}

impl_plain_old_dict!(TriggerInputOutputCapabilities);

impl Default for TriggerInputOutputCapabilities {
    fn default() -> Self {
        Self {
            input: Default::default(),
            output: Default::default(),
        }
    }
}

impl TryFrom<capi::CameraPropertyMetadata_CameraPropertiesTriggerMetadata_camera_properties_metadata_trigger_capabilities_s> for TriggerInputOutputCapabilities {
    type Error = anyhow::Error;

    fn try_from(
        value: capi::CameraPropertyMetadata_CameraPropertiesTriggerMetadata_camera_properties_metadata_trigger_capabilities_s,
    ) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|_| -> PyResult<_> {
            Ok(Self {
                input: value.input,
                output: value.output,
            })
        })?)
    }
}

/// CameraCapabilities::TriggerCapabilities
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerCapabilities {
    #[pyo3(get)]
    acquisition_start: Py<TriggerInputOutputCapabilities>,

    #[pyo3(get)]
    exposure: Py<TriggerInputOutputCapabilities>,

    #[pyo3(get)]
    frame_start: Py<TriggerInputOutputCapabilities>,
}

impl_plain_old_dict!(TriggerCapabilities);

impl Default for TriggerCapabilities {
    fn default() -> Self {
        let (acquisition_start, exposure, frame_start) = Python::with_gil(|py| {
            (
                Py::new(py, TriggerInputOutputCapabilities::default()).unwrap(),
                Py::new(py, TriggerInputOutputCapabilities::default()).unwrap(),
                Py::new(py, TriggerInputOutputCapabilities::default()).unwrap(),
            )
        });
        Self {
            acquisition_start,
            exposure,
            frame_start,
        }
    }
}

impl TryFrom<capi::CameraPropertyMetadata_CameraPropertiesTriggerMetadata> for TriggerCapabilities {
    type Error = anyhow::Error;

    fn try_from(
        value: capi::CameraPropertyMetadata_CameraPropertiesTriggerMetadata,
    ) -> Result<Self, Self::Error> {
        let (acquisition_start, exposure, frame_start) = Python::with_gil(|py| -> PyResult<_> {
            let acquisition_start: TriggerInputOutputCapabilities =
                value.acquisition_start.try_into()?;
            let exposure: TriggerInputOutputCapabilities = value.exposure.try_into()?;
            let frame_start: TriggerInputOutputCapabilities = value.frame_start.try_into()?;
            Ok((
                Py::new(py, acquisition_start)?,
                Py::new(py, exposure)?,
                Py::new(py, frame_start)?,
            ))
        })?;
        Ok(Python::with_gil(|_| -> PyResult<_> {
            Ok(Self {
                acquisition_start,
                exposure,
                frame_start,
            })
        })?)
    }
}

/// CameraCapabilities
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraCapabilities {
    #[pyo3(get)]
    exposure_time_us: Property,

    #[pyo3(get)]
    line_interval_us: Property,

    #[pyo3(get)]
    readout_direction: Property,

    #[pyo3(get)]
    binning: Property,

    #[pyo3(get)]
    offset: Py<OffsetCapabilities>,

    #[pyo3(get)]
    shape: Py<ShapeCapabilities>,

    #[pyo3(get)]
    supported_pixel_types: Vec<SampleType>,

    #[pyo3(get)]
    digital_lines: Py<DigitalLineCapabilities>,

    #[pyo3(get)]
    triggers: Py<TriggerCapabilities>,
}

impl_plain_old_dict!(CameraCapabilities);

impl Default for CameraCapabilities {
    fn default() -> Self {
        let (offset, shape, digital_lines, triggers) = Python::with_gil(|py| {
            (
                Py::new(py, OffsetCapabilities::default()).unwrap(),
                Py::new(py, ShapeCapabilities::default()).unwrap(),
                Py::new(py, DigitalLineCapabilities::default()).unwrap(),
                Py::new(py, TriggerCapabilities::default()).unwrap(),
            )
        });
        Self {
            exposure_time_us: Property::default(),
            line_interval_us: Property::default(),
            readout_direction: Property::default(),
            binning: Property::default(),
            offset,
            shape,
            supported_pixel_types: Default::default(),
            digital_lines,
            triggers,
        }
    }
}

impl TryFrom<capi::CameraPropertyMetadata> for CameraCapabilities {
    type Error = anyhow::Error;

    fn try_from(value: capi::CameraPropertyMetadata) -> Result<Self, Self::Error> {
        let (offset, shape, digital_lines, triggers) = Python::with_gil(|py| -> PyResult<_> {
            let offset: OffsetCapabilities = value.offset.try_into()?;
            let shape: ShapeCapabilities = value.shape.try_into()?;
            let digital_lines: DigitalLineCapabilities = value.digital_lines.try_into()?;
            let triggers: TriggerCapabilities = value.triggers.try_into()?;
            Ok((
                Py::new(py, offset)?,
                Py::new(py, shape)?,
                Py::new(py, digital_lines)?,
                Py::new(py, triggers)?,
            ))
        })?;

        let mut supported_pixel_types: Vec<SampleType> = Default::default();
        for (i, &x) in SampleType::iter().enumerate() {
            if value.supported_pixel_types & (1 << i) != 0 {
                supported_pixel_types.push(x);
            }
        }

        Ok(Self {
            exposure_time_us: value.exposure_time_us.try_into()?,
            line_interval_us: value.line_interval_us.try_into()?,
            readout_direction: value.readout_direction.try_into()?,
            binning: value.binning.try_into()?,
            offset,
            shape,
            supported_pixel_types,
            digital_lines,
            triggers,
        })
    }
}

/// capi

impl Default for capi::CameraPropertyMetadata_camera_properties_metadata_offset_s {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl TryFrom<&OffsetCapabilities>
    for capi::CameraPropertyMetadata_camera_properties_metadata_offset_s
{
    type Error = anyhow::Error;

    fn try_from(value: &OffsetCapabilities) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|_| -> PyResult<_> {
            Ok(Self {
                x: (&value.x).try_into()?,
                y: (&value.y).try_into()?,
            })
        })?)
    }
}

impl Default for capi::CameraPropertyMetadata_camera_properties_metadata_shape_s {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl TryFrom<&ShapeCapabilities>
    for capi::CameraPropertyMetadata_camera_properties_metadata_shape_s
{
    type Error = anyhow::Error;

    fn try_from(value: &ShapeCapabilities) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|_| -> PyResult<_> {
            Ok(Self {
                x: (&value.x).try_into()?,
                y: (&value.y).try_into()?,
            })
        })?)
    }
}

impl Default for capi::CameraPropertyMetadata_CameraPropertyMetadataDigitalLineMetadata {
    fn default() -> Self {
        Self {
            line_count: Default::default(),
            names: [
                [0; 64], [0; 64], [0; 64], [0; 64], [0; 64], [0; 64], [0; 64], [0; 64],
            ],
        }
    }
}

impl TryFrom<&DigitalLineCapabilities>
    for capi::CameraPropertyMetadata_CameraPropertyMetadataDigitalLineMetadata
{
    type Error = anyhow::Error;

    fn try_from(value: &DigitalLineCapabilities) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|_| -> PyResult<_> {
            let mut names: [[c_char; 64]; 8] = [
                [0; 64], [0; 64], [0; 64], [0; 64], [0; 64], [0; 64], [0; 64], [0; 64],
            ];
            for (i, name) in value.names.iter().enumerate() {
                let name = std::ffi::CString::new(name.as_str())?;
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        name.as_ptr() as *const c_void,
                        names[i].as_mut_ptr() as *mut c_void,
                        64,
                    );
                }
            }
            Ok(Self {
                line_count: value.line_count,
                names,
            })
        })?)
    }
}

impl Default for capi::CameraPropertyMetadata_CameraPropertiesTriggerMetadata_camera_properties_metadata_trigger_capabilities_s {
    fn default() -> Self {
        Self {
            input: Default::default(),
            output: Default::default(),
        }
    }
}

impl TryFrom<&TriggerInputOutputCapabilities> for capi::CameraPropertyMetadata_CameraPropertiesTriggerMetadata_camera_properties_metadata_trigger_capabilities_s {
    type Error = anyhow::Error;

    fn try_from(
        value: &TriggerInputOutputCapabilities,
    ) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|_| -> PyResult<_> {
            Ok(Self {
                input: value.input,
                output: value.output,
            })
        })?)
    }
}

impl Default for capi::CameraPropertyMetadata_CameraPropertiesTriggerMetadata {
    fn default() -> Self {
        Self {
            acquisition_start: Default::default(),
            exposure: Default::default(),
            frame_start: Default::default(),
        }
    }
}

impl TryFrom<&TriggerCapabilities>
    for capi::CameraPropertyMetadata_CameraPropertiesTriggerMetadata
{
    type Error = anyhow::Error;

    fn try_from(value: &TriggerCapabilities) -> Result<Self, Self::Error> {
        let acquisition_start = Python::with_gil(|py| -> PyResult<_> {
            let acquisition_start: TriggerInputOutputCapabilities =
                value.acquisition_start.extract(py)?;
            Ok(acquisition_start)
        })?;

        let exposure = Python::with_gil(|py| -> PyResult<_> {
            let exposure: TriggerInputOutputCapabilities = value.exposure.extract(py)?;
            Ok(exposure)
        })?;

        let frame_start = Python::with_gil(|py| -> PyResult<_> {
            let frame_start: TriggerInputOutputCapabilities = value.frame_start.extract(py)?;
            Ok(frame_start)
        })?;

        Ok(Self {
            acquisition_start: (&acquisition_start).try_into()?,
            exposure: (&exposure).try_into()?,
            frame_start: (&frame_start).try_into()?,
        })
    }
}

impl Default for capi::CameraPropertyMetadata {
    fn default() -> Self {
        Self {
            exposure_time_us: Default::default(),
            line_interval_us: Default::default(),
            readout_direction: Default::default(),
            binning: Default::default(),
            offset: Default::default(),
            shape: Default::default(),
            supported_pixel_types: Default::default(),
            digital_lines: Default::default(),
            triggers: Default::default(),
        }
    }
}

impl TryFrom<&CameraCapabilities> for capi::CameraPropertyMetadata {
    type Error = anyhow::Error;

    fn try_from(src: &CameraCapabilities) -> Result<Self, Self::Error> {
        let offset = Python::with_gil(|py| -> PyResult<_> {
            let offset: OffsetCapabilities = src.offset.extract(py)?;
            Ok(offset)
        })?;

        let shape = Python::with_gil(|py| -> PyResult<_> {
            let shape: ShapeCapabilities = src.shape.extract(py)?;
            Ok(shape)
        })?;

        let digital_lines = Python::with_gil(|py| -> PyResult<_> {
            let digital_lines: DigitalLineCapabilities = src.digital_lines.extract(py)?;
            Ok(digital_lines)
        })?;

        let triggers = Python::with_gil(|py| -> PyResult<_> {
            let triggers: TriggerCapabilities = src.triggers.extract(py)?;
            Ok(triggers)
        })?;

        let mut supported_pixel_types: u64 = 0;
        for &x in &src.supported_pixel_types {
            supported_pixel_types |= 1 << x as u64;
        }

        Ok(Self {
            exposure_time_us: (&src.exposure_time_us).try_into()?,
            line_interval_us: (&src.line_interval_us).try_into()?,
            readout_direction: (&src.readout_direction).try_into()?,
            binning: (&src.binning).try_into()?,
            offset: (&offset).try_into()?,
            shape: (&shape).try_into()?,
            supported_pixel_types,
            digital_lines: (&digital_lines).try_into()?,
            triggers: (&triggers).try_into()?,
        })
    }
}
