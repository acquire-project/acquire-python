use crate::{
    capi,
    components::{macros::impl_plain_old_dict, SampleType, Trigger},
};
use anyhow::anyhow;
use pyo3::prelude::*;
use pyo3::types::PyList;
use serde::{
    de::{self, Visitor},
    ser::{SerializeSeq, SerializeStruct},
    Deserialize, Serialize,
};

#[pyclass]
#[derive(Debug, Clone)]
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
    triggers: Py<PyList>, // triggers: Vec<Trigger>, // FIXME: Should be Py<PyList>
}

impl_plain_old_dict!(CameraProperties);

impl Default for CameraProperties {
    fn default() -> Self {
        Self {
            gain_db: Default::default(),
            exposure_time_us: Default::default(),
            binning: Default::default(),
            pixel_type: Default::default(),
            offset: Default::default(),
            shape: Default::default(),
            triggers: Python::with_gil(|py| PyList::empty(py).into()),
        }
    }
}

impl TryFrom<capi::CameraProperties> for CameraProperties {
    type Error = anyhow::Error;

    fn try_from(value: capi::CameraProperties) -> Result<Self, Self::Error> {
        let triggers = Python::with_gil(|py| -> PyResult<_> {
            let v: Vec<_> = (0..value.triggers.line_count as usize)
                .map(|i| {
                    let t: Trigger = value.triggers.lines[i].try_into()?;
                    Py::new(py, t)
                })
                .collect::<Result<_, _>>()?;
            Ok(PyList::new(py, v).into())
        })?;

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

impl TryFrom<&CameraProperties> for capi::CameraProperties {
    type Error = anyhow::Error;

    fn try_from(src: &CameraProperties) -> Result<Self, Self::Error> {
        let mut dst_triggers: capi::CameraProperties_camera_properties_triggers_s =
            unsafe { std::mem::zeroed() };
        let src_trigger_count = Python::with_gil(|py| src.triggers.as_ref(py).len());
        if src_trigger_count > dst_triggers.lines.len() {
            Err(anyhow!(
                "Expected fewer trigger lines. Require {}<{}",
                src_trigger_count,
                dst_triggers.lines.len()
            ))
        } else {
            let offset = capi::CameraProperties_camera_properties_offset_s {
                x: src.offset.0,
                y: src.offset.1,
            };
            let shape = capi::CameraProperties_camera_properties_shape_s {
                x: src.shape.0,
                y: src.shape.1,
            };
            Python::with_gil(|py| -> PyResult<()> {
                for (src, dst) in src
                    .triggers
                    .as_ref(py)
                    .iter()
                    .zip(dst_triggers.lines.iter_mut())
                {
                    *dst = src.extract::<Trigger>()?.into()
                }
                Ok(())
            })?;
            Ok(capi::CameraProperties {
                gain_dB: src.gain_db,
                exposure_time_us: src.exposure_time_us,
                binning: src.binning,
                pixel_type: src.pixel_type.into(),
                offset,
                shape,
                triggers: dst_triggers,
            })
        }
    }
}

impl Serialize for CameraProperties {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        struct TriggerList<'a>(&'a Py<PyList>);

        impl<'a> Serialize for TriggerList<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                Ok(Python::with_gil(|py| {
                    let list = self.0.as_ref(py);
                    let mut seq = serializer.serialize_seq(Some(list.len()))?;
                    for e in list {
                        let w = e.extract::<Py<Trigger>>().unwrap();
                        seq.serialize_element(&w)?;
                    }
                    seq.end()
                })?)
            }
        }

        let mut item = serializer.serialize_struct("camera", 7)?;

        macro_rules! ser_field {
            ($name:tt) => {
                item.serialize_field(stringify!(name), &self.$name)
            };
        }
        ser_field!(gain_db)?;
        ser_field!(exposure_time_us)?;
        ser_field!(binning)?;
        ser_field!(pixel_type)?;
        ser_field!(offset)?;
        ser_field!(shape)?;
        item.serialize_field("triggers", &TriggerList(&self.triggers))?;
        item.end()
    }
}

