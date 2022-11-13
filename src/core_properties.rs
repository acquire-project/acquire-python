use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    camera::CameraProperties, capi, components::macros::impl_plain_old_dict,
    device::DeviceIdentifier, signals::SignalProperties, stage_axis::StageAxisProperties,
    storage::StorageProperties,
};

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct Camera {
    #[pyo3(get, set)]
    identifier: Option<Py<DeviceIdentifier>>,

    #[pyo3(get, set)]
    settings: Py<CameraProperties>,
}

impl Default for Camera {
    fn default() -> Self {
        Python::with_gil(|py| Self {
            identifier: Some(Py::new(py, DeviceIdentifier::default()).unwrap()),
            settings: Py::new(py, CameraProperties::default()).unwrap(),
        })
    }
}

impl_plain_old_dict!(Camera);

impl AsRef<Camera> for Camera {
    fn as_ref(&self) -> &Camera {
        self
    }
}

impl TryFrom<capi::CpxProperties_cpx_properties_video_s_cpx_properties_camera_s> for Camera {
    type Error = anyhow::Error;

    fn try_from(
        value: capi::CpxProperties_cpx_properties_video_s_cpx_properties_camera_s,
    ) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let identifier: DeviceIdentifier = value.identifier.try_into()?;
            let settings: CameraProperties = value.settings.try_into()?;
            Ok(Self {
                identifier: Some(Py::new(py, identifier)?),
                settings: Py::new(py, settings)?,
            })
        })?)
    }
}

impl TryFrom<&Camera> for capi::CpxProperties_cpx_properties_video_s_cpx_properties_camera_s {
    type Error = anyhow::Error;

    fn try_from(value: &Camera) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let identifier: DeviceIdentifier = match &value.identifier {
                None => DeviceIdentifier::none(),
                Some(inner) => inner.extract(py)?,
            };
            let settings: CameraProperties = value.settings.extract(py)?;
            Ok(Self {
                identifier: (&identifier).try_into()?,
                settings: (&settings).try_into()?,
            })
        })?)
    }
}

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    #[pyo3(get, set)]
    identifier: Option<Py<DeviceIdentifier>>,

    #[pyo3(get, set)]
    settings: Py<StorageProperties>,

    write_delay_ms: f32,
}

// FIXME: (nclack) be consistent about "settings" vs "properties" vs "configuration"

impl_plain_old_dict!(@out Storage);

impl Default for Storage {
    fn default() -> Self {
        Python::with_gil(|py| Self {
            identifier: Some(Py::new(py, DeviceIdentifier::default()).unwrap()),
            settings: Py::new(py, StorageProperties::default()).unwrap(),
            write_delay_ms: Default::default(),
        })
    }
}

impl TryFrom<capi::CpxProperties_cpx_properties_video_s_cpx_properties_storage_s> for Storage {
    type Error = anyhow::Error;

    fn try_from(
        value: capi::CpxProperties_cpx_properties_video_s_cpx_properties_storage_s,
    ) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let identifier: DeviceIdentifier = value.identifier.try_into()?;
            let settings: StorageProperties = value.settings.try_into()?;
            Ok(Self {
                identifier: Some(Py::new(py, identifier)?),
                settings: Py::new(py, settings)?,
                write_delay_ms: value.write_delay_ms,
            })
        })?)
    }
}

impl TryFrom<&Storage> for capi::CpxProperties_cpx_properties_video_s_cpx_properties_storage_s {
    type Error = anyhow::Error;

    fn try_from(value: &Storage) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let identifier: DeviceIdentifier = match &value.identifier {
                None => DeviceIdentifier::none(),
                Some(inner) => inner.extract(py)?,
            };
            let settings: StorageProperties = value.settings.extract(py)?;
            Ok(Self {
                identifier: (&identifier).try_into()?,
                settings: (&settings).try_into()?,
                write_delay_ms: value.write_delay_ms,
            })
        })?)
    }
}

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageAxis {
    #[pyo3(get, set)]
    identifier: Option<Py<DeviceIdentifier>>,

    #[pyo3(get, set)]
    settings: Py<StageAxisProperties>,
}

impl_plain_old_dict!(@out StageAxis);

impl AsRef<StageAxis> for StageAxis {
    fn as_ref(&self) -> &StageAxis {
        self
    }
}

impl Default for StageAxis {
    fn default() -> Self {
        Python::with_gil(|py| Self {
            identifier: Some(Py::new(py, DeviceIdentifier::default()).unwrap()),
            settings: Py::new(py, StageAxisProperties::default()).unwrap(),
        })
    }
}

