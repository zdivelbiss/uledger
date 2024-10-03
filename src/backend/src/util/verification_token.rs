pub struct VerificationToken([u8; 3]);

impl VerificationToken {
    pub fn gen() -> Self {
        Self(rand::random())
    }
}

impl std::fmt::Display for VerificationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        hex::encode(&self.0).fmt(f)
    }
}

impl serde::Serialize for VerificationToken {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
