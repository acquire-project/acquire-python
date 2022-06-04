mod core_runtime;
use std::{
    ffi::CStr,
    ptr::{null_mut, NonNull},
    sync::Arc,
};

use anyhow::{anyhow, Result};
use log::{debug, error, info, trace};
use numpy::{
    ndarray::{Dim, IntoDimension, RawArrayView},
    Element, Ix4, PyArray4, ToPyArray,
};
use pyo3::prelude::*;

use crate::core_runtime::core_shutdown;

trait Status: Sized {
    fn ok(&self) -> Result<Self>;
}

impl Status for core_runtime::DeviceStatusCode {
    fn ok(&self) -> Result<Self> {
        if *self == core_runtime::CoreStatusCode_CoreStatus_Ok {
            Ok(*self)
        } else {
            Err(anyhow!("Failed status check"))
        }
    }
}

#[pyfunction]
fn core_api_version() -> PyResult<String> {
    let ptr = unsafe { core_runtime::core_api_version_string() };
    Ok(unsafe { CStr::from_ptr(ptr) }.to_str()?.to_owned())
}

unsafe extern "C" fn reporter(
    is_error: ::std::os::raw::c_int,
    file: *const ::std::os::raw::c_char,
    line: ::std::os::raw::c_int,
    function: *const ::std::os::raw::c_char,
    msg: *const ::std::os::raw::c_char,
) {
    fn as_str(ptr: *const ::std::os::raw::c_char) -> &'static str {
        if !ptr.is_null() {
            unsafe { CStr::from_ptr(ptr) }.to_str().unwrap()
        } else {
            "(null)"
        }
    }

    let file = as_str(file);
    let function = as_str(function);
    let msg = as_str(msg);
    if is_error > 0 {
        error!("{}:{} - {}(): {}", file, line, function, msg);
    } else {
        info!("{}:{} - {}(): {}", file, line, function, msg);
    }
}

struct RawRuntime {
    inner: NonNull<core_runtime::CoreRuntime>,
}

unsafe impl Send for RawRuntime {}
unsafe impl Sync for RawRuntime {}

impl RawRuntime {
    fn new() -> Result<Self> {
        Ok(Self {
            inner: NonNull::new(unsafe { core_runtime::core_init(Some(reporter)) })
                .ok_or(anyhow!("Failed to initialize core runtime."))?,
        })
    }
}

impl Drop for RawRuntime {
    fn drop(&mut self) {
        debug!("SHUTDOWN Runtime");
        unsafe {
            core_shutdown(self.inner.as_mut())
                .ok()
                .expect("Core runtime shutdown failed.");
        }
    }
}

impl AsRef<NonNull<core_runtime::CoreRuntime>> for RawRuntime {
    fn as_ref(&self) -> &NonNull<core_runtime::CoreRuntime> {
        &self.inner
    }
}

#[pyclass]
struct Runtime {
    inner: Arc<RawRuntime>,
}

impl AsRef<NonNull<core_runtime::CoreRuntime>> for Runtime {
    fn as_ref(&self) -> &NonNull<core_runtime::CoreRuntime> {
        &self.inner.inner
    }
}

#[pymethods]
impl Runtime {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self {
            inner: Arc::new(RawRuntime::new()?),
        })
    }

    fn get_available_data(&self) -> PyResult<AvailableData> {
        let mut buf = null_mut();
        let mut nbytes = 0;
        unsafe {
            core_runtime::core_map_read(self.as_ref().as_ptr(), &mut buf, &mut nbytes).ok()?;
        }
        Ok(AvailableData{ inner: Arc::new(
            RawAvailableData {
                runtime: self.inner.clone(),
                buf: NonNull::new(buf),
                nbytes: nbytes as _,
                consumed_bytes: None,
            }
        )})
    }
}

struct RawAvailableData {
    runtime: Arc<RawRuntime>,
    buf: Option<NonNull<core_runtime::VideoFrame>>,
    nbytes: usize,
    consumed_bytes: Option<usize>,
}

unsafe impl Send for AvailableData {}

impl RawAvailableData {
    fn get_frame_count(&self) -> usize {
        let mut count = 0;
        if let Some(buf) = self.buf {
            unsafe {
                let end = buf.as_ptr().offset(self.nbytes as _);
                let mut cur = buf.as_ptr();
                while cur < end {
                    cur = cur.offset((*cur).bytes_of_frame as _);
                    count += 1;
                }
            }
        }
        count
    }
}

