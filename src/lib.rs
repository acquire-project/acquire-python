mod core;
use std::ffi::CStr;

use log::{error, info, trace, debug};
use pyo3::{prelude::*, class::buffer::PyBufferReleaseBufferProtocol};
use anyhow::anyhow;


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
    fn as_str(ptr: *const ::std::os::raw::c_char)->&'static str {
        if !ptr.is_null() {
            unsafe{CStr::from_ptr(ptr)}.to_str().unwrap()
        } else {
            "(null)"
        }
    }

    let file=as_str(file);
    let function=as_str(function);
    let msg=as_str(msg);
    if is_error>0 {
        error!("{}:{} - {}(): {}",file,line,function,msg);
    } else {
        info!("{}:{} - {}(): {}",file,line,function,msg);
    }
}

#[pyclass]
struct Runtime {
    inner: *mut core::CoreRuntime
}

unsafe impl Send for Runtime {}

#[pymethods]
impl Runtime {
    #[new]
    fn new()->PyResult<Self> {        
        let inner=unsafe {core::core_init(Some(reporter))};
        if inner.is_null() {
            Err(anyhow!("Failed to initialize the core runtime.").into())
        } else { 
            Ok(Self{inner})
        }
    }

    fn get_available_data(&self)->PyResult<AvailableData> {
        todo!()
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        debug!("SHUTDOWN Runtime");
        unsafe{ core::core_shutdown(self.inner);}
    }
}

#[pyclass]
struct AvailableData{
    bytes: [u8]
};

#[pymethods]
impl AvailableData {
    fn new(bytes:&[u8]) {

    }
}



#[pymodule]
fn demo_python_api(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<Runtime>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
