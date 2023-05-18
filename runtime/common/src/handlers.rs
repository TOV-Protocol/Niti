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

use super::entities::*;
use super::{AccountId, IdtyIndex};
use frame_support::dispatch::UnfilteredDispatchable;
use frame_support::instances::{Instance1, Instance2};
use frame_support::pallet_prelude::Weight;
use frame_support::traits::Get;
use frame_support::Parameter;
use pallet_identity::IdtyEvent;
use sp_runtime::traits::IsMember;

// new session handler
pub struct OnNewSessionHandler<Runtime>(core::marker::PhantomData<Runtime>);
impl<Runtime> pallet_authority_members::traits::OnNewSession for OnNewSessionHandler<Runtime>
where
    Runtime: pallet_provide_randomness::Config,
{
    fn on_new_session(_index: sp_staking::SessionIndex) -> Weight {
        pallet_provide_randomness::Pallet::<Runtime>::on_new_epoch();
        Weight::zero()
    }
}

// identity change runtime handler
pub struct OnIdtyChangeHandler<Runtime>(core::marker::PhantomData<Runtime>);
impl<T> pallet_identity::traits::OnIdtyChange<T> for OnIdtyChangeHandler<T>
where
    T: frame_system::Config<AccountId = AccountId>,
    T: pallet_authority_members::Config<MemberId = IdtyIndex>,
    T: pallet_identity::Config<IdtyIndex = IdtyIndex, IdtyData = IdtyData>,
    T: pallet_universal_dividend::Config,
{
    fn on_idty_change(idty_index: IdtyIndex, idty_event: &IdtyEvent<T>) -> Weight {
        match idty_event {
            IdtyEvent::Validated => {
                // when identity is validated, it starts getting right to UD
                // but this is handeled by membership event handler (MembershipAcquired)
            }
            IdtyEvent::ChangedOwnerKey { new_owner_key } => {
                if let Err(e) = pallet_authority_members::Pallet::<T>::change_owner_key(
                    idty_index,
                    new_owner_key.clone(),
                ) {
                    log::error!(
                        "on_idty_change: pallet_authority_members.change_owner_key(): {:?}",
                        e
                    );
                }
            }
            IdtyEvent::Created { .. } | IdtyEvent::Confirmed | IdtyEvent::Removed { .. } => {}
        }
        Weight::zero()
    }
}

// membership event runtime handler
pub struct OnMembershipEventHandler<Inner, Runtime>(core::marker::PhantomData<(Inner, Runtime)>);
impl<
        Inner: sp_membership::traits::OnEvent<IdtyIndex, ()>,
        Runtime: frame_system::Config<AccountId = AccountId>
            + pallet_identity::Config<IdtyData = IdtyData, IdtyIndex = IdtyIndex>
            + pallet_membership::Config<Instance1, MetaData = ()>
            + pallet_universal_dividend::Config,
    > sp_membership::traits::OnEvent<IdtyIndex, ()> for OnMembershipEventHandler<Inner, Runtime>
{
    fn on_event(membership_event: &sp_membership::Event<IdtyIndex, ()>) -> Weight {
        (match membership_event {
            // when membership is removed, call on_removed_member handler which auto claims UD
            sp_membership::Event::MembershipRevoked(idty_index)
            | sp_membership::Event::MembershipExpired(idty_index) => {
                if let Some(idty_value) = pallet_identity::Identities::<Runtime>::get(idty_index) {
                    if let Some(first_ud_index) = idty_value.data.first_eligible_ud.into() {
                        pallet_universal_dividend::Pallet::<Runtime>::on_removed_member(
                            first_ud_index,
                            &idty_value.owner_key,
                        )
                    } else {
                        Runtime::DbWeight::get().reads(1)
                    }
                } else {
                    Runtime::DbWeight::get().reads(1)
                }
            }
            // when main membership is acquired, it starts getting right to UD
            sp_membership::Event::MembershipAcquired(idty_index, _) => {
                pallet_identity::Identities::<Runtime>::mutate_exists(idty_index, |idty_val_opt| {
                    if let Some(ref mut idty_val) = idty_val_opt {
                        idty_val.data = IdtyData {
                            first_eligible_ud:
                                pallet_universal_dividend::Pallet::<Runtime>::init_first_eligible_ud(
                                ),
                        }
                    }
                });
                Weight::zero()
            }
            // in other case, ther is nothing to do
            sp_membership::Event::MembershipRenewed(_)
            | sp_membership::Event::MembershipRequested(_)
            | sp_membership::Event::PendingMembershipExpired(_) => Weight::zero(),
        }) + Inner::on_event(membership_event)
    }
}

