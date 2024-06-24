// Copyright 2021 Axiom-Team
//
// This file is part of Duniter-v2S.
//
// Duniter-v2S is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// Duniter-v2S is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with Duniter-v2S. If not, see <https://www.gnu.org/licenses/>.

//! Various basic types for use in the identity pallet.

use crate::SmithStatus;
use codec::{Decode, Encode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_staking::SessionIndex;

/// Represents a certification metadata attached to a Smith identity.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct SmithMeta<IdtyIndex> {
    /// Current status of the Smith.
    pub status: SmithStatus,
    /// The session at which the Smith will expire (for lack of validation activity).
    pub expires_on: Option<SessionIndex>,
    /// Certifications issued to other Smiths.
    pub issued_certs: sp_std::vec::Vec<IdtyIndex>,
    /// Certifications received from other Smiths.
    pub received_certs: sp_std::vec::Vec<IdtyIndex>,
}

/// By default, a smith has the least possible privileges
impl<IdtyIndex> Default for SmithMeta<IdtyIndex> {
    fn default() -> Self {
        Self {
            status: SmithStatus::Excluded,
            expires_on: None,
            issued_certs: sp_std::vec::Vec::<IdtyIndex>::new(),
            received_certs: sp_std::vec::Vec::<IdtyIndex>::new(),
        }
    }
}
