//! <https://github.com/EOSIO/eosio.cdt/blob/796ff8bee9a0fc864f665a0a4d018e0ff18ac383/libraries/eosiolib/contracts/eosio/producer_schedule.hpp#L15-L45>
use crate::{AccountName, NumBytes, PublicKey, Read, Write};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

/// Maps producer with its signing key, used for producer schedule
#[derive(Read, Write, NumBytes, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[eosio_core_root_path = "crate"]
#[repr(C)]
pub struct ProducerKey {
    /// Name of the producer
    pub producer_name: AccountName,

    /// Block signing key used by this producer
    #[cfg_attr(feature = "std", serde(deserialize_with = "super::string_to_public_key"))]
    pub block_signing_key: PublicKey,
}

impl ProducerKey {
    pub fn new(
        producer_name: AccountName,
        block_signing_key: PublicKey
    ) -> Self {
        ProducerKey { producer_name, block_signing_key }
    }
}

#[derive(Read, Write, NumBytes, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[eosio_core_root_path = "crate"]
#[repr(C)]
pub struct ProducerKeyV2 {
    pub producer_name: AccountName,
    pub authority: Authority,
}

pub type Authority = (u8, KeysAndThreshold);

#[derive(Read, Write, NumBytes, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[eosio_core_root_path = "crate"]
#[repr(C)]
pub struct KeysAndThreshold {
    pub threshold: u32,
    pub keys: Vec<Key>,
}

#[derive(Read, Write, NumBytes, Clone, Default, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[eosio_core_root_path = "crate"]
#[repr(C)]
pub struct Key {
    #[cfg_attr(feature = "std", serde(deserialize_with = "super::string_to_public_key"))]
    pub key: PublicKey,
    pub weight: u16,
}
