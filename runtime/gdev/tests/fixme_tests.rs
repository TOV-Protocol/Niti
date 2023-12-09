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

// these integration tests show current behavior that is counter-intuitive or outside specs
// they should failed after the related issue is fixed

mod common;

use common::*;
use frame_support::assert_ok;
use gdev_runtime::*;
use pallet_membership::MembershipRemovalReason;
use sp_keyring::AccountKeyring;

/// issue #136
/// a smith should not be able to add a smith cert to an
/// identity who has no smith membership or pending membership
#[test]
fn can_add_smith_cert_without_pending_membership_or_membership() {
    // 3 smith (1. Alice, 2. Bob, 3. Charlie)
    // 4 identities (4. Dave)
    ExtBuilder::new(1, 3, 4).build().execute_with(|| {
        run_to_block(1);

        // FIXME Bob can add new smith cert to to dave even he did not requested smith membership
        assert_ok!(SmithCert::add_cert(
            frame_system::RawOrigin::Signed(AccountKeyring::Bob.to_account_id()).into(),
            2, // bob
            4  // dave
        ));
    });
}

/// issue #136
/// an identity should not be able to add cert
/// when its membership is suspended
#[test]
fn can_still_issue_cert_when_membership_lost() {
    ExtBuilder::new(1, 3, 4).build().execute_with(|| {
        run_to_block(1);

        // expire Bob membership (could happen for negative distance evaluation for example)
        assert_ok!(Membership::force_expire_membership(
            2, // Bob
        ));
        System::assert_has_event(RuntimeEvent::Membership(
            pallet_membership::Event::MembershipRemoved {
                member: 2,
                reason: MembershipRemovalReason::Expired,
            },
        ));

        // FIXME this should not be possible
        assert_ok!(Cert::add_cert(
            frame_system::RawOrigin::Signed(AccountKeyring::Bob.to_account_id()).into(),
            2, // Bob
            3, // Charlie
        ));
        System::assert_has_event(RuntimeEvent::Cert(
            pallet_certification::Event::CertRenewed {
                issuer: 2,
                receiver: 3,
            },
        ));
    });
}