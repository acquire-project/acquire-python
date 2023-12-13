use pyo3::prelude::*;

use crate::components::macros::impl_plain_old_dict;
use crate::{capi, components::macros::cvt};
use anyhow::anyhow;
use serde::{Deserialize, Serialize};

/// PropertyType
#[pyclass]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PropertyType {
    FixedPrecision,
    FloatingPrecision,
    Enum,
    String,
}

impl Default for PropertyType {
    fn default() -> Self {
        PropertyType::FixedPrecision
    }
}

cvt!(PropertyType => capi::PropertyType,
    FixedPrecision => PropertyType_PropertyType_FixedPrecision,
    FloatingPrecision => PropertyType_PropertyType_FloatingPrecision,
    Enum => PropertyType_PropertyType_Enum,
    String => PropertyType_PropertyType_String
);

/// Property
#[pyclass]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    #[pyo3(get)]
    writable: bool,

    #[pyo3(get)]
    low: f32,

    #[pyo3(get)]
    high: f32,

    #[pyo3(get)]
    kind: PropertyType,
}

impl_plain_old_dict!(Property);

impl Default for Property {
    fn default() -> Self {
        Self {
            writable: false,
            low: 0.0,
            high: 0.0,
            kind: PropertyType::default(),
        }
    }
}

impl TryFrom<capi::Property> for Property {
    type Error = anyhow::Error;

    fn try_from(value: capi::Property) -> Result<Self, Self::Error> {
        Ok(Self {
            writable: value.writable != 0,
            low: value.low,
            high: value.high,
            kind: value.type_.try_into()?,
        })
    }
}

/// capi
impl Default for capi::Property {
    fn default() -> Self {
        Self {
            writable: 0,
            low: 0.0,
            high: 0.0,
            type_: PropertyType::default().into(),
        }
    }
}

impl TryFrom<&Property> for capi::Property {
    type Error = anyhow::Error;

    fn try_from(value: &Property) -> Result<Self, Self::Error> {
        Ok(Self {
            writable: value.writable as u8,
            low: value.low,
            high: value.high,
            type_: value.kind.try_into()?,
        })
    }
}
