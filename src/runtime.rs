use anyhow::{anyhow, Result};
use log::{debug, error, info};
use numpy::{
    ndarray::{Dim, IntoDimension, RawArrayView},
    Ix4, ToPyArray,
};
use parking_lot::Mutex;
use pyo3::prelude::*;
use std::{
    ffi::CStr,
    ptr::{null_mut, NonNull},
    sync::Arc,
};

use crate::{core_runtime, device_manager, Status, core_properties::CoreProperties};

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

pub(crate) struct RawRuntime {
    inner: NonNull<core_runtime::CpxRuntime>,
}

unsafe impl Send for RawRuntime {}
unsafe impl Sync for RawRuntime {}

impl RawRuntime {
    fn new() -> Result<Self> {
        Ok(Self {
            inner: NonNull::new(unsafe { core_runtime::cpx_init(Some(reporter)) })
                .ok_or(anyhow!("Failed to initialize core runtime."))?,
        })
    }
}

impl Drop for RawRuntime {
    fn drop(&mut self) {
        debug!("SHUTDOWN Runtime");
        unsafe {
            core_runtime::cpx_shutdown(self.inner.as_mut())
                .ok()
                .expect("Core runtime shutdown failed.");
        }
    }
}

impl AsRef<NonNull<core_runtime::CpxRuntime>> for RawRuntime {
    fn as_ref(&self) -> &NonNull<core_runtime::CpxRuntime> {
        &self.inner
    }
}

#[pyclass]
pub struct Runtime {
    inner: Arc<RawRuntime>,
}

impl AsRef<NonNull<core_runtime::CpxRuntime>> for Runtime {
    fn as_ref(&self) -> &NonNull<core_runtime::CpxRuntime> {
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

    fn device_manager(&self) -> PyResult<device_manager::DeviceManager> {
        Ok(device_manager::DeviceManager {
            _runtime: self.inner.clone(),
            inner: NonNull::new(unsafe {
                core_runtime::cpx_device_manager(self.as_ref().as_ptr())
                as _
            }).ok_or(anyhow!("Failed to get device manager"))?,
        })
    }

    fn get_configuration(&self)->PyResult<CoreProperties> {
        let mut props:core_runtime::CpxProperties=unsafe{std::mem::zeroed()};
        unsafe{core_runtime::cpx_get_configuration(self.as_ref().as_ptr(), &mut props)}.ok()?;
        Ok(props.try_into()?)
    }

    fn get_available_data(&self) -> PyResult<Option<AvailableData>> {
        let mut buf = null_mut();
        let mut nbytes = 0;
        unsafe {
            core_runtime::cpx_map_read(self.as_ref().as_ptr(), &mut buf, &mut nbytes).ok()?;
        }
        Ok(if nbytes > 0 {
            Some(AvailableData {
                inner: Arc::new(RawAvailableData {
                    runtime: self.inner.clone(),
                    buf: NonNull::new(buf).ok_or(anyhow!("Expected non-null buffer"))?,
                    nbytes: nbytes as _,
                    consumed_bytes: None,
                }),
            })
        } else {
            None
        })
    }
}

/// References to a region of raw data being read from a video stream.
struct RawAvailableData {
    /// Reference to the context that owns the region
    runtime: Arc<RawRuntime>,
    /// Pointer to the reserved region
    buf: NonNull<core_runtime::VideoFrame>,
    /// Number of bytes in the mapped region
    nbytes: usize,

    consumed_bytes: Option<usize>,
}

unsafe impl Send for RawAvailableData {}
unsafe impl Sync for RawAvailableData {}

impl RawAvailableData {
    fn get_frame_count(&self) -> usize {
        let mut count = 0;
        unsafe {
            let end = self.buf.as_ptr().offset(self.nbytes as _);
            let mut cur = self.buf.as_ptr();
            while cur < end {
                cur = cur.offset((*cur).bytes_of_frame as _);
                count += 1;
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
            core_runtime::cpx_unmap_read(self.runtime.inner.as_ptr(), consumed_bytes as _)
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

    fn frames(&self) -> VideoFrameIterator {
        VideoFrameIterator {
            store: self.inner.clone(),
            cur: Mutex::new(self.inner.buf),
            end: unsafe {
                NonNull::new_unchecked(self.inner.buf.as_ptr().offset(self.inner.nbytes as _))
            },
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<VideoFrameIterator>> {
        Py::new(slf.py(), slf.frames())
    }
}

#[pyclass]
struct VideoFrameIterator {
    store: Arc<RawAvailableData>,
    cur: Mutex<NonNull<core_runtime::VideoFrame>>,
    end: NonNull<core_runtime::VideoFrame>,
}

unsafe impl Send for VideoFrameIterator {}

impl VideoFrameIterator {}

impl Iterator for VideoFrameIterator {
    type Item = VideoFrame;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cur = self.cur.lock();
        if *cur < self.end {
            let out = VideoFrame {
                _store: self.store.clone(),
                cur: *cur,
            };

            let c = cur.as_ptr();
            let o = unsafe { (c as *const u8).offset((*c).bytes_of_frame as _) }
                as *mut core_runtime::VideoFrame;
            *cur = unsafe { NonNull::new_unchecked(o) };

            Some(out)
        } else {
            None
        }
    }
}

#[pyclass]
#[derive(Debug, Default, Clone, Copy)]
struct VideoFrameTimestamps {
    #[pyo3(get,set)]
    hardware: u64,

    #[pyo3(get,set)]
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
#[derive(Debug, Default, Clone, Copy)]
struct VideoFrameMetadata {
    #[pyo3(get,set)]
    frame_id: u64,

    #[pyo3(get,set)]
    timestamps: VideoFrameTimestamps,
}

enum SupportedImageView {
    U8(RawArrayView<u8, Ix4>),
    U16(RawArrayView<u16, Ix4>),
    I8(RawArrayView<i8, Ix4>),
    I16(RawArrayView<i16, Ix4>),
    F32(RawArrayView<f32, Ix4>),
}

impl SupportedImageView {
    fn to_pyobject<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        macro_rules! cvt {
            ($im:ident) => {
                unsafe { $im.deref_into_view() }
                    .to_pyarray(py)
                    .to_object(py)
            };
        }
        match self {
            SupportedImageView::U8(im) => cvt!(im),
            SupportedImageView::U16(im) => cvt!(im),
            SupportedImageView::I8(im) => cvt!(im),
            SupportedImageView::I16(im) => cvt!(im),
            SupportedImageView::F32(im) => cvt!(im),
        }
    }
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
    _store: Arc<RawAvailableData>,
    cur: NonNull<core_runtime::VideoFrame>,
}

unsafe impl Send for VideoFrame {}

#[pymethods]
impl VideoFrame {
    fn metadata(slf: PyRef<'_, Self>) -> PyResult<VideoFrameMetadata> {
        let cur = slf.cur.as_ptr();
        let meta = unsafe {
            VideoFrameMetadata {
                frame_id: (*cur).frame_id,
                timestamps: (*cur).timestamps.into(),
            }
        };
        Ok(meta)
    }

    fn data<'py>(&self, py: Python<'py>) -> Py<PyAny> {
        let cur = self.cur.as_ptr();

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
        }
        .unwrap();

        array.to_pyobject(py)
    }
}

// TODO: Can't ensure the output array doesn't outlive the available data
// TODO: Is everything really Send
// TODO: mark iterable and videoframe as things that can't be shared across threads
