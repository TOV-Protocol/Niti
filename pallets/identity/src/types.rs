// Copyright 2021 Axiom-Team
//
// This file is part of Substrate-Libre-Currency.
//
// Substrate-Libre-Currency is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// Substrate-Libre-Currency is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Substrate-Libre-Currency. If not, see <https://www.gnu.org/licenses/>.

//! Various basic types for use in the certification pallet.

use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;

pub enum IdtyEvent<T: crate::Config> {
    Created { creator: T::IdtyIndex },
    Confirmed,
    Validated,
    Removed,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, PartialOrd, Ord, RuntimeDebug)]
pub struct IdtyName(pub Vec<u8>);

impl scale_info::TypeInfo for IdtyName {
    type Identity = str;

    fn type_info() -> scale_info::Type {
        Self::Identity::type_info()
    }
}

#[cfg(feature = "std")]
impl From<&str> for IdtyName {
    fn from(s: &str) -> Self {
        Self(s.as_bytes().to_vec())
    }
}

#[cfg(feature = "std")]
impl serde::Serialize for IdtyName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        std::str::from_utf8(&self.0)
            .map_err(|e| serde::ser::Error::custom(format!("{:?}", e)))?
            .serialize(serializer)
    }
}

#[cfg(feature = "std")]
impl<'de> serde::Deserialize<'de> for IdtyName {
    fn deserialize<D: serde::Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        Ok(Self(String::deserialize(de)?.as_bytes().to_vec()))
    }
}

#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum IdtyStatus {
    Created,
    ConfirmedByOwner,
    Validated,
}
impl Default for IdtyStatus {
    fn default() -> Self {
        IdtyStatus::Created
    }
}

#[cfg_attr(feature = "std", derive(Deserialize, Serialize))]
#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
pub struct IdtyValue<
    AccountId: Decode + Encode + TypeInfo,
    BlockNumber: Decode + Encode + TypeInfo,
    IdtyData: Decode + Encode + TypeInfo,
    IdtyRight: Decode + Encode + TypeInfo,
> {
    pub data: IdtyData,
    pub owner_key: AccountId,
    pub name: IdtyName,
    pub next_creatable_identity_on: BlockNumber,
    pub removable_on: BlockNumber,
    pub rights: Vec<(IdtyRight, Option<AccountId>)>,
    pub status: IdtyStatus,
}

impl<AccountId, BlockNumber, IdtyData, IdtyRight>
    IdtyValue<AccountId, BlockNumber, IdtyData, IdtyRight>
where
    AccountId: Clone + Decode + Encode + TypeInfo,
    BlockNumber: Decode + Encode + TypeInfo,
    IdtyData: Decode + Encode + TypeInfo,
    IdtyRight: crate::traits::IdtyRight + Decode + Encode + TypeInfo,
{
    pub fn get_right_key(&self, right: IdtyRight) -> Option<AccountId> {
        if let Ok(index) = self
            .rights
            .binary_search_by(|(right_, _)| right_.cmp(&right))
        {
            if self.rights[index].1.is_some() {
                self.rights[index].1.clone()
            } else if right.allow_owner_key() {
                Some(self.owner_key.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}
