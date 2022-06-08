use pyo3::prelude::*;

use crate::{
    camera::CameraProperties, core_runtime, signals::SignalProperties,
    stage_axis::StageAxisProperties, storage::StorageProperties, device::DeviceIdentifier,
};

#[pyclass]
#[derive(Debug, Clone, Default)]
struct Camera {
    #[pyo3(get, set)]
    identifier: Option<DeviceIdentifier>,

    #[pyo3(get, set)]
    settings: CameraProperties,
}

impl TryFrom<core_runtime::CoreProperties_core_properties_camera_s> for Camera {
    type Error = anyhow::Error;

    fn try_from(
        value: core_runtime::CoreProperties_core_properties_camera_s,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: value.identifier.try_into().ok(),
            settings: value.settings.try_into()?,
        })
    }
}

#[pyclass]
#[derive(Debug, Clone, Default)]
struct Storage {
    #[pyo3(get, set)]
    identifier: Option<DeviceIdentifier>,

    #[pyo3(get, set)]
    settings: StorageProperties,
}

impl TryFrom<core_runtime::CoreProperties_core_properties_storage_s> for Storage {
    type Error = anyhow::Error;

    fn try_from(
        value: core_runtime::CoreProperties_core_properties_storage_s,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: value.identifier.try_into().ok(),
            settings: value.settings.try_into()?,
        })
    }
}

#[pyclass]
#[derive(Debug, Clone, Default)]
struct StageAxis {
    #[pyo3(get, set)]
    identifier: Option<DeviceIdentifier>,

    #[pyo3(get, set)]
    settings: StageAxisProperties,
}

impl TryFrom<core_runtime::CoreProperties_core_properties_stages_s> for StageAxis {
    type Error = anyhow::Error;

    fn try_from(
        value: core_runtime::CoreProperties_core_properties_stages_s,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: value.identifier.try_into().ok(),
            settings: value.settings.try_into()?,
        })
    }
}

#[pyclass]
#[derive(Debug, Clone, Default)]
struct Signals {
    #[pyo3(get, set)]
    identifier: Option<DeviceIdentifier>,

    #[pyo3(get, set)]
    settings: SignalProperties,
}

impl TryFrom<core_runtime::CoreProperties_core_properties_signals_s> for Signals {
    type Error = anyhow::Error;

    fn try_from(
        value: core_runtime::CoreProperties_core_properties_signals_s,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            identifier: value.identifier.try_into().ok(),
            settings: value.settings.try_into()?,
        })
    }
}

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct CoreProperties {
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

impl TryFrom<core_runtime::CoreProperties> for CoreProperties {
    type Error = anyhow::Error;

    fn try_from(value: core_runtime::CoreProperties) -> Result<Self, Self::Error> {
        let camera = value.camera.try_into()?;
        let storage = value.storage.try_into()?;
        let stages = (value.stages[0].try_into()?,value.stages[1].try_into()?,value.stages[2].try_into()?);
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