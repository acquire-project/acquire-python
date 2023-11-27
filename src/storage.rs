use crate::{
    capi,
    components::{
        macros::impl_plain_old_dict,
        Property,
    },
};
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

impl TryFrom<capi::StorageProperties_storage_properties_chunking_s> for ChunkingShardingDims {
    type Error = anyhow::Error;

    fn try_from(value: capi::StorageProperties_storage_properties_chunking_s) -> Result<Self, Self::Error> {
        Ok(ChunkingShardingDims {
            width: value.width,
            height: value.height,
            planes: value.planes,
        })
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

impl TryFrom<capi::StorageProperties_storage_properties_sharding_s> for ChunkingShardingDims {
    type Error = anyhow::Error;

    fn try_from(value: capi::StorageProperties_storage_properties_sharding_s) -> Result<Self, Self::Error> {
        Ok(ChunkingShardingDims {
            width: value.width,
            height: value.height,
            planes: value.planes,
        })
    }
}

impl TryFrom<&ChunkingShardingDims> for capi::StorageProperties_storage_properties_sharding_s {
    type Error = anyhow::Error;

    fn try_from(value: &ChunkingShardingDims) -> Result<Self, Self::Error> {
        Ok(capi::StorageProperties_storage_properties_sharding_s {
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

/// StorageCapabilities::ChunkingShardingCapabilities
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkingShardingCapabilities {
    #[pyo3(get)]
    is_supported: bool,

    #[pyo3(get)]
    width: Py<Property>,

    #[pyo3(get)]
    height: Py<Property>,

    #[pyo3(get)]
    planes: Py<Property>,
}

impl_plain_old_dict!(ChunkingShardingCapabilities);

impl Default for ChunkingShardingCapabilities {
    fn default() -> Self {
        let width = Python::with_gil(|py| Py::new(py, Property::default()).unwrap());
        let height = Python::with_gil(|py| Py::new(py, Property::default()).unwrap());
        let planes = Python::with_gil(|py| Py::new(py, Property::default()).unwrap());
        Self {
            is_supported: Default::default(),
            width,
            height,
            planes,
        }
    }
}

/// StorageCapabilities::MultiscaleCapabilities
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiscaleCapabilities {
    #[pyo3(get)]
    is_supported: bool,
}

impl_plain_old_dict!(MultiscaleCapabilities);

impl Default for MultiscaleCapabilities {
    fn default() -> Self {
        Self {
            is_supported: Default::default(),
        }
    }
}

/// StorageCapabilities
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageCapabilities {
    #[pyo3(get)]
    chunk_dims_px: Py<ChunkingShardingCapabilities>,

    #[pyo3(get)]
    shard_dims_chunks: Py<ChunkingShardingCapabilities>,

    #[pyo3(get)]
    multiscale: Py<MultiscaleCapabilities>,
}

impl_plain_old_dict!(StorageCapabilities);

impl Default for StorageCapabilities {
    fn default() -> Self {
        let chunk_dims_px = Python::with_gil(|py| Py::new(py, ChunkingShardingCapabilities::default()).unwrap());
        let shard_dims_chunks = Python::with_gil(|py| Py::new(py, ChunkingShardingCapabilities::default()).unwrap());
        let multiscale = Python::with_gil(|py| Py::new(py, MultiscaleCapabilities::default()).unwrap());
        Self {
            chunk_dims_px,
            shard_dims_chunks,
            multiscale,
        }
    }
}

impl TryFrom<capi::StoragePropertyMetadata> for StorageCapabilities {
    type Error = anyhow::Error;

    fn try_from(value: capi::StoragePropertyMetadata) -> Result<Self, Self::Error> {
        let chunk_dims_px = Python::with_gil(|py| -> PyResult<_> {
            let width: Property = value.chunk_dims_px.width.try_into()?;
            let height: Property = value.chunk_dims_px.height.try_into()?;
            let planes: Property = value.chunk_dims_px.planes.try_into()?;
            let chunking = ChunkingShardingCapabilities {
                is_supported: (value.chunk_dims_px.is_supported == 1),
                width: Py::new(py, width)?,
                height: Py::new(py, height)?,
                planes: Py::new(py, planes)?,
            };
            Py::new(py, chunking)
        })?;

        let shard_dims_chunks = Python::with_gil(|py| -> PyResult<_> {
            let width: Property = value.shard_dims_chunks.width.try_into()?;
            let height: Property = value.shard_dims_chunks.height.try_into()?;
            let planes: Property = value.shard_dims_chunks.planes.try_into()?;
            let sharding = ChunkingShardingCapabilities {
                is_supported: (value.shard_dims_chunks.is_supported == 1),
                width: Py::new(py, width)?,
                height: Py::new(py, height)?,
                planes: Py::new(py, planes)?,
            };
            Py::new(py, sharding)
        })?;

        let multiscale = Python::with_gil(|py| -> PyResult<_> {
            let multiscale = MultiscaleCapabilities {
                is_supported: (value.multiscale.is_supported == 1),
            };
            Py::new(py, multiscale)
        })?;

        Ok(Self {
            chunk_dims_px,
            shard_dims_chunks,
            multiscale,
        })
    }
}

/// capi
impl Default for capi::StoragePropertyMetadata_storage_property_metadata_chunking_s {
    fn default() -> Self {
        Self {
            is_supported: Default::default(),
            width: Default::default(),
            height: Default::default(),
            planes: Default::default(),
        }
    }
}

impl TryFrom<&ChunkingShardingCapabilities> for capi::StoragePropertyMetadata_storage_property_metadata_chunking_s {
    type Error = anyhow::Error;

    fn try_from(value: &ChunkingShardingCapabilities) -> Result<Self, Self::Error> {
        let (width, height, planes) = Python::with_gil(|py| -> PyResult<_> {
            let width: Property = value.width.extract(py)?;
            let height: Property = value.height.extract(py)?;
            let planes: Property = value.planes.extract(py)?;
            Ok((width, height, planes))
        })?;

        Ok(Self {
            is_supported: value.is_supported as u8,
            width: (&width).try_into()?,
            height: (&height).try_into()?,
            planes: (&planes).try_into()?,
        })
    }
}

impl Default for capi::StoragePropertyMetadata_storage_property_metadata_sharding_s {
    fn default() -> Self {
        Self {
            is_supported: Default::default(),
            width: Default::default(),
            height: Default::default(),
            planes: Default::default(),
        }
    }
}

impl TryFrom<&ChunkingShardingCapabilities> for capi::StoragePropertyMetadata_storage_property_metadata_sharding_s {
    type Error = anyhow::Error;

    fn try_from(value: &ChunkingShardingCapabilities) -> Result<Self, Self::Error> {
        let (width, height, planes) = Python::with_gil(|py| -> PyResult<_> {
            let width: Property = value.width.extract(py)?;
            let height: Property = value.height.extract(py)?;
            let planes: Property = value.planes.extract(py)?;
            Ok((width, height, planes))
        })?;

        Ok(Self {
            is_supported: value.is_supported as u8,
            width: (&width).try_into()?,
            height: (&height).try_into()?,
            planes: (&planes).try_into()?,
        })
    }
}

impl Default for capi::StoragePropertyMetadata_storage_property_metadata_multiscale_s {
    fn default() -> Self {
        Self {
            is_supported: Default::default(),
        }
    }
}

impl TryFrom<&MultiscaleCapabilities> for capi::StoragePropertyMetadata_storage_property_metadata_multiscale_s {
    type Error = anyhow::Error;

    fn try_from(value: &MultiscaleCapabilities) -> Result<Self, Self::Error> {
        Ok(Self {
            is_supported: value.is_supported as u8,
        })
    }
}

impl Default for capi::StoragePropertyMetadata {
    fn default() -> Self {
        Self {
            chunk_dims_px: Default::default(),
            shard_dims_chunks: Default::default(),
            multiscale: Default::default(),
        }
    }
}

impl TryFrom<&StorageCapabilities> for capi::StoragePropertyMetadata {
    type Error = anyhow::Error;

    fn try_from(value: &StorageCapabilities) -> Result<Self, Self::Error> {
        let (chunk_dims_px, shard_dims_chunks, multiscale) = Python::with_gil(|py| -> PyResult<_> {
            let chunk_dims_px: ChunkingShardingCapabilities = value.chunk_dims_px.extract(py)?;
            let shard_dims_chunks: ChunkingShardingCapabilities = value.shard_dims_chunks.extract(py)?;
            let multiscale: MultiscaleCapabilities = value.multiscale.extract(py)?;
            Ok((chunk_dims_px, shard_dims_chunks, multiscale))
        })?;

        Ok(Self {
            chunk_dims_px: (&chunk_dims_px).try_into()?,
            shard_dims_chunks: (&shard_dims_chunks).try_into()?,
            multiscale: (&multiscale).try_into()?,
        })
    }
}