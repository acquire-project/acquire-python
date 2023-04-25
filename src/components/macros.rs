macro_rules! cvt {
    ($TA:ty => $TB:ty, $($A:ident => $B:ident),+) => {
        crate::components::macros::cvt!(@tryfrom $TA, $TB,  $($A => $B),+);
        crate::components::macros::cvt!(@into $TA, $TB,  $($A => $B),+);
    };
    (@tryfrom $TA:ty, $TB:ty, $($A:ident => $B:ident),+) => {
        impl TryFrom<$TB> for $TA {
            type Error=anyhow::Error;

            fn try_from(value: $TB) -> Result<Self, Self::Error> {
                match value as $TB {
                    $(
                        capi::$B => Ok(<$TA>::$A),
                    )+
                    _ => Err(anyhow!("Unknown {}: {}",stringify!($TA),value))
                }
            }
        }
    };
    (@into $TA:ty, $TB:ty, $($A:ident => $B:ident),+) => {
        impl Into<$TB> for $TA {
            fn into(self) -> $TB {
                match self {
                    $(
                        <$TA>::$A => capi::$B as _,
                    )+
                }
            }
        }
    }
}
pub(crate) use cvt;

// FIXME: (nclack) modularize the parts, dedup code
macro_rules! impl_plain_old_dict {
    (@out $T:ty) => {
        #[pymethods]
        impl $T {
            #[doc=concat!("Make a dict representation of ",stringify!($T))]
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

                Ok(format!("{}({})", stringify!($T), args))
            }
        }
    };
    ($T:ty) => {
        #[pymethods]
        impl $T {
            #[new]
            #[pyo3(signature = (**kwargs))]
            fn __new__(kwargs: Option<&pyo3::types::PyDict>) -> anyhow::Result<Self> {
                if let Some(kwargs) = kwargs {
                    Ok(pythonize::depythonize(kwargs)?)
                } else {
                    Ok(Default::default())
                }
            }

            #[doc=concat!("Make a dict representation of ",stringify!($T))]
            fn dict(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
                Ok(pythonize::pythonize(py, self)?)
            }

            fn __repr__(&self, py: Python<'_>) -> PyResult<String> {
                let obj = pythonize::pythonize(py, self)?;
                let obj = obj.as_ref(py).downcast::<pyo3::types::PyDict>()?;
                let args: String = obj
                    .iter()
                    .map(|(k, v)| format!("{}={:?}", k, v))
                    .reduce(|acc, e| format!("{},{}", acc, e))
                    .unwrap_or(String::new());

                Ok(format!("{}({})", stringify!($T), args))
            }
        }
    };
}
pub(crate) use impl_plain_old_dict;
