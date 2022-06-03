mod core_runtime;
use std::{ffi::CStr, ptr::null_mut};

use anyhow::{anyhow, Result};
use core_runtime::{core_init, core_map_read, core_unmap_read, CoreRuntime, CoreStatusCode};
use log::{debug, error, info, trace};
use numpy::{
    ndarray::{Dim, IntoDimension, RawArrayView},
    Ix4,
};
use pyo3::prelude::*;

use crate::core_runtime::core_shutdown;

fn check(ecode: CoreStatusCode) -> Result<()> {
    todo!()
    // if ecode==core::
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
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

#[pyclass]
struct Runtime {
    inner: *mut CoreRuntime,
}

unsafe impl Send for Runtime {}

#[pymethods]
impl Runtime {
    #[new]
    fn new() -> PyResult<Self> {
        let inner = unsafe { core_init(Some(reporter)) };
        if inner.is_null() {
            Err(anyhow!("Failed to initialize the core runtime.").into())
        } else {
            Ok(Self { inner })
        }
    }

    fn get_available_data(&self) -> PyResult<AvailableData> {
        let mut buf = null_mut();
        let mut nbytes = 0;
        unsafe {
            check(core_map_read(self.inner, &mut buf, &mut nbytes))?;
        }
        Ok(AvailableData {
            inner: self.inner,
            buf,
            nbytes: nbytes as _,
            consumed_bytes: None,
        })
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        debug!("SHUTDOWN Runtime");
        unsafe {
            core_shutdown(self.inner);
        }
    }
}

#[pyclass]
#[derive(Debug)]
struct AvailableData {
    inner: *mut CoreRuntime,
    buf: *const core_runtime::VideoFrame,
    nbytes: usize,
    consumed_bytes: Option<usize>,
}

unsafe impl Send for AvailableData {}

#[pymethods]
impl AvailableData {
    fn get_frame_count(&self) -> usize {
        let mut count = 0;
        unsafe {
            let end = self.buf.offset(self.nbytes as _);
            let mut cur = self.buf;
            while cur < end {
                cur = cur.offset((*cur).bytes_of_frame as _);
                count += 1;
            }
        }
        count
    }

    fn __iter__(&self) -> VideoFrameIterator {
        VideoFrameIterator {
            cur: self.buf,
            end: unsafe { self.buf.offset(self.nbytes as _) },
        }
    }
}

impl Drop for AvailableData {
    fn drop(&mut self) {
        let consumed_bytes = self.consumed_bytes.unwrap_or(self.nbytes);
        unsafe {
            check(core_unmap_read(self.inner, consumed_bytes as _))
                .expect("Unexpected failure: Was the CoreRuntime NULL?");
        }
    }
}

#[pyclass]
struct VideoFrameIterator {
    cur: *const core_runtime::VideoFrame,
    end: *const core_runtime::VideoFrame,
}

unsafe impl Send for VideoFrameIterator {}

impl VideoFrameIterator {}

impl Iterator for VideoFrameIterator {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur < self.end {
            Some(VideoFrameIterator {
                cur: unsafe { (self.cur as *const u8).offset((*self.cur).bytes_of_frame as _) }
                    as *const core_runtime::VideoFrame,
                end: self.end,
            })
        } else {
            None
        }
    }
}

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

struct VideoFrame {
    array: SupportedImageView,
    metadata: VideoFrameMetadata,
}

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
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

// TODO: Probably need a smart pointer on core, other objects
// TODO: Is everything really Send