impl Drop for RawAvailableData {
    fn drop(&mut self) {
        debug!("Unmapping read region");
        let consumed_bytes = self.consumed_bytes.unwrap_or(self.nbytes);
        unsafe {
            core_runtime::core_unmap_read(self.runtime.inner.as_ptr(), consumed_bytes as _)
                .ok()
                .expect("Unexpected failure: Was the CoreRuntime NULL?");
        }
    }
}

#[pyclass]
struct AvailableData {
    inner: Arc<RawAvailableData>,
}

#[pymethods]
impl AvailableData {
    fn get_frame_count(&self) -> usize {
        self.inner.get_frame_count()
    }

    fn frames(&self)->VideoFrameIterator {
        
        VideoFrameIterator { _store: self.inner.clone(), cur: self.inner.buf, end: unsafe{self.inner.buf} }
    }


    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<VideoFrameIterator>> {
        let iter = VideoFrameIterator {
            cur: slf.buf,
            end: unsafe { slf.buf.offset(slf.nbytes as _) },
        };
        Py::new(slf.py(), iter)
    }
}

struct VideoFrameIterator {
    _store: Arc<RawAvailableData>,
    cur: NonNull<core_runtime::VideoFrame>,
    end: NonNull<core_runtime::VideoFrame>,
}

impl VideoFrameIterator {

}

impl Iterator for VideoFrameIterator {
    type Item = VideoFrame;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.end {
            self.cur = unsafe { (self.cur as *const u8).offset((*self.cur).bytes_of_frame as _) }
                as *const core_runtime::VideoFrame;
            VideoFrame::new(self.cur).ok()
        } else {
            None
        }
    }
}

#[pyclass]
#[derive(Debug, Default)]
struct VideoFrameTimestamps {
    hardware: u64,
    acq_thread: u64,
}

impl From<core_runtime::VideoFrame_video_frame_timestamps_s> for VideoFrameTimestamps {
    fn from(x: core_runtime::VideoFrame_video_frame_timestamps_s) -> Self {
        VideoFrameTimestamps {
            hardware: x.hardware,
            acq_thread: x.acq_thread,
        }
    }
}

#[pyclass]
struct VideoFrameMetadata {
    frame_id: u64,
    timestamps: VideoFrameTimestamps,
}

enum SupportedImageView {
    U8(RawArrayView<u8, Ix4>),
    U16(RawArrayView<u16, Ix4>),
    I8(RawArrayView<i8, Ix4>),
    I16(RawArrayView<i16, Ix4>),
    F32(RawArrayView<f32, Ix4>),
}

impl IntoDimension for core_runtime::ImageShape_image_dims_s {
    type Dim = Ix4;

    fn into_dimension(self) -> Self::Dim {
        Dim([
            self.channels as usize,
            self.width as usize,
            self.height as usize,
            self.planes as usize,
        ])
    }
}

impl IntoDimension for core_runtime::ImageShape {
    type Dim = Ix4;

    fn into_dimension(self) -> Self::Dim {
        self.dims.into_dimension()
    }
}

#[pyclass]
struct VideoFrame {
    array: SupportedImageView,
    metadata: VideoFrameMetadata,
}

unsafe impl Send for VideoFrame {}

impl VideoFrame {
    fn new(cur: *const core_runtime::VideoFrame) -> Result<Self> {
        macro_rules! gen_match {
            ($x:expr, $($A:ident => $B:ident),+) => {
                {
                let sh=(*cur).shape;
                match $x{
                    $(
                        core_runtime::$A => Ok(SupportedImageView::$B(RawArrayView::from_shape_ptr(
                            sh.into_dimension(),
                            (*cur).data.as_ptr() as _,
                        ))),
                    )+
                    _ => Err(anyhow!(
                        "Unexpected image pixel type. Got value {}",
                        (*cur).shape.type_
                    )),
                }
            }
            };
        }

        let array = unsafe {
            gen_match!((*cur).shape.type_,
                SampleType_SampleType_u8 => U8,
                SampleType_SampleType_u16 => U16,
                SampleType_SampleType_i8 => I8,
                SampleType_SampleType_i16 => I16,
                SampleType_SampleType_f32 => F32
            )
        }?;
        Ok(Self {
            array,
            metadata: unsafe {
                VideoFrameMetadata {
                    frame_id: (*cur).frame_id,
                    timestamps: (*cur).timestamps.into(),
                }
            },
        })
    }
}

#[pymodule]
fn demo_python_api(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<Runtime>()?;
    m.add_function(wrap_pyfunction!(core_api_version, m)?);
    Ok(())
}

// TODO: Probably need a smart pointer on core, other objects
// TODO: Is everything really Send
