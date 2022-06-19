use crate::{
    capi,
    components::{macros::impl_plain_old_dict, Channel, Timing, Trigger},
};
use anyhow::anyhow;
use log::info;
use pyo3::{prelude::*, types::PyList};
use serde::{
    de::{self, Visitor},
    ser::{SerializeSeq, SerializeStruct},
    Deserialize, Serialize,
};

#[pyclass]
#[derive(Clone)]
pub struct SignalProperties {
    channels: Py<PyList>,
    timing: Py<Timing>,
    triggers: Py<PyList>,
}

impl_plain_old_dict!(SignalProperties);

impl Default for SignalProperties {
    fn default() -> Self {
        let (channels, triggers) =
            Python::with_gil(|py| (PyList::empty(py).into(), PyList::empty(py).into()));
        Self {
            channels,
            timing: Python::with_gil(|py| Py::new(py, Timing::default())).unwrap(),
            triggers,
        }
    }
}

impl TryFrom<capi::SignalProperties> for SignalProperties {
    type Error = anyhow::Error;

    fn try_from(value: capi::SignalProperties) -> Result<Self, Self::Error> {
        Ok(Python::with_gil(|py| -> PyResult<_> {
            let triggers = {
                let v: Vec<_> = (0..value.triggers.line_count as usize)
                    .map(|i| {
                        let t: Trigger = value.triggers.lines[i].try_into()?;
                        Py::new(py, t)
                    })
                    .collect::<Result<_, _>>()?;
                PyList::new(py, v).into()
            };
            let channels = {
                let v: Vec<_> = (0..value.channels.line_count as usize)
                    .map(|i| {
                        let t: Channel = value.channels.lines[i].try_into()?;
                        Py::new(py, t)
                    })
                    .collect::<Result<_, _>>()?;
                PyList::new(py, v).into()
            };
            let timing: Timing = value.timing.try_into()?;
            let timing = Py::new(py, timing)?;
            Ok(Self {
                channels,
                timing,
                triggers,
            })
        })?)
    }
}

impl TryFrom<&SignalProperties> for capi::SignalProperties {
    type Error = anyhow::Error;

    fn try_from(src: &SignalProperties) -> Result<Self, Self::Error> {
        let mut dst_channels: capi::SignalProperties_signal_properties_channels_s =
            unsafe { std::mem::zeroed() };
        let mut dst_triggers: capi::SignalProperties_signal_properties_triggers_s =
            unsafe { std::mem::zeroed() };

        let (src_channel_count, src_trigger_count) = Python::with_gil(|py| {
            let src_channel_count = src.channels.as_ref(py).len();
            let src_trigger_count = src.triggers.as_ref(py).len();
            (src_channel_count, src_trigger_count)
        });

        if src_channel_count > dst_channels.lines.len() {
            return Err(anyhow!(
                "Expected fewer channel lines. Require {}≤{}",
                src_channel_count,
                dst_channels.lines.len()
            ));
        }

        if src_trigger_count > dst_triggers.lines.len() {
            return Err(anyhow!(
                "Expected fewer trigger lines. Require {}≤{}",
                src_trigger_count,
                dst_triggers.lines.len()
            ));
        }
        Ok(Python::with_gil(|py| -> PyResult<_> {
            for (src, dst) in src
                .channels
                .as_ref(py)
                .iter()
                .zip(dst_channels.lines.iter_mut())
            {
                *dst = src.extract::<Channel>()?.into()
            }

            for (src, dst) in src
                .triggers
                .as_ref(py)
                .iter()
                .zip(dst_triggers.lines.iter_mut())
            {
                *dst = src.extract::<Trigger>()?.into()
            }

            let timing: Timing = src.timing.extract(py)?;
            let timing = (&timing).into();
            Ok(Self {
                channels: dst_channels,
                timing,
                triggers: dst_triggers,
            })
        })?)
    }
}

impl Serialize for SignalProperties {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        struct ChannelsList<'a>(&'a Py<PyList>);
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
        impl<'a> Serialize for ChannelsList<'a> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                Ok(Python::with_gil(|py| {
                    let list = self.0.as_ref(py);
                    let mut seq = serializer.serialize_seq(Some(list.len()))?;
                    for e in list {
                        let w = e.extract::<Py<Channel>>().unwrap();
                        seq.serialize_element(&w)?;
                    }
                    seq.end()
                })?)
            }
        }

        let mut item = serializer.serialize_struct("signals", 3)?;
        item.serialize_field("channels", &ChannelsList(&self.channels))?;
        item.serialize_field("timing", &self.timing)?;
        item.serialize_field("triggers", &TriggerList(&self.triggers))?;
        item.end()
    }
}

impl<'de> Deserialize<'de> for SignalProperties {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Channels,
            Timing,
            Triggers,
        }

        struct SelfVisitor;

        impl<'de> Visitor<'de> for SelfVisitor {
            type Value = SignalProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct SignalProperties")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut channels = None;
                let mut timing = None;
                let mut triggers = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Channels => {
                            if channels.is_some() {
                                return Err(de::Error::duplicate_field("channels"));
                            }
                            let v: Vec<Trigger> = map.next_value()?;
                            channels = Some(Python::with_gil(|py| {
                                PyList::new(py, v.into_iter().map(|w| Py::new(py, w).unwrap()))
                                    .into()
                            }));
                        }
                        Field::Timing => {
                            if timing.is_some() {
                                return Err(de::Error::duplicate_field("timing"));
                            }
                            timing = Some(map.next_value()?);
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

                let channels = channels.ok_or_else(|| de::Error::missing_field("channels"))?;
                let timing = timing.ok_or_else(|| de::Error::missing_field("timing"))?;
                let triggers = triggers.ok_or_else(|| de::Error::missing_field("triggers"))?;

                Ok(SignalProperties {
                    channels,
                    timing,
                    triggers,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["channels", "timing", "triggers"];
        deserializer.deserialize_struct("Character", FIELDS, SelfVisitor)
    }
}

impl Default for capi::SignalProperties {
    fn default() -> Self {
        Self {
            channels: Default::default(),
            timing: Default::default(),
            triggers: Default::default(),
        }
    }
}

impl Default for capi::SignalProperties_signal_properties_channels_s {
    fn default() -> Self {
        Self {
            line_count: Default::default(),
            lines: Default::default(),
        }
    }
}

impl Default for capi::SignalProperties_signal_properties_timing_s {
    fn default() -> Self {
        Self {
            terminal: Default::default(),
            edge: Default::default(),
            samples_per_second: Default::default(),
        }
    }
}

impl Default for capi::SignalProperties_signal_properties_triggers_s {
    fn default() -> Self {
        Self {
            line_count: Default::default(),
            lines: Default::default(),
        }
    }
}
