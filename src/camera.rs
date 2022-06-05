use pyo3::prelude::*;
use crate::components::SampleType;

#[pyclass]
#[derive(Debug,Clone,Copy,Default)]
pub struct CameraProperties {
    #[pyo3(get,set)]
    exposure_time_us: f32,

    #[pyo3(get,set)]
    binning: u8,

    #[pyo3(get,set)]
    sample_type: SampleType,

    #[pyo3(get,set)]
    offset: (u32,u32),

    #[pyo3(get,set)]
    shape: (u32,u32),
}

#[pymethods]
impl CameraProperties {
    #[new]
    fn new()->Self {
        Default::default()
    }
}