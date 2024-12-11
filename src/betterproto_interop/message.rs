use super::{error::InteropResult, BetterprotoMessageClass};
use pyo3::{
    intern,
    types::{PyAnyMethods, PyBytes},
    Bound, FromPyObject, PyAny, Python, ToPyObject,
};

#[derive(FromPyObject, Clone)]
pub struct BetterprotoMessage<'py>(pub(super) Bound<'py, PyAny>);

impl<'py> BetterprotoMessage<'py> {
    pub fn class(&self) -> BetterprotoMessageClass {
        BetterprotoMessageClass(self.0.get_type().unbind())
    }

    pub fn py(&self) -> Python<'py> {
        self.0.py()
    }

    pub fn set_field(&self, field_name: &str, value: impl ToPyObject) -> InteropResult<()> {
        self.0.setattr(field_name, value)?;
        Ok(())
    }

    pub fn get_field(&self, field_name: &str) -> InteropResult<Option<Bound<'py, PyAny>>> {
        let res = self.0.clone().getattr(field_name).expect("Attribute exists").extract()?;

        Ok(res)
    }

    pub fn append_unknown_fields(&self, mut data: Vec<u8>) -> InteropResult<()> {
        let attr_name = intern!(self.py(), "_unknown_fields");
        if !data.is_empty() {
            let mut unknown_fields = self.0.getattr(attr_name)?.extract::<Vec<u8>>()?;
            unknown_fields.append(&mut data);
            self.0
                .setattr(attr_name, PyBytes::new_bound(self.py(), &unknown_fields))?;
        }
        Ok(())
    }

    pub fn get_unknown_fields(&self) -> InteropResult<Vec<u8>> {
        Ok(self
            .0
            .getattr(intern!(self.py(), "_unknown_fields"))?
            .extract()?)
    }
}

impl ToPyObject for BetterprotoMessage<'_> {
    fn to_object(&self, py: Python<'_>) -> pyo3::PyObject {
        self.0.to_object(py)
    }
}
