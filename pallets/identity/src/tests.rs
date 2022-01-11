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

use crate::mock::IdtyRight as Right;
use crate::mock::*;
use crate::{Error, IdtyName};
use frame_support::assert_err;
use frame_support::assert_ok;
use frame_system::{EventRecord, Phase};

#[test]
fn test_no_identity() {
    let identities = Vec::with_capacity(0);
    new_test_ext(IdentityConfig { identities }).execute_with(|| {
        assert_eq!(Identity::identities_count(), 0);
    });
}

#[test]
fn test_two_identities() {
    let identities = vec![
        crate::IdtyValue {
            name: IdtyName(vec![0]),
            expire_on: 5,
            owner_key: 1,
            removable_on: 0,
            renewable_on: 3,
            rights: vec![(Right::Right2, Some(10))],
            status: crate::IdtyStatus::Validated,
            data: (),
        },
        crate::IdtyValue {
            name: IdtyName(vec![1]),
            expire_on: 5,
            owner_key: 2,
            removable_on: 0,
            renewable_on: 3,
            rights: vec![(Right::Right1, Some(20))],
            status: crate::IdtyStatus::Validated,
            data: (),
        },
    ];

    new_test_ext(IdentityConfig { identities }).execute_with(|| {
        // Should have two identities
        assert_eq!(Identity::identities_count(), 2);

        // We need to initialize at least one block before any call
        run_to_block(1);

        // Add right Right1 for IdtyName(vec![0])
        // Should succes and trigger the correct event
        assert_ok!(Identity::add_right(Origin::root(), 1, Right::Right1));
        let events = System::events();
        assert_eq!(events.len(), 1);
        assert_eq!(
            events[0],
            EventRecord {
                phase: Phase::Initialization,
                event: Event::Identity(crate::Event::IdtyAcquireRight(
                    IdtyName(vec![0]),
                    Right::Right1
                )),
                topics: vec![],
            }
        );
        // Add right Right2 for IdtyName(vec![0])
        // Should fail because IdtyName(vec![0]) already have this right
        assert_err!(
            Identity::add_right(Origin::root(), 1, Right::Right2),
            Error::<Test>::RightAlreadyAdded
        );

        run_to_block(3);

        // Delete right Right1 for IdtyName(vec![1])
        // Should succes and trigger the correct event
        assert_ok!(Identity::del_right(Origin::root(), 2, Right::Right1));
        let events = System::events();
        assert_eq!(events.len(), 2);
        assert_eq!(
            events[1],
            EventRecord {
                phase: Phase::Initialization,
                event: Event::Identity(crate::Event::IdtyLostRight(
                    IdtyName(vec![1]),
                    Right::Right1
                )),
                topics: vec![],
            }
        );

        // The IdtyName(vec![1]) identity has no more rights, the inactivity period must start to run
        let idty2 = Identity::identity(2).expect("idty not found");
        assert!(idty2.rights.is_empty());
        assert_eq!(idty2.removable_on, 7);
    });
}