// smith membership event handler
pub struct OnSmithMembershipEventHandler<Inner, Runtime>(
    core::marker::PhantomData<(Inner, Runtime)>,
);
impl<
        IdtyIndex: Copy + Parameter,
        SessionKeysWrapper: Clone,
        Inner: sp_membership::traits::OnEvent<IdtyIndex, SmithMembershipMetaData<SessionKeysWrapper>>,
        Runtime: frame_system::Config<AccountId = AccountId>
            + pallet_identity::Config<IdtyIndex = IdtyIndex>
            + pallet_authority_members::Config<KeysWrapper = SessionKeysWrapper, MemberId = IdtyIndex>
            + pallet_membership::Config<
                Instance2,
                MetaData = SmithMembershipMetaData<SessionKeysWrapper>,
            >,
    > sp_membership::traits::OnEvent<IdtyIndex, SmithMembershipMetaData<SessionKeysWrapper>>
    for OnSmithMembershipEventHandler<Inner, Runtime>
{
    fn on_event(
        membership_event: &sp_membership::Event<
            IdtyIndex,
            SmithMembershipMetaData<SessionKeysWrapper>,
        >,
    ) -> Weight {
        (match membership_event {
            sp_membership::Event::MembershipAcquired(
                _idty_index,
                SmithMembershipMetaData {
                    owner_key,
                    session_keys,
                    ..
                },
            ) => {
                let call = pallet_authority_members::Call::<Runtime>::set_session_keys {
                    keys: session_keys.clone(),
                };
                if let Err(e) = call.dispatch_bypass_filter(
                    frame_system::Origin::<Runtime>::Signed(owner_key.clone()).into(),
                ) {
                    sp_std::if_std! {
                        println!("fail to set session keys: {:?}", e)
                    }
                }
                Weight::zero()
            }
            sp_membership::Event::MembershipRevoked(idty_index) => {
                let call = pallet_authority_members::Call::<Runtime>::remove_member {
                    member_id: *idty_index,
                };
                if let Err(e) =
                    call.dispatch_bypass_filter(frame_system::Origin::<Runtime>::Root.into())
                {
                    sp_std::if_std! {
                        println!("faid to remove member: {:?}", e)
                    }
                }
                Weight::zero()
            }
            _ => Weight::zero(),
        }) + Inner::on_event(membership_event)
    }
}

// authority member removal handler
pub struct OnRemovedAuthorityMemberHandler<Runtime>(core::marker::PhantomData<Runtime>);
impl<Runtime> pallet_authority_members::traits::OnRemovedMember<IdtyIndex>
    for OnRemovedAuthorityMemberHandler<Runtime>
where
    Runtime: frame_system::Config + pallet_membership::Config<Instance2, IdtyId = IdtyIndex>,
{
    fn on_removed_member(idty_index: IdtyIndex) -> Weight {
        // TODO investigate why we should remove smith membership when removing authority member
        pallet_membership::Pallet::<Runtime, Instance2>::force_revoke_membership(idty_index);
        // TODO investigate why weight zero
        Weight::zero()
    }
}

// identity removal handler
pub struct RemoveIdentityConsumersImpl<Runtime>(core::marker::PhantomData<Runtime>);
impl<Runtime> pallet_identity::traits::RemoveIdentityConsumers<IdtyIndex>
    for RemoveIdentityConsumersImpl<Runtime>
where
    Runtime: pallet_identity::Config<IdtyIndex = IdtyIndex>
        + pallet_authority_members::Config<MemberId = IdtyIndex>
        + pallet_membership::Config<Instance1, IdtyId = IdtyIndex>
        + pallet_membership::Config<Instance2, IdtyId = IdtyIndex>,
{
    fn remove_idty_consumers(idty_index: IdtyIndex) -> Weight {
        // Remove smith member
        if pallet_membership::Pallet::<Runtime, Instance2>::is_member(&idty_index) {
            pallet_membership::Pallet::<Runtime, Instance2>::force_revoke_membership(idty_index);
        }
        // Remove "classic" member
        pallet_membership::Pallet::<Runtime, Instance1>::force_revoke_membership(idty_index);

        Weight::zero()
    }
}

// spend treasury handler
pub struct TreasurySpendFunds<Runtime>(core::marker::PhantomData<Runtime>);
impl<Runtime> pallet_treasury::SpendFunds<Runtime> for TreasurySpendFunds<Runtime>
where
    Runtime: pallet_treasury::Config,
{
    fn spend_funds(
        _budget_remaining: &mut pallet_treasury::BalanceOf<Runtime>,
        _imbalance: &mut pallet_treasury::PositiveImbalanceOf<Runtime>,
        _total_weight: &mut Weight,
        missed_any: &mut bool,
    ) {
        *missed_any = true;
    }
}
