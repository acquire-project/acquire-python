use pyo3::prelude::*;
use serde::Serialize;

use crate::{
    camera::CameraProperties, capi, device::DeviceIdentifier, signals::SignalProperties,
    stage_axis::StageAxisProperties, storage::StorageProperties, components::macros::impl_plain_old_dict,
};

#[pyclass]
#[derive(Debug, Clone, Default, Serialize)]
struct Camera {
    #[pyo3(get, set)]
    identifier: Option<DeviceIdentifier>,

    #[pyo3(get, set)]
    settings: CameraProperties,
}

impl_plain_old_dict!(@out Camera);

impl TryFrom<capi::CpxProperties_cpx_properties_camera_s> for Camera {
    type Error = anyhow::Error;

    fn try_from(value: capi::CpxProperties_cpx_properties_camera_s) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: value.identifier.try_into().ok(),
            settings: value.settings.try_into()?,
        })
    }
}

#[pyclass]
#[derive(Debug, Clone, Default, Serialize)]
struct Storage {
    #[pyo3(get, set)]
    identifier: Option<DeviceIdentifier>,

    #[pyo3(get, set)]
    settings: StorageProperties,
}

impl_plain_old_dict!(@out Storage);

impl TryFrom<capi::CpxProperties_cpx_properties_storage_s> for Storage {
    type Error = anyhow::Error;

    fn try_from(value: capi::CpxProperties_cpx_properties_storage_s) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: value.identifier.try_into().ok(),
            settings: value.settings.try_into()?,
        })
    }
}

#[pyclass]
#[derive(Debug, Clone, Default, Serialize)]
struct StageAxis {
    #[pyo3(get, set)]
    identifier: Option<DeviceIdentifier>,

    #[pyo3(get, set)]
    settings: StageAxisProperties,
}

impl_plain_old_dict!(@out StageAxis);

impl TryFrom<capi::CpxProperties_cpx_properties_stages_s> for StageAxis {
    type Error = anyhow::Error;

    fn try_from(value: capi::CpxProperties_cpx_properties_stages_s) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: value.identifier.try_into().ok(),
            settings: value.settings.try_into()?,
        })
    }
}

#[pyclass]
#[derive(Debug, Clone, Default, Serialize)]
struct Signals {
    #[pyo3(get, set)]
    identifier: Option<DeviceIdentifier>,

    #[pyo3(get, set)]
    settings: SignalProperties,
}

impl_plain_old_dict!(@out Signals);

impl TryFrom<capi::CpxProperties_cpx_properties_signals_s> for Signals {
    type Error = anyhow::Error;

    fn try_from(value: capi::CpxProperties_cpx_properties_signals_s) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: value.identifier.try_into().ok(),
            settings: value.settings.try_into()?,
        })
    }
}

#[pyclass]
#[derive(Debug, Clone, Default, Serialize)]
pub struct Properties {
    #[pyo3(get, set)]
    camera: Camera,

    #[pyo3(get, set)]
    storage: Storage,

    #[pyo3(get, set)]
    stages: (StageAxis, StageAxis, StageAxis),

    #[pyo3(get, set)]
    signals: Signals,

    #[pyo3(get, set)]
    max_frame_count: u64,

    #[pyo3(get, set)]
    frame_average_count: u32,
}

#[pymethods]
impl Properties {
    #[new]
    #[args(kwargs = "**")]
    fn __new__(kwargs: Option<&pyo3::types::PyDict>) -> anyhow::Result<Self> {
        if let Some(kwargs) = kwargs {
            macro_rules! get_device_field {
                ($field:ident, $T:tt) => {
                    if let Some(obj) = kwargs.get_item(stringify!($field)) {
                        $T {
                            identifier: None,
                            settings: pythonize::depythonize(obj)?,
                        }
                    } else {
                        Default::default()
                    }
                };
            }

            let camera = get_device_field!(camera, Camera);
            let storage = get_device_field!(storage, Storage);
            let signals = get_device_field!(signals, Signals);
            let stage_axis = get_device_field!(stage_axis, StageAxis);

            let max_frame_count = if let Some(obj) = kwargs.get_item("max_frame_count") {
                obj.extract()?
            } else {
                0
            };

            let frame_average_count = if let Some(obj) = kwargs.get_item("frame_average_count") {
                obj.extract()?
            } else {
                0
            };

            Ok(Self {
                camera,
                storage,
                signals,
                stages: (stage_axis.clone(), stage_axis.clone(), stage_axis.clone()),
                max_frame_count,
                frame_average_count,
            })
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

impl TryFrom<capi::CpxProperties> for Properties {
    type Error = anyhow::Error;

    fn try_from(value: capi::CpxProperties) -> Result<Self, Self::Error> {
        let camera = value.camera.try_into()?;
        let storage = value.storage.try_into()?;
        let stages = (
            value.stages[0].try_into()?,
            value.stages[1].try_into()?,
            value.stages[2].try_into()?,
        );
        let signals = value.signals.try_into()?;
        Ok(Self {
            camera,
            storage,
            stages,
            signals,
            max_frame_count: value.max_frame_count,
            frame_average_count: value.frame_average_count,
        })
    }
}
