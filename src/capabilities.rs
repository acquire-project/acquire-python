use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    capi,
    storage::StorageCapabilities,
    camera::CameraCapabilities,
    components::{macros::impl_plain_old_dict, Property},
};

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct VideoStreamCapabilities {
    #[pyo3(get)]
    camera: Py<CameraCapabilities>,

    #[pyo3(get)]
    storage: Py<StorageCapabilities>,

    #[pyo3(get)]
    max_frame_count: Py<Property>,

    #[pyo3(get)]
    frame_average_count: Py<Property>,
}

impl_plain_old_dict!(VideoStreamCapabilities);

impl Default for VideoStreamCapabilities {
    fn default() -> Self {
        Python::with_gil(|py| Self {
            camera: Py::new(py, CameraCapabilities::default()).unwrap(),
            storage: Py::new(py, StorageCapabilities::default()).unwrap(),
            max_frame_count: Py::new(py, Property::default()).unwrap(),
            frame_average_count: Py::new(py, Property::default()).unwrap(),
        })
    }
}

impl TryFrom<capi::AcquirePropertyMetadata_aq_metadata_video_s> for VideoStreamCapabilities {
    type Error = anyhow::Error;

    fn try_from(
        value: capi::AcquirePropertyMetadata_aq_metadata_video_s,
    ) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let camera: CameraCapabilities = value.camera.try_into()?;
            let storage: StorageCapabilities = value.storage.try_into()?;
            let max_frame_count: Property = value.max_frame_count.try_into()?;
            let frame_average_count: Property = value.frame_average_count.try_into()?;

            Ok(Self {
                camera: Py::new(py, camera)?,
                storage: Py::new(py, storage)?,
                max_frame_count: Py::new(py, max_frame_count)?,
                frame_average_count: Py::new(py, frame_average_count)?,
            })
        })?)
    }
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct Capabilities {
    #[pyo3(get)]
    video: (Py<VideoStreamCapabilities>, Py<VideoStreamCapabilities>),
}

impl_plain_old_dict!(Capabilities);

impl Default for Capabilities {
    fn default() -> Self {
        Python::with_gil(|py| Self {
            video: (
                Py::new(py, VideoStreamCapabilities::default()).unwrap(),
                Py::new(py, VideoStreamCapabilities::default()).unwrap(),
            ),
        })
    }
}

impl TryFrom<&capi::AcquirePropertyMetadata> for Capabilities {
    type Error = anyhow::Error;

    fn try_from(value: &capi::AcquirePropertyMetadata) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let video: (VideoStreamCapabilities, VideoStreamCapabilities) =
                (value.video[0].try_into()?, value.video[1].try_into()?);

            Ok(Self {
                video: (
                    Py::new(py, video.0)?,
                    Py::new(py, video.1)?,
                ),
            })
        })?)
    }
}

/// capi

impl Default for capi::AcquirePropertyMetadata_aq_metadata_video_s {
    fn default() -> Self {
        Self {
            camera: Default::default(),
            storage: Default::default(),
            max_frame_count: Default::default(),
            frame_average_count: Default::default(),
        }
    }
}

impl TryFrom<&VideoStreamCapabilities> for capi::AcquirePropertyMetadata_aq_metadata_video_s {
    type Error = anyhow::Error;

    fn try_from(value: &VideoStreamCapabilities) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let camera: CameraCapabilities = value.camera.extract(py)?;
            let camera: capi::CameraPropertyMetadata = (&camera).try_into()?;
            let storage: StorageCapabilities = value.storage.extract(py)?;
            let storage: capi::StoragePropertyMetadata = (&storage).try_into()?;
            let max_frame_count: Property = value.max_frame_count.extract(py)?;
            let frame_average_count: Property = value.max_frame_count.extract(py)?;

            Ok(Self {
                camera,
                storage,
                max_frame_count: (&max_frame_count).try_into()?,
                frame_average_count: (&frame_average_count).try_into()?,
            })
        })?)
    }
}

impl Default for capi::AcquirePropertyMetadata {
    fn default() -> Self {
        Self {
            video: [
                capi::AcquirePropertyMetadata_aq_metadata_video_s::default(),
                capi::AcquirePropertyMetadata_aq_metadata_video_s::default(),
            ],
        }
    }
}

impl TryFrom<&Capabilities> for capi::AcquirePropertyMetadata {
    type Error = anyhow::Error;

    fn try_from(value: &Capabilities) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let video: (VideoStreamCapabilities, VideoStreamCapabilities) =
                (value.video.0.extract(py)?, value.video.1.extract(py)?);

            Ok(Self {
                video: [
                    (&video.0).try_into()?,
                    (&video.1).try_into()?,
                ],
            })
        })?)
    }
}