use crate::{capi, components::macros::impl_plain_old_dict};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    ffi::{CStr, CString},
    fmt::{Debug, Display},
    ptr::{null, null_mut},
};

#[pyclass]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChunkingShardingDims {
    #[pyo3(get, set)]
    #[serde(default)]
    width: u32,

    #[pyo3(get, set)]
    #[serde(default)]
    height: u32,

    #[pyo3(get, set)]
    #[serde(default)]
    planes: u32,
}

impl_plain_old_dict!(ChunkingShardingDims);

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageProperties {
    #[pyo3(get, set)]
    #[serde(default)]
    pub(crate) filename: Option<String>,

    #[pyo3(get, set)]
    #[serde(default)]
    pub(crate) external_metadata_json: Option<String>,

    /// Doesn't do anything right now. One day could be used for file-rollover.
    #[pyo3(get, set)]
    #[serde(default)]
    pub(crate) first_frame_id: u32,

    #[pyo3(get, set)]
    pub(crate) pixel_scale_um: (f64, f64),

    #[pyo3(get, set)]
    pub(crate) chunk_dims_px: Py<ChunkingShardingDims>,

    #[pyo3(get, set)]
    pub(crate) shard_dims_chunks: Py<ChunkingShardingDims>,

    #[pyo3(get, set)]
    pub(crate) enable_multiscale: bool,
}

impl_plain_old_dict!(StorageProperties);

impl Default for StorageProperties {
    fn default() -> Self {
        let chunk_dims_px = Python::with_gil(|py| Py::new(py, ChunkingShardingDims::default()).unwrap());
        let shard_dims_chunks = Python::with_gil(|py| Py::new(py, ChunkingShardingDims::default()).unwrap());
        Self {
            filename: Default::default(),
            external_metadata_json: Default::default(),
            first_frame_id: Default::default(),
            pixel_scale_um: Default::default(),
            chunk_dims_px,
            shard_dims_chunks,
            enable_multiscale: Default::default(),
        }
    }
}

impl TryFrom<capi::StorageProperties> for StorageProperties {
    type Error = anyhow::Error;

    fn try_from(value: capi::StorageProperties) -> Result<Self, Self::Error> {
        let filename = if value.filename.nbytes == 0 {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(value.filename.str_) }
                    .to_str()?
                    .to_owned(),
            )
        };
        let external_metadata_json = if value.external_metadata_json.nbytes == 0 {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(value.external_metadata_json.str_) }
                    .to_str()?
                    .to_owned(),
            )
        };

        let chunk_dims_px = Python::with_gil(|py| {
            Py::new(
                py,
                ChunkingShardingDims {
                    width: value.chunk_dims_px.width,
                    height: value.chunk_dims_px.height,
                    planes: value.chunk_dims_px.planes,
                },
            ).unwrap()
        });

        let shard_dims_chunks = Python::with_gil(|py| {
            Py::new(
                py,
                ChunkingShardingDims {
                    width: value.shard_dims_chunks.width,
                    height: value.shard_dims_chunks.height,
                    planes: value.shard_dims_chunks.planes,
                },
            ).unwrap()
        });

        Ok(Self {
            filename,
            first_frame_id: value.first_frame_id,
            external_metadata_json,
            pixel_scale_um: (value.pixel_scale_um.x, value.pixel_scale_um.y),
            chunk_dims_px,
            shard_dims_chunks,
            enable_multiscale: (value.enable_multiscale == 1),
        })
    }
}

impl TryFrom<&StorageProperties> for capi::StorageProperties {
    type Error = anyhow::Error;

    fn try_from(value: &StorageProperties) -> Result<Self, Self::Error> {
        let mut out: capi::StorageProperties = unsafe { std::mem::zeroed() };
        // Careful: x needs to live long enough
        let x = if let Some(filename) = &value.filename {
            Some(CString::new(filename.as_str())?)
        } else {
            None
        };
        let (filename, bytes_of_filename) = if let Some(ref x) = x {
            (x.as_ptr(), x.to_bytes_with_nul().len())
        } else {
            (null(), 0)
        };

        // Careful: y needs to live long enough
        let y = if let Some(metadata) = &value.external_metadata_json {
            Some(CString::new(metadata.as_str())?)
        } else {
            None
        };
        let (metadata, bytes_of_metadata) = if let Some(ref y) = y {
            (y.as_ptr(), y.to_bytes_with_nul().len())
        } else {
            (null(), 0)
        };

        let chunk_dims_px = Python::with_gil(|py| -> PyResult<_> {
            let chunk_dims_px: ChunkingShardingDims = value.chunk_dims_px.extract(py)?;
            Ok(chunk_dims_px)
        })?;

        let shard_dims_chunks = Python::with_gil(|py| -> PyResult<_> {
            let shard_dims_chunks: ChunkingShardingDims = value.shard_dims_chunks.extract(py)?;
            Ok(shard_dims_chunks)
        })?;

        // This copies the string into a buffer owned by the return value.
        if !unsafe {
            capi::storage_properties_init(
                &mut out,
                value.first_frame_id,
                filename,
                bytes_of_filename as _,
                metadata,
                bytes_of_metadata as _,
                capi::PixelScale {
                    x: value.pixel_scale_um.0,
                    y: value.pixel_scale_um.1,
                },
            ) == 1
        } {
            Err(anyhow::anyhow!("Failed acquire api status check"))
        } else if !unsafe {
            capi::storage_properties_set_chunking_props(
                &mut out,
                chunk_dims_px.width,
                chunk_dims_px.height,
                chunk_dims_px.planes,
            ) == 1
        } {
            Err(anyhow::anyhow!("Failed acquire api status check"))
        } else if !unsafe {
            capi::storage_properties_set_sharding_props(
                &mut out,
                shard_dims_chunks.width,
                shard_dims_chunks.height,
                shard_dims_chunks.planes,
            ) == 1
        } {
            Err(anyhow::anyhow!("Failed acquire api status check"))
        } else if !unsafe {
            capi::storage_properties_set_enable_multiscale(&mut out, value.enable_multiscale as u8)
                == 1
        } {
            Err(anyhow::anyhow!("Failed acquire api status check"))
        } else {
            Ok(out)
        }
    }
}

impl Default for capi::StorageProperties {
    fn default() -> Self {
        Self {
            filename: Default::default(),
            first_frame_id: Default::default(),
            external_metadata_json: Default::default(),
            pixel_scale_um: Default::default(),
            chunk_dims_px: Default::default(),
            shard_dims_chunks: Default::default(),
            enable_multiscale: Default::default(),
        }
    }
}

impl Default for capi::String {
    fn default() -> Self {
        Self {
            str_: null_mut(),
            nbytes: Default::default(),
            is_ref: Default::default(),
        }
    }
}

impl Default for capi::PixelScale {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

impl Default for capi::StorageProperties_storage_properties_chunking_s {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            planes: Default::default(),
        }
    }
}

impl Default for capi::StorageProperties_storage_properties_sharding_s {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
            planes: Default::default(),
        }
    }
}

impl TryFrom<&ChunkingShardingDims> for capi::StorageProperties_storage_properties_chunking_s {
    type Error = anyhow::Error;

    fn try_from(value: &ChunkingShardingDims) -> Result<Self, Self::Error> {
        Ok(capi::StorageProperties_storage_properties_chunking_s {
            width: value.width,
            height: value.height,
            planes: value.planes,
        })
    }
}

impl Default for capi::ImageShape_image_dims_s {
    fn default() -> Self {
        Self {
            channels: Default::default(),
            width: Default::default(),
            height: Default::default(),
            planes: Default::default(),
        }
    }
}

impl Default for capi::ImageShape_image_strides_s {
    fn default() -> Self {
        Self {
            channels: Default::default(),
            width: Default::default(),
            height: Default::default(),
            planes: Default::default(),
        }
    }
}

impl Default for capi::ImageShape {
    fn default() -> Self {
        Self {
            dims: Default::default(),
            strides: Default::default(),
            type_: Default::default(),
        }
    }
}

impl Display for capi::String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = unsafe { CStr::from_ptr(self.str_) }.to_string_lossy();
        write!(f, "{}", s)
    }
}
