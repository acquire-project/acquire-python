use crate::{
    capi,
    components::macros::{cvt, impl_plain_old_dict},
};
use anyhow::{anyhow, Result};
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    ffi::{CStr, CString},
    fmt::{Debug, Display},
    ptr::{null, null_mut},
};

#[pyclass]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DimensionType {
    NONE,
    Spatial,
    Channel,
    Time,
}

impl Default for DimensionType {
    fn default() -> Self {
        DimensionType::NONE
    }
}

cvt!(DimensionType => capi::DimensionType,
    NONE => DimensionType_DimensionType_None,
    Spatial => DimensionType_DimensionType_Spatial,
    Channel => DimensionType_DimensionType_Channel,
    Time => DimensionType_DimensionType_Time
);

#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    #[pyo3(get, set)]
    #[serde(default)]
    pub(crate) name: Option<String>,

    #[pyo3(get, set)]
    #[serde(default)]
    pub(crate) kind: DimensionType,

    #[pyo3(get, set)]
    #[serde(default)]
    pub(crate) array_size_px: u32,

    #[pyo3(get, set)]
    #[serde(default)]
    pub(crate) chunk_size_px: u32,

    #[pyo3(get, set)]
    #[serde(default)]
    pub(crate) shard_size_chunks: u32,
}

impl Default for Dimension {
    fn default() -> Self {
        Self {
            name: Default::default(),
            kind: Default::default(),
            array_size_px: Default::default(),
            chunk_size_px: Default::default(),
            shard_size_chunks: Default::default(),
        }
    }
}

impl_plain_old_dict!(Dimension);

impl TryFrom<capi::Dimension> for Dimension {
    type Error = anyhow::Error;

    fn try_from(value: capi::Dimension) -> Result<Self, Self::Error> {
        let name = if value.name.nbytes == 0 {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(value.name.str_) }
                    .to_str()?
                    .to_owned(),
            )
        };

        Ok(Self {
            name,
            kind: value.kind.try_into()?,
            array_size_px: value.array_size_px,
            chunk_size_px: value.chunk_size_px,
            shard_size_chunks: value.shard_size_chunks,
        })
    }
}

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
    pub(crate) acquisition_dimensions: Vec<Py<Dimension>>,

    #[pyo3(get, set)]
    pub(crate) enable_multiscale: bool,
}

impl_plain_old_dict!(StorageProperties);

