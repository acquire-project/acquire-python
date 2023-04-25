use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    camera::CameraProperties, capi, components::macros::impl_plain_old_dict,
    device::DeviceIdentifier, storage::StorageProperties,
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

impl TryFrom<capi::AcquireProperties_aq_properties_video_s_aq_properties_camera_s> for Camera {
    type Error = anyhow::Error;

    fn try_from(
        value: capi::AcquireProperties_aq_properties_video_s_aq_properties_camera_s,
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

impl TryFrom<&Camera> for capi::AcquireProperties_aq_properties_video_s_aq_properties_camera_s {
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

impl TryFrom<capi::AcquireProperties_aq_properties_video_s_aq_properties_storage_s> for Storage {
    type Error = anyhow::Error;

    fn try_from(
        value: capi::AcquireProperties_aq_properties_video_s_aq_properties_storage_s,
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

impl TryFrom<&Storage> for capi::AcquireProperties_aq_properties_video_s_aq_properties_storage_s {
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

impl TryFrom<capi::AcquireProperties_aq_properties_video_s> for VideoStream {
    type Error = anyhow::Error;

    fn try_from(value: capi::AcquireProperties_aq_properties_video_s) -> Result<Self, Self::Error> {
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

impl TryFrom<&VideoStream> for capi::AcquireProperties_aq_properties_video_s {
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
}

impl Default for Properties {
    fn default() -> Self {
        Python::with_gil(|py| Self {
            video: (
                Py::new(py, VideoStream::default()).unwrap(),
                Py::new(py, VideoStream::default()).unwrap(),
            ),
        })
    }
}

#[pymethods]
impl Properties {
    #[new]
    #[pyo3(signature = (**kwargs))]
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

impl TryFrom<&capi::AcquireProperties> for Properties {
    type Error = anyhow::Error;

    fn try_from(value: &capi::AcquireProperties) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let video_streams: (VideoStream, VideoStream) =
                (value.video[0].try_into()?, value.video[1].try_into()?);
            let video = (Py::new(py, video_streams.0)?, Py::new(py, video_streams.1)?);

            Ok(Self { video })
        })?)
    }
}

impl TryFrom<&Properties> for capi::AcquireProperties {
    type Error = anyhow::Error;

    fn try_from(value: &Properties) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let video: (VideoStream, VideoStream) =
                (value.video.0.extract(py)?, value.video.1.extract(py)?);
            let video = [(&video.0).try_into()?, (&video.1).try_into()?];

            let out = Ok(capi::AcquireProperties { video });
            out
        })?)
    }
}

// The main concern here is `storage.settings.filename.str`
// Specifically it needs to remain pinned during the call
// to Runtime::set_configuration().
unsafe impl Send for capi::AcquireProperties {}

impl Default for capi::AcquireProperties {
    fn default() -> Self {
        Self {
            video: Default::default(),
        }
    }
}

impl Default for capi::AcquireProperties_aq_properties_video_s {
    fn default() -> Self {
        Self {
            camera: Default::default(),
            storage: Default::default(),
            max_frame_count: Default::default(),
            frame_average_count: Default::default(),
        }
    }
}

impl Default for capi::AcquireProperties_aq_properties_video_s_aq_properties_camera_s {
    fn default() -> Self {
        Self {
            identifier: Default::default(),
            settings: Default::default(),
        }
    }
}

impl Default for capi::AcquireProperties_aq_properties_video_s_aq_properties_storage_s {
    fn default() -> Self {
        Self {
            identifier: Default::default(),
            settings: Default::default(),
            write_delay_ms: Default::default(),
        }
    }
}