impl TryFrom<capi::CpxProperties_cpx_properties_stages_s> for StageAxis {
    type Error = anyhow::Error;

    fn try_from(value: capi::CpxProperties_cpx_properties_stages_s) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let identifier: DeviceIdentifier = value.identifier.try_into()?;
            let settings: StageAxisProperties = value.settings.try_into()?;
            Ok(Self {
                identifier: Some(Py::new(py, identifier)?),
                settings: Py::new(py, settings)?,
            })
        })?)
    }
}

impl TryFrom<&StageAxis> for capi::CpxProperties_cpx_properties_stages_s {
    type Error = anyhow::Error;

    fn try_from(value: &StageAxis) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let identifier: DeviceIdentifier = match &value.identifier {
                None => DeviceIdentifier::none(),
                Some(inner) => inner.extract(py)?,
            };
            let settings: StageAxisProperties = value.settings.extract(py)?;
            Ok(Self {
                identifier: (&identifier).try_into()?,
                settings: (&settings).into(),
            })
        })?)
    }
}

#[pyclass]
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Signals {
    #[pyo3(get, set)]
    identifier: Option<DeviceIdentifier>,

    #[pyo3(get, set)]
    settings: SignalProperties,
}

impl_plain_old_dict!(@out Signals);

impl AsRef<Signals> for Signals {
    fn as_ref(&self) -> &Signals {
        self
    }
}

impl TryFrom<capi::CpxProperties_cpx_properties_signals_s> for Signals {
    type Error = anyhow::Error;

    fn try_from(value: capi::CpxProperties_cpx_properties_signals_s) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: value.identifier.try_into().ok(),
            settings: value.settings.try_into()?,
        })
    }
}

impl TryFrom<&Signals> for capi::CpxProperties_cpx_properties_signals_s {
    type Error = anyhow::Error;

    fn try_from(value: &Signals) -> Result<Self, Self::Error> {
        let identifier = value
            .identifier
            .as_ref()
            .map(|e| e.try_into())
            .unwrap_or_else(|| Ok(Default::default()))?;
        Ok(Self {
            identifier,
            settings: (&value.settings).try_into()?,
        })
    }
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct VideoStream {
    #[pyo3(get, set)]
    camera: Py<Camera>,

    #[pyo3(get, set)]
    storage: Py<Storage>,

    #[pyo3(get, set)]
    max_frame_count: u64,

    #[pyo3(get, set)]
    frame_average_count: u32,
}

impl_plain_old_dict!(VideoStream);

impl Default for VideoStream {
    fn default() -> Self {
        Python::with_gil(|py| Self {
            camera: Py::new(py, Camera::default()).unwrap(),
            storage: Py::new(py, Storage::default()).unwrap(),
            max_frame_count: Default::default(),
            frame_average_count: Default::default(),
        })
    }
}

impl TryFrom<capi::CpxProperties_cpx_properties_video_s> for VideoStream {
    type Error = anyhow::Error;

    fn try_from(value: capi::CpxProperties_cpx_properties_video_s) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let camera: Camera = value.camera.try_into()?;
            let storage: Storage = value.storage.try_into()?;

            Ok(Self {
                camera: Py::new(py, camera)?,
                storage: Py::new(py, storage)?,
                max_frame_count: value.max_frame_count,
                frame_average_count: value.frame_average_count,
            })
        })?)
    }
}

impl TryFrom<&VideoStream> for capi::CpxProperties_cpx_properties_video_s {
    type Error = anyhow::Error;

    fn try_from(value: &VideoStream) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let camera: Camera = value.camera.extract(py)?;
            let camera = (&camera).try_into()?;
            let storage: Storage = value.storage.extract(py)?;
            let storage = (&storage).try_into()?;
            let out = Ok(Self {
                camera,
                storage,
                max_frame_count: value.max_frame_count,
                frame_average_count: value.frame_average_count,
            });
            out
        })?)
    }
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct Properties {
    #[pyo3(get, set)]
    video: (Py<VideoStream>, Py<VideoStream>), // TODO: should be List of VideoStream? Are there ownership/reference problems?

    #[pyo3(get, set)]
    stages: (Py<StageAxis>, Py<StageAxis>, Py<StageAxis>), // FIXME: (nclack) this isn't ideal, should be PyList

    #[pyo3(get, set)]
    signals: Py<Signals>,
}

