mod betterproto_interop;
mod decode;
mod descriptors;
mod encode;
mod well_known_types;

use betterproto_interop::BetterprotoMessage;
use decode::{merge_into_message, DecodeResult};
use encode::{EncodeResult, MessageEncoder};
use pyo3::{prelude::*, types::PyBytes};
use std::sync::Arc;

pub type Str = Arc<str>;

#[pyfunction]
fn deserialize(obj: BetterprotoMessage, mut buf: &[u8]) -> DecodeResult<()> {
    merge_into_message(&obj, &mut buf)
}

#[pyfunction]
fn serialize<'py>(py: Python<'py>, msg: BetterprotoMessage) -> EncodeResult<Bound<'py, PyBytes>> {
    let cls = msg.class();
    let encoder = MessageEncoder::from_betterproto_msg(msg, cls.descriptor(py)?.get())?;
    Ok(PyBytes::new_bound(py, &encoder.into_vec()))
}

#[pymodule]
fn betterproto2_rust_codec(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(deserialize, m)?)?;
    m.add_function(wrap_pyfunction!(serialize, m)?)?;
    Ok(())
}
