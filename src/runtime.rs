use anyhow::{anyhow, Result};
use log::{debug, error};
use numpy::{
    ndarray::{Dim, IntoDimension, RawArrayView},
    Ix4, ToPyArray,
};
use parking_lot::Mutex;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    ffi::CStr,
    ptr::{null_mut, NonNull},
    sync::Arc,
};

use crate::capabilities::Capabilities;
use crate::{
    capi, components::macros::impl_plain_old_dict, core_properties::Properties,
    device::DeviceState, device_manager, Status,
};

unsafe extern "C" fn reporter(
    is_error: ::std::os::raw::c_int,
    file: *const ::std::os::raw::c_char,
    line: ::std::os::raw::c_int,
    function: *const ::std::os::raw::c_char,
    msg: *const ::std::os::raw::c_char,
) {
    fn as_string(ptr: *const ::std::os::raw::c_char) -> String {
        if !ptr.is_null() {
            unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into()
        } else {
            "(null)".into()
        }
    }

    let file = as_string(file);
    let function = as_string(function);
    let msg = as_string(msg);
    if is_error > 0 {
        error!("{}:{} - {}(): {}", file, line, function, msg);
    } else {
        debug!("{}:{} - {}(): {}", file, line, function, msg);
    }
}

pub(crate) struct RawRuntime {
    inner: NonNull<capi::AcquireRuntime>,
}

unsafe impl Send for RawRuntime {}
unsafe impl Sync for RawRuntime {}

impl RawRuntime {
    fn new() -> Result<Self> {
        Ok(Self {
            inner: NonNull::new(unsafe { capi::acquire_init(Some(reporter)) })
                .ok_or(anyhow!("Failed to initialize core runtime."))?,
        })
    }

    fn start(&self) -> Result<()> {
        unsafe { capi::acquire_start(self.inner.as_ptr()) }.ok()?;
        Ok(())
    }

    fn execute_trigger(&self, stream_id: u32) -> Result<()> {
        unsafe { capi::acquire_execute_trigger(self.inner.as_ptr(), stream_id) }.ok()?;
        Ok(())
    }

    fn stop(&self) -> Result<()> {
        unsafe { capi::acquire_stop(self.inner.as_ptr()) }.ok()?;
        Ok(())
    }

    fn abort(&self) -> Result<()> {
        unsafe { capi::acquire_abort(self.inner.as_ptr()) }.ok()?;
        Ok(())
    }

    fn map_read(&self, stream_id: u32) -> Result<(*mut capi::VideoFrame, *mut capi::VideoFrame)> {
        let mut beg = null_mut();
        let mut end = null_mut();
        unsafe {
            capi::acquire_map_read(self.inner.as_ptr(), stream_id, &mut beg, &mut end).ok()?;
        }
        Ok((beg, end))
    }

    fn unmap_read(&self, stream_id: u32, consumed_bytes: usize) -> Result<()> {
        unsafe {
            capi::acquire_unmap_read(self.inner.as_ptr(), stream_id, consumed_bytes).ok()?;
        }
        Ok(())
    }
}

impl Drop for RawRuntime {
    fn drop(&mut self) {
        debug!("SHUTDOWN Runtime");
        unsafe {
            capi::acquire_shutdown(self.inner.as_mut())
                .ok()
                .expect("Core runtime shutdown failed.");
        }
    }
}

impl AsRef<NonNull<capi::AcquireRuntime>> for RawRuntime {
    fn as_ref(&self) -> &NonNull<capi::AcquireRuntime> {
        &self.inner
    }
}

#[pyclass]
/// The core runtime state
pub struct Runtime {
    inner: Arc<RawRuntime>,
}

