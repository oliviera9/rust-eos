//! <https://github.com/EOSIO/eosio.cdt/blob/4985359a30da1f883418b7133593f835927b8046/libraries/eosiolib/contracts/eosio/action.hpp#L249-L274>
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use core::str::FromStr;

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
#[cfg(feature = "std")]
use serde::ser::{Serializer, SerializeStruct};
use bitcoin_hashes::{ sha256, Hash };

use crate::{
    Read,
    Write,
    Asset,
    NumBytes,
    ActionName,
    AccountName,
    Checksum256,
    SerializeData,
    PermissionLevel,
    utils::convert_hex_string_to_checksum256,
};

pub type AuthSequences = Vec<AuthSequence>;
pub type ActionReceipts = Vec<ActionReceipt>;

#[cfg_attr(feature = "std", derive(Deserialize))]
#[derive(
    Clone,
    Debug,
    Serialize,
    Read,
    Write,
    NumBytes,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[eosio_core_root_path = "crate"]
pub struct ActionReceipt {
    pub recipient: AccountName,
    pub act_digest: Checksum256,
    pub global_sequence: u64,
    pub recv_sequence: u64,
    pub auth_sequence: AuthSequences,
    pub code_sequence: usize,
    pub abi_sequence: usize,
}

impl SerializeData for ActionReceipt {}

impl ActionReceipt {
    pub fn new(
        recipient: &str,
        act_digest_string: &str,
        recv_sequence: u64,
        abi_sequence: usize,
        global_sequence: u64,
        code_sequence: usize,
        auth_sequences: AuthSequences,
    ) -> crate::Result<Self> {
        Ok(
            ActionReceipt {
                abi_sequence,
                code_sequence,
                recv_sequence,
                global_sequence,
                auth_sequence: auth_sequences,
                act_digest: convert_hex_string_to_checksum256(
                    act_digest_string
                )?,
                recipient: AccountName::from_str(recipient.as_ref())
                        .map_err(crate::Error::from)?,
            }
        )
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.to_serialize_data()
    }

    pub fn to_digest(&self) -> Vec<u8> {
        sha256::Hash::hash(&self.serialize()).to_vec()
    }
}


#[cfg_attr(feature = "std", derive(Deserialize))]
#[derive(
    Clone,
    Debug,
    Serialize,
    Read,
    Write,
    NumBytes,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[eosio_core_root_path = "crate"]
pub struct AuthSequence(AccountName, u64);

impl SerializeData for AuthSequence {}

impl AuthSequence {
    pub fn new(recipient: &str, number: u64) -> crate::Result<Self> {
        Ok(
            AuthSequence(
                AccountName::from_str(recipient.as_ref())
                    .map_err(crate::Error::from)?,
                number,
            )
        )
    }
}

/// This is the packed representation of an action along with meta-data about
/// the authorization levels.
#[cfg_attr(feature = "std", derive(Deserialize))]
#[derive(Clone, Debug, Read, Write, NumBytes, Default)]
#[eosio_core_root_path = "crate"]
pub struct Action {
    /// Name of the account the action is intended for
    pub account: AccountName,
    /// Name of the action
    pub name: ActionName,
    /// List of permissions that authorize this action
    pub authorization: Vec<PermissionLevel>,
    /// Payload data
    pub data: Vec<u8>,
}

impl Action {
    pub fn new(
        account: AccountName,
        name: ActionName,
        authorization: Vec<PermissionLevel>,
        data: Vec<u8>
    ) -> Self {
        Action { account, name, authorization, data }
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.to_serialize_data()
    }

    pub fn to_digest(&self) -> Vec<u8> {
        sha256::Hash::hash(&self.serialize()).to_vec()
    }

    pub fn from_str<T: AsRef<str>, S: SerializeData>(
        account: T,
        name: T,
        authorization: Vec<PermissionLevel>,
        action_data: S
    ) -> crate::Result<Self> {
        let account = AccountName::from_str(account.as_ref()).map_err(crate::Error::from)?;
        let name =  ActionName::from_str(name.as_ref()).map_err(crate::Error::from)?;
        let data = action_data.to_serialize_data();

        Ok(Action { account, name, authorization, data })
    }

    pub fn transfer<T: AsRef<str>>(from: T, to: T, quantity: T, memo: T) -> crate::Result<Action> {
        let permission_level = PermissionLevel::from_str(from.as_ref(), "active")?;
        let action_transfer = ActionTransfer::from_str(from, to, quantity, memo)?;
        Action::from_str(
            "eosio.token",
            "transfer",
            vec![permission_level],
            action_transfer
        )
    }
}

impl SerializeData for Action {}

impl core::fmt::Display for Action {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "account: {}\n\
            name: {}\n\
            authorization: {}\n\
            hex_data: {}",
            self.account,
            self.name,
            self.authorization.iter().map(|item| format!("{}", item)).collect::<String>(),
            hex::encode(&self.data),
        )
    }
}

#[cfg(feature = "std")]
impl serde::ser::Serialize for Action {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("Action", 5)?;
        state.serialize_field("account", &self.account)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("authorization", &self.authorization)?;
        // state.serialize_field("hex_data", &hex::encode(&self.data))?;
        // match (self.account.to_string().as_str(), self.name.to_string().as_str()) {
        //     ("eosio.token", "transfer") => {
        //         let data = ActionTransfer::read(&self.data, &mut 0).expect("Action read from data failed.");
        //         state.serialize_field("data", &data)?;
        //     },
        //     _ => {}
        // }
        state.end()
    }
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Read, Write, NumBytes, Default)]
#[eosio_core_root_path = "crate"]
pub struct ActionTransfer {
    pub from: AccountName,
    pub to: AccountName,
    pub quantity: Asset,
    pub memo: String,
}

impl ActionTransfer {
    pub fn new(from: AccountName, to: AccountName, quantity: Asset, memo: String) -> Self {
        ActionTransfer { from, to, quantity, memo }
    }

    pub fn from_str<T: AsRef<str>>(from: T, to: T, quantity: T, memo: T) -> crate::Result<Self> {
        let from = AccountName::from_str(from.as_ref()).map_err(crate::Error::from)?;
        let to = AccountName::from_str(to.as_ref()).map_err(crate::Error::from)?;
        let quantity = Asset::from_str(quantity.as_ref()).map_err(crate::Error::from)?;
        let memo = memo.as_ref().to_string();

        Ok(ActionTransfer { from, to, quantity, memo })
    }
}

impl SerializeData for ActionTransfer {}

pub trait ToAction: Write + NumBytes {
    const NAME: u64;

    #[inline]
    fn to_action(
        &self,
        account: AccountName,
        authorization: Vec<PermissionLevel>,
    ) -> Action {
        let mut data = vec![0_u8; self.num_bytes()];
        self.write(&mut data, &mut 0).expect("write");

        Action {
            account,
            name: Self::NAME.into(),
            authorization,
            data,
        }
    }
}

#[cfg(test)]
mod tests {
    use hex;

    use super::*;

    #[test]
    fn action_should_work() {
        let action = Action::transfer("testa", "testb", "1.0000 EOS", "a memo").ok().unwrap();
        let data = action.to_serialize_data();
        assert_eq!(
            hex::encode(data),
            "00a6823403ea3055000000572d3ccdcd01000000000093b1ca00000000a8ed323227000000000093b1ca000000008093b1ca102700000000000004454f53000000000661206d656d6f"
        );
    }
}