impl<'de> Deserialize<'de> for CameraProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            GainDb,
            ExposureTimeUs,
            Binning,
            PixelType,
            Offset,
            Shape,
            Triggers,
        }

        struct SelfVisitor;

        impl<'de> Visitor<'de> for SelfVisitor {
            type Value = CameraProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct CameraProperties")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut gain_db = None;
                let mut exposure_time_us = None;
                let mut binning = None;
                let mut pixel_type = None;
                let mut offset = None;
                let mut shape = None;
                let mut triggers = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::GainDb => {
                            if gain_db.is_some() {
                                return Err(de::Error::duplicate_field("gain_db"));
                            }
                            gain_db = Some(map.next_value()?);
                        }
                        Field::ExposureTimeUs => {
                            if exposure_time_us.is_some() {
                                return Err(de::Error::duplicate_field("exposure_time_us"));
                            }
                            exposure_time_us = Some(map.next_value()?);
                        }
                        Field::Binning => {
                            if binning.is_some() {
                                return Err(de::Error::duplicate_field("binning"));
                            }
                            binning = Some(map.next_value()?);
                        }
                        Field::PixelType => {
                            if pixel_type.is_some() {
                                return Err(de::Error::duplicate_field("pixel_type"));
                            }
                            pixel_type = Some(map.next_value()?);
                        }
                        Field::Offset => {
                            if offset.is_some() {
                                return Err(de::Error::duplicate_field("offset"));
                            }
                            offset = Some(map.next_value()?);
                        }
                        Field::Shape => {
                            if shape.is_some() {
                                return Err(de::Error::duplicate_field("shape"));
                            }
                            shape = Some(map.next_value()?);
                        }
                        Field::Triggers => {
                            if triggers.is_some() {
                                return Err(de::Error::duplicate_field("triggers"));
                            }
                            let v: Vec<Trigger> = map.next_value()?;
                            triggers = Some(Python::with_gil(|py| {
                                PyList::new(py, v.into_iter().map(|w| Py::new(py, w).unwrap()))
                                    .into()
                            }));
                        }
                    }
                }

                let gain_db = gain_db.ok_or_else(|| de::Error::missing_field("gain_db"))?;
                let exposure_time_us =
                    exposure_time_us.ok_or_else(|| de::Error::missing_field("exposure_time_us"))?;
                let binning = binning.ok_or_else(|| de::Error::missing_field("binning"))?;
                let pixel_type =
                    pixel_type.ok_or_else(|| de::Error::missing_field("pixel_type"))?;
                let offset = offset.ok_or_else(|| de::Error::missing_field("offset"))?;
                let shape = shape.ok_or_else(|| de::Error::missing_field("shape"))?;
                let triggers = triggers.ok_or_else(|| de::Error::missing_field("triggers"))?;

                Ok(CameraProperties {
                    gain_db,
                    exposure_time_us,
                    binning,
                    pixel_type,
                    offset,
                    shape,
                    triggers,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "gain_db",
            "exposure_time_us",
            "binning",
            "pixel_type",
            "offset",
            "shape",
            "triggers",
        ];
        deserializer.deserialize_struct("Character", FIELDS, SelfVisitor)
    }
}

impl Default for capi::CameraProperties {
    fn default() -> Self {
        Self {
            gain_dB: 1.0,
            exposure_time_us: Default::default(),
            binning: 1,
            pixel_type: capi::SampleType_SampleType_u16,
            offset: Default::default(),
            shape: Default::default(),
            triggers: Default::default(),
        }
    }
}

impl Default for capi::CameraProperties_camera_properties_offset_s {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default() }
    }
}

impl Default for capi::CameraProperties_camera_properties_shape_s {
    fn default() -> Self {
        Self { x: Default::default(), y: Default::default() }
    }
}

impl Default for capi::CameraProperties_camera_properties_triggers_s {
    fn default() -> Self {
        Self { line_count: Default::default(), lines: Default::default() }
    }
}

// TODO: (nclack) having to completely implement Serialize and Deserialize
//                here is terrible. Find an alternative...something where
//                the fields can be spec'd once
// TODO: (nclack) macro for Deserialize. Just need value and list types.