impl AsRef<NonNull<capi::AcquireRuntime>> for Runtime {
    fn as_ref(&self) -> &NonNull<capi::AcquireRuntime> {
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

    fn start(&self, py: Python<'_>) -> PyResult<()> {
        Python::allow_threads(py, || Ok(self.inner.start()?))
    }

    fn stop(&self, py: Python<'_>) -> PyResult<()> {
        Python::allow_threads(py, || Ok(self.inner.stop()?))
    }

    fn abort(&self, py: Python<'_>) -> PyResult<()> {
        Python::allow_threads(py, || Ok(self.inner.abort()?))
    }

    fn device_manager(&self) -> PyResult<device_manager::DeviceManager> {
        Ok(device_manager::DeviceManager {
            _runtime: self.inner.clone(),
            inner: NonNull::new(unsafe {
                capi::acquire_device_manager(self.as_ref().as_ptr()) as _
            })
            .ok_or(anyhow!("Failed to get device manager"))?,
        })
    }

    fn set_configuration(&self, properties: &Properties, py: Python<'_>) -> PyResult<Properties> {
        let mut props: capi::AcquireProperties = properties.try_into()?;
        Python::allow_threads(py, || {
            unsafe { capi::acquire_configure(self.as_ref().as_ptr(), &mut props) }.ok()
        })?;
        Ok((&props).try_into()?)
    }

    fn get_configuration(&self, py: Python<'_>) -> PyResult<Properties> {
        let mut props: capi::AcquireProperties = Default::default();
        Python::allow_threads(py, || {
            unsafe { capi::acquire_get_configuration(self.as_ref().as_ptr(), &mut props) }.ok()
        })?;
        Ok((&props).try_into()?)
    }

    fn get_capabilities(&self, py: Python<'_>) -> PyResult<Capabilities> {
        let mut meta: capi::AcquirePropertyMetadata = Default::default();
        Python::allow_threads(py, || {
            unsafe { capi::acquire_get_configuration_metadata(self.as_ref().as_ptr(), &mut meta) }
                .ok()
        })?;
        Ok((&meta).try_into()?)
    }

    fn get_state(&self, py: Python<'_>) -> PyResult<DeviceState> {
        Ok(Python::allow_threads(py, || unsafe {
            capi::acquire_get_state(self.as_ref().as_ptr())
        })
        .try_into()?)
    }

    fn execute_trigger(&self, stream_id: u32, py: Python<'_>) -> PyResult<()> {
        Python::allow_threads(py, || Ok(self.inner.execute_trigger(stream_id)?))
    }

    fn get_available_data(&self, stream_id: u32) -> PyResult<AvailableDataContext> {
        Ok(AvailableDataContext {
            inner: self.inner.clone(),
            stream_id,
            available_data: Python::with_gil(|py| {
                Py::new(
                    py,
                    AvailableData {
                        inner: Arc::new(Mutex::new(None)),
                    },
                )
            })?,
        })
    }
}

/// References to a region of raw data being read from a video stream.
struct RawAvailableData {
    /// Reference to the context that owns the region
    runtime: Arc<RawRuntime>,
    /// Pointer to the reserved region
    beg: NonNull<capi::VideoFrame>,
    end: NonNull<capi::VideoFrame>,

    /// The video stream owning the region
    stream_id: u32,

    /// When none, the entire region will be unmapped. Otherwise just the first
    /// `consumed_bytes`.
    consumed_bytes: Option<usize>,
}

unsafe impl Send for RawAvailableData {}
unsafe impl Sync for RawAvailableData {}

// Can replace this if `pointer_byte_offsets` gets stabilized.
unsafe fn byte_offset<T>(origin: *mut T, count: isize) -> *mut T {
    (origin as *const u8).offset(count) as *mut T
}

// Can replace this if `pointer_byte_offsets` gets stabilized.
unsafe fn byte_offset_from<T>(beg: *mut T, end: *mut T) -> isize {
    (end as *const u8).offset_from(beg as *const u8)
}

impl RawAvailableData {
    fn get_frame_count(&self) -> usize {
        let mut count = 0;
        unsafe {
            let mut cur = self.beg.as_ptr();
            let end = self.end.as_ptr();
            while cur < end {
                let frame: &capi::VideoFrame = &std::ptr::read_unaligned(cur);
                log::trace!(
                    "[stream {}] Advancing count for frame {} w size {}",
                    self.stream_id,
                    frame.frame_id,
                    frame.bytes_of_frame
                );
                assert!(frame.bytes_of_frame > 0);
                cur = byte_offset(cur, frame.bytes_of_frame as _);
                count += 1;
            }
        }
        count
    }
}

impl Drop for RawAvailableData {
    fn drop(&mut self) {
        let consumed_bytes = self
            .consumed_bytes
            .unwrap_or(unsafe { byte_offset_from(self.beg.as_ptr(), self.end.as_ptr()) } as usize);
        log::debug!(
            "[stream {}] DROP read region: {:p}-{:p}:{}",
            self.stream_id,
            self.beg.as_ptr(),
            self.end.as_ptr(),
            consumed_bytes
        );
        self.runtime
            .unmap_read(self.stream_id, consumed_bytes)
            .expect("Unexpected failure: Was the runtime NULL?");
    }
}

#[pyclass]
pub(crate) struct AvailableData {
    inner: Arc<Mutex<Option<RawAvailableData>>>,
}

#[pymethods]
impl AvailableData {
    fn get_frame_count(&self) -> usize {
        if let Some(inner) = &*self.inner.lock() {
            inner.get_frame_count()
        } else {
            0
        }
    }

    fn frames(&self) -> VideoFrameIterator {
        VideoFrameIterator {
            inner: if let Some(frames) = &*self.inner.lock() {
                Some(VideoFrameIteratorInner {
                    store: self.inner.clone(),
                    cur: Mutex::new(frames.beg),
                    end: frames.end,
                })
            } else {
                None
            },
        }
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResult<Py<VideoFrameIterator>> {
        Py::new(slf.py(), slf.frames())
    }
}

impl AvailableData {
    fn invalidate(&mut self) {
        // Will drop the RawAvailableData and cause Available data to act like
        // an empty iterator.
        *self.inner.lock() = None;
    }
}

#[pyclass]
pub(crate) struct AvailableDataContext {
    inner: Arc<RawRuntime>,
    stream_id: u32,
    available_data: Py<AvailableData>,
}

#[pymethods]
impl AvailableDataContext {
    fn __enter__(&mut self) -> PyResult<Py<AvailableData>> {
        let AvailableDataContext {
            inner,
            stream_id,
            available_data,
        } = self;
        let stream_id = *stream_id;
        let (beg, end) = inner.map_read(stream_id)?;
        let nbytes = unsafe { byte_offset_from(beg, end) };

        log::trace!(
            "[stream {}] ACQUIRED {:p}-{:p}:{} bytes",
            stream_id,
            beg,
            end,
            nbytes
        );
        *available_data = Python::with_gil(|py| {
            Py::new(
                py,
                AvailableData {
                    inner: Arc::new(Mutex::new(Some(RawAvailableData {
                        runtime: self.inner.clone(),
                        beg: NonNull::new(beg).ok_or(anyhow!("Expected non-null buffer"))?,
                        end: NonNull::new(end).ok_or(anyhow!("Expected non-null buffer"))?,
                        stream_id,
                        consumed_bytes: None,
                    }))),
                },
            )
        })?;
        return Ok(self.available_data.clone());
    }

