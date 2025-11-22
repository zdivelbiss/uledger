pub mod profile;
pub mod verification;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    sqlx::Type,
)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "USER_ACCESS", rename_all = "UPPERCASE")]
pub enum Access {
    Admin,
    Regular,
}
