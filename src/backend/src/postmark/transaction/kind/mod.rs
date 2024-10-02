mod verification;
pub use verification::*;

mod html_body;
pub use html_body::*;

pub trait Kind: std::fmt::Debug {
    const FIELDS: usize;

    fn serialize_into<S: serde::ser::SerializeStruct>(
        &self,
        serializer: &mut S,
    ) -> Result<(), S::Error>;
}