    fn __exit__(&mut self, _exc_type: &PyAny, _exc_value: &PyAny, _traceback: &PyAny) {
        Python::with_gil(|py| {
            (&self.available_data).as_ref(py).borrow_mut().invalidate();
        });
    }
}

struct VideoFrameIteratorInner {
    store: Arc<Mutex<Option<RawAvailableData>>>,
    cur: Mutex<NonNull<capi::VideoFrame>>,
    end: NonNull<capi::VideoFrame>,
}

unsafe impl Send for VideoFrameIteratorInner {}

impl Iterator for VideoFrameIteratorInner {
    type Item = VideoFrame;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cur = self.cur.lock();
        if (*self.store.lock()).is_some() && *cur < self.end {
            let out = VideoFrame {
                _store: self.store.clone(),
                cur: *cur,
            };

            let c = cur.as_ptr();
            let o = unsafe { (c as *const u8).offset((*c).bytes_of_frame as _) }
                as *mut capi::VideoFrame;
            *cur = unsafe { NonNull::new_unchecked(o) };

            Some(out)
        } else {
            None
        }
    }
}

#[pyclass]
struct VideoFrameIterator {
    inner: Option<VideoFrameIteratorInner>,
}

#[pymethods]
impl VideoFrameIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }
    fn __next__(&mut self) -> Option<VideoFrame> {
        self.next()
    }
}

impl Iterator for VideoFrameIterator {
    type Item = VideoFrame;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.as_mut().and_then(|it| it.next())
    }
}

#[pyclass]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct VideoFrameTimestamps {
    #[pyo3(get, set)]
    hardware: u64,

    #[pyo3(get, set)]
    acq_thread: u64,
}

impl_plain_old_dict!(VideoFrameTimestamps);

impl From<capi::VideoFrame_video_frame_timestamps_s> for VideoFrameTimestamps {
    fn from(x: capi::VideoFrame_video_frame_timestamps_s) -> Self {
        VideoFrameTimestamps {
            hardware: x.hardware,
            acq_thread: x.acq_thread,
        }
    }
}

#[pyclass]
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct VideoFrameMetadata {
    #[pyo3(get, set)]
    frame_id: u64,

    #[pyo3(get, set)]
    timestamps: VideoFrameTimestamps,
}

impl_plain_old_dict!(VideoFrameMetadata);

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

impl IntoDimension for capi::ImageShape_image_dims_s {
    type Dim = Ix4;

    fn into_dimension(self) -> Self::Dim {
        Dim([
            self.planes as usize,
            self.height as usize,
            self.width as usize,
            self.channels as usize,
        ])
    }
}

impl IntoDimension for capi::ImageShape {
    type Dim = Ix4;

    fn into_dimension(self) -> Self::Dim {
        self.dims.into_dimension()
    }
}

#[pyclass]
pub(crate) struct VideoFrame {
    _store: Arc<Mutex<Option<RawAvailableData>>>,
    cur: NonNull<capi::VideoFrame>,
}

unsafe impl Send for VideoFrame {}

#[pymethods]
impl VideoFrame {
    fn metadata(slf: PyRef<'_, Self>) -> PyResult<VideoFrameMetadata> {
        if (*slf._store.lock()).is_none() {
            return Err(PyRuntimeError::new_err(
                "VideoFrame is not valid outside of context",
            ));
        }
        let cur = slf.cur.as_ptr();
        let meta = unsafe {
            VideoFrameMetadata {
                frame_id: (*cur).frame_id,
                timestamps: (*cur).timestamps.into(),
            }
        };
        Ok(meta)
    }

    fn data<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
        if (*self._store.lock()).is_none() {
            return Err(PyRuntimeError::new_err(
                "VideoFrame is not valid outside of context",
            ));
        }
        let cur = self.cur.as_ptr();

        macro_rules! gen_match {
            ($x:expr, $($A:ident => $B:ident),+) => {
                {
                let sh=(*cur).shape;
                match $x{
                    $(
                        capi::$A => Ok(SupportedImageView::$B(RawArrayView::from_shape_ptr(
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

        Ok(array.to_pyobject(py))
    }
}

// TODO: Is everything really Send
// TODO: mark iterable and videoframe as things that can't be shared across threads