impl Default for StorageProperties {
    fn default() -> Self {
        Self {
            filename: Default::default(),
            external_metadata_json: Default::default(),
            first_frame_id: Default::default(),
            pixel_scale_um: Default::default(),
            acquisition_dimensions: Default::default(),
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

        let mut acquisition_dimensions: Vec<Py<Dimension>> = Default::default();
        for i in 1..value.acquisition_dimensions.size {
            acquisition_dimensions.push(Python::with_gil(|py| {
                Py::new(
                    py,
                    Dimension::try_from(unsafe { *value.acquisition_dimensions.data.add(i) })
                        .unwrap(),
                )
                .unwrap()
            }));
        }

        Ok(Self {
            filename,
            first_frame_id: value.first_frame_id,
            external_metadata_json,
            pixel_scale_um: (value.pixel_scale_um.x, value.pixel_scale_um.y),
            acquisition_dimensions,
            enable_multiscale: (value.enable_multiscale == 1),
        })
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

/// StorageCapabilities::ChunkingCapabilities
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkingCapabilities {
    #[pyo3(get)]
    is_supported: bool,
}

impl_plain_old_dict!(ChunkingCapabilities);

impl Default for ChunkingCapabilities {
    fn default() -> Self {
        Self {
            is_supported: Default::default(),
        }
    }
}

/// StorageCapabilities::ShardingCapabilities
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShardingCapabilities {
    #[pyo3(get)]
    is_supported: bool,
}

impl_plain_old_dict!(ShardingCapabilities);

impl Default for ShardingCapabilities {
    fn default() -> Self {
        Self {
            is_supported: Default::default(),
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
    chunking: Py<ChunkingCapabilities>,

    #[pyo3(get)]
    sharding: Py<ShardingCapabilities>,

    #[pyo3(get)]
    multiscale: Py<MultiscaleCapabilities>,
}

impl_plain_old_dict!(StorageCapabilities);

impl Default for StorageCapabilities {
    fn default() -> Self {
        Self {
            chunking: Python::with_gil(|py| Py::new(py, ChunkingCapabilities::default()).unwrap()),
            sharding: Python::with_gil(|py| Py::new(py, ShardingCapabilities::default()).unwrap()),
            multiscale: Python::with_gil(|py| {
                Py::new(py, MultiscaleCapabilities::default()).unwrap()
            }),
        }
    }
}

impl TryFrom<capi::StoragePropertyMetadata> for StorageCapabilities {
    type Error = anyhow::Error;

    fn try_from(value: capi::StoragePropertyMetadata) -> Result<Self, Self::Error> {
        let chunking = Python::with_gil(|py| -> PyResult<_> {
            let chunking = ChunkingCapabilities {
                is_supported: (value.multiscale.is_supported == 1),
            };
            Py::new(py, chunking)
        })?;

        let sharding = Python::with_gil(|py| -> PyResult<_> {
            let sharding = ShardingCapabilities {
                is_supported: (value.multiscale.is_supported == 1),
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
            chunking,
            sharding,
            multiscale,
        })
    }
}

/// capi::StorageProperties
impl Default for capi::StorageProperties {
    fn default() -> Self {
        Self {
            filename: Default::default(),
            first_frame_id: Default::default(),
            external_metadata_json: Default::default(),
            pixel_scale_um: Default::default(),
            acquisition_dimensions: Default::default(),
            enable_multiscale: Default::default(),
        }
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
            capi::storage_properties_set_enable_multiscale(&mut out, value.enable_multiscale as u8)
                == 1
        } {
            Err(anyhow::anyhow!("Failed acquire api status check"))
        } else {
            Ok(out)
        }
    }
}

/// capi::StorageProperties_storage_properties_dimensions_s
impl Default for capi::StorageProperties_storage_properties_dimensions_s {
    fn default() -> Self {
        Self {
            data: null_mut(),
            size: Default::default(),
            capacity: Default::default(),
        }
    }
}

/// capi::Dimension
impl Default for capi::Dimension {
    fn default() -> Self {
        Self {
            name: Default::default(),
            kind: Default::default(),
            array_size_px: Default::default(),
            chunk_size_px: Default::default(),
            shard_size_chunks: Default::default(),
        }
    }
}

impl TryFrom<&Dimension> for capi::Dimension {
    type Error = anyhow::Error;

    fn try_from(value: &Dimension) -> Result<Self, Self::Error> {
        let mut out: capi::Dimension = unsafe { std::mem::zeroed() };
        // Careful: x needs to live long enough
        let x = if let Some(name) = &value.name {
            Some(CString::new(name.as_str())?)
        } else {
            None
        };
        let (name, bytes_of_name) = if let Some(ref x) = x {
            (x.as_ptr(), x.to_bytes_with_nul().len())
        } else {
            (null(), 0)
        };

        // This copies the string into a buffer owned by the return value.
        if !unsafe {
            capi::dimension_init(
                &mut out,
                name,
                bytes_of_name as _,
                value.kind.into(),
                value.array_size_px,
                value.chunk_size_px,
                value.shard_size_chunks,
            ) == 1
        } {
            Err(anyhow::anyhow!("Failed acquire api status check"))
        } else {
            Ok(out)
        }
    }
}

/// capi::StoragePropertyMetadata
impl Default for capi::StoragePropertyMetadata {
    fn default() -> Self {
        Self {
            chunking: Default::default(),
            sharding: Default::default(),
            multiscale: Default::default(),
        }
    }
}

impl TryFrom<&StorageCapabilities> for capi::StoragePropertyMetadata {
    type Error = anyhow::Error;

    fn try_from(value: &StorageCapabilities) -> Result<Self, Self::Error> {
        let (chunking, sharding, multiscale) = Python::with_gil(|py| -> PyResult<_> {
            let chunking: ChunkingCapabilities = value.chunking.extract(py)?;
            let sharding: ShardingCapabilities = value.sharding.extract(py)?;
            let multiscale: MultiscaleCapabilities = value.multiscale.extract(py)?;
            Ok((chunking, sharding, multiscale))
        })?;

        Ok(Self {
            chunking: (&chunking).try_into()?,
            sharding: (&sharding).try_into()?,
            multiscale: (&multiscale).try_into()?,
        })
    }
}

/// capi::StoragePropertyMetadata_storage_property_metadata_chunking_s
impl Default for capi::StoragePropertyMetadata_storage_property_metadata_chunking_s {
    fn default() -> Self {
        Self {
            is_supported: Default::default(),
        }
    }
}

impl TryFrom<&ChunkingCapabilities>
    for capi::StoragePropertyMetadata_storage_property_metadata_chunking_s
{
    type Error = anyhow::Error;

    fn try_from(value: &ChunkingCapabilities) -> Result<Self, Self::Error> {
        Ok(Self {
            is_supported: value.is_supported as u8,
        })
    }
}

/// capi::StoragePropertyMetadata_storage_property_metadata_sharding_s
impl Default for capi::StoragePropertyMetadata_storage_property_metadata_sharding_s {
    fn default() -> Self {
        Self {
            is_supported: Default::default(),
        }
    }
}

impl TryFrom<&ShardingCapabilities>
    for capi::StoragePropertyMetadata_storage_property_metadata_sharding_s
{
    type Error = anyhow::Error;

    fn try_from(value: &ShardingCapabilities) -> Result<Self, Self::Error> {
        Ok(Self {
            is_supported: value.is_supported as u8,
        })
    }
}

/// capi::StoragePropertyMetadata_storage_property_metadata_multiscale_s
impl Default for capi::StoragePropertyMetadata_storage_property_metadata_multiscale_s {
    fn default() -> Self {
        Self {
            is_supported: Default::default(),
        }
    }
}

impl TryFrom<&MultiscaleCapabilities>
    for capi::StoragePropertyMetadata_storage_property_metadata_multiscale_s
{
    type Error = anyhow::Error;

    fn try_from(value: &MultiscaleCapabilities) -> Result<Self, Self::Error> {
        Ok(Self {
            is_supported: value.is_supported as u8,
        })
    }
}