impl Default for Properties {
    fn default() -> Self {
        Python::with_gil(|py| Self {
            video: (
                Py::new(py, VideoStream::default()).unwrap(),
                Py::new(py, VideoStream::default()).unwrap(),
            ),
            stages: (
                Py::new(py, StageAxis::default()).unwrap(),
                Py::new(py, StageAxis::default()).unwrap(),
                Py::new(py, StageAxis::default()).unwrap(),
            ),
            signals: Py::new(py, Signals::default()).unwrap(),
        })
    }
}

#[pymethods]
impl Properties {
    #[new]
    #[args(kwargs = "**")]
    fn __new__(kwargs: Option<&pyo3::types::PyDict>) -> anyhow::Result<Self> {
        if let Some(kwargs) = kwargs {
            Ok(pythonize::depythonize(kwargs)?)
        } else {
            Ok(Default::default())
        }
    }

    fn dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        Ok(pythonize::pythonize(py, self)?)
    }

    fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
        let obj = pythonize::pythonize(py, self)?;
        let obj = obj.as_ref(py).downcast::<pyo3::types::PyDict>()?;
        let args: String = obj
            .iter()
            .map(|(k, v)| format!("{}='{}'", k, v))
            .reduce(|acc, e| format!("{},{}", acc, e))
            .unwrap_or(String::new());

        Ok(format!("Properties({})", args))
    }
}

impl TryFrom<&capi::CpxProperties> for Properties {
    type Error = anyhow::Error;

    fn try_from(value: &capi::CpxProperties) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let video_streams: (VideoStream, VideoStream) =
                (value.video[0].try_into()?, value.video[1].try_into()?);
            let video = (Py::new(py, video_streams.0)?, Py::new(py, video_streams.1)?);

            let stages: (StageAxis, StageAxis, StageAxis) = (
                value.stages[0].try_into()?,
                value.stages[1].try_into()?,
                value.stages[2].try_into()?,
            );
            let stages = (
                Py::new(py, stages.0)?,
                Py::new(py, stages.1)?,
                Py::new(py, stages.2)?,
            );
            let signals: Signals = value.signals.try_into()?;
            Ok(Self {
                video,
                stages,
                signals: Py::new(py, signals)?,
            })
        })?)
    }
}

impl TryFrom<&Properties> for capi::CpxProperties {
    type Error = anyhow::Error;

    fn try_from(value: &Properties) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let video: (VideoStream, VideoStream) =
                (value.video.0.extract(py)?, value.video.1.extract(py)?);
            let video = [(&video.0).try_into()?, (&video.1).try_into()?];

            let stages: (StageAxis, StageAxis, StageAxis) = (
                value.stages.0.extract(py)?,
                value.stages.1.extract(py)?,
                value.stages.2.extract(py)?,
            );
            let stages = [
                (&stages.0).try_into()?,
                (&stages.1).try_into()?,
                (&stages.2).try_into()?,
            ];
            let signals: Signals = value.signals.extract(py)?;
            let signals = (&signals).try_into()?;
            let out = Ok(capi::CpxProperties {
                video,
                stages,
                signals,
            });
            out
        })?)
    }
}

// The main concern here is `storage.settings.filename.str`
// Specifically it needs to remain pinned during the call
// to Runtime::set_configuration().
unsafe impl Send for capi::CpxProperties {}

impl Default for capi::CpxProperties {
    fn default() -> Self {
        Self {
            video: Default::default(),
            stages: Default::default(),
            signals: Default::default(),
        }
    }
}

impl Default for capi::CpxProperties_cpx_properties_video_s {
    fn default() -> Self {
        Self {
            camera: Default::default(),
            storage: Default::default(),
            max_frame_count: Default::default(),
            frame_average_count: Default::default(),
        }
    }
}

impl Default for capi::CpxProperties_cpx_properties_video_s_cpx_properties_camera_s {
    fn default() -> Self {
        Self {
            identifier: Default::default(),
            settings: Default::default(),
        }
    }
}

impl Default for capi::CpxProperties_cpx_properties_video_s_cpx_properties_storage_s {
    fn default() -> Self {
        Self {
            identifier: Default::default(),
            settings: Default::default(),
            write_delay_ms: Default::default(),
        }
    }
}

impl Default for capi::CpxProperties_cpx_properties_stages_s {
    fn default() -> Self {
        Self {
            identifier: Default::default(),
            settings: Default::default(),
        }
    }
}

impl Default for capi::CpxProperties_cpx_properties_signals_s {
    fn default() -> Self {
        Self {
            identifier: Default::default(),
            settings: Default::default(),
        }
    }
}
