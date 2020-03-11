use crate::{Read, Write, NumBytes};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Read, Write, NumBytes, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Default)]
#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[eosio_core_root_path = "crate"]
pub struct Extension(pub u16, pub Vec<u8>);

impl Extension {
    pub fn new(bytes: Vec<u8>) -> Self {
        let mut array = [0; 2];
        let u16_bytes = &bytes[..array.len()];
        array.copy_from_slice(u16_bytes);
        let u_16 = u16::from_le_bytes(array);
        Extension(u_16, bytes[2..].to_vec())
    }
}

impl core::fmt::Display for Extension {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}, {}", self.0, hex::encode(&self.1))
    }
}
