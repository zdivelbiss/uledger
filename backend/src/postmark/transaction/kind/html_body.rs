use serde::ser::SerializeStruct;
use std::fmt::Debug;

#[derive(Debug)]
pub struct HtmlBody(String);

impl HtmlBody {
    pub fn new(html_body: String) -> Self {
        Self(html_body)
    }
}

impl super::Kind for HtmlBody {
    const FIELDS: usize = 2;

    fn serialize_into<S: SerializeStruct>(&self, serializer: &mut S) -> Result<(), S::Error> {
        serializer.serialize_field("HtmlBody", &self.0)?;

        Ok(())
    }
}
