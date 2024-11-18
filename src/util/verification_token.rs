#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct VerificationToken([u8; Self::COMPELXITY]);

impl VerificationToken {
    pub const COMPELXITY: usize = 3;

    pub fn gen() -> Self {
        Self(rand::random())
    }

    pub fn from_str(v: impl AsRef<str>) -> Result<Self, hex::FromHexError> {
        let mut data = [0u8; Self::COMPELXITY];
        hex::decode_to_slice(v.as_ref(), &mut data)?;

        Ok(Self(data))
    }

    pub fn as_bytes(&self) -> &[u8; 3] {
        &self.0
    }
}

impl std::fmt::Display for VerificationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        hex::encode_upper(self.0).fmt(f)
    }
}
