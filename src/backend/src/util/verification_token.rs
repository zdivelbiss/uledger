pub struct VerificationToken([u8; Self::SIZE]);

impl VerificationToken {
    pub const SIZE: usize = 3;

    pub fn gen() -> Self {
        Self(rand::random())
    }

    pub fn from_str(value: impl AsRef<str>) -> Result<Self, hex::FromHexError> {
        let mut data = [0u8; Self::SIZE];
        hex::decode_to_slice(value.as_ref(), &mut data)?;

        Ok(Self(data))
    }
}

impl std::fmt::Display for VerificationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        hex::encode(self.0).fmt(f)
    }
}

impl serde::Serialize for VerificationToken {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
