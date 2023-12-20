// Copyright 2021-2023 Axiom-Team
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

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{account, benchmarks};
use sp_runtime::traits::One;

fn assert_has_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

benchmarks! {
    where_clause {
        where
            IdtyId<T>: From<u32>,
            BalanceOf<T>: From<u64>,
            T::AccountId: From<[u8; 32]>,
    }
    queue_refund {
        let account: T::AccountId = account("Alice", 1, 1);
        let dummy_refund = Refund {
            account: account.clone(),
            identity: 0u32.into(),
            amount: 20u64.into(),
        };
        let refund = Refund {
            account,
            identity: 1u32.into(),
            amount: 10u64.into(),
        };
        // Complexity is bound to MAX_QUEUD_REFUNDS where an insertion is O(n-1)
        for i in 0..MAX_QUEUED_REFUNDS-1 {
            Pallet::<T>::queue_refund(dummy_refund.clone())
        }
    }: { Pallet::<T>::queue_refund(refund.clone()) }
    verify {
        assert_eq!(RefundQueue::<T>::get().last(), Some(refund).as_ref());
        assert_eq!(RefundQueue::<T>::get().len() as u32, MAX_QUEUED_REFUNDS);
    }
    spend_quota {
        let idty_id: IdtyId<T> = 1u32.into();
        let amount = 2u64;
        let quota_amount = 10u64;
        IdtyQuota::<T>::insert(
            idty_id,
            Quota {
                last_use: T::BlockNumber::zero(),
                amount: quota_amount.into(),
            },
        );
    }: { Pallet::<T>::spend_quota(idty_id, amount.into()) }
    verify {
        let quota_growth = sp_runtime::Perbill::from_rational(
            T::BlockNumber::one(),
            T::ReloadRate::get(),
        ).mul_floor(T::MaxQuota::get());
        assert_eq!(IdtyQuota::<T>::get(idty_id).unwrap().amount,  quota_growth +quota_amount.into() - amount.into());
    }
    try_refund {
        let account: T::AccountId = account("Alice", 1, 1);
        let idty_id: IdtyId<T> = 1u32.into();
        IdtyQuota::<T>::insert(
            idty_id,
            Quota {
                last_use: T::BlockNumber::zero(),
                amount: 10u64.into(),
            },
        );
        let _ = CurrencyOf::<T>:: make_free_balance_be(
                &T::RefundAccount::get(),u32::MAX.into(),
            );
        // The worst-case scenario is when the refund fails
        // and can only be triggered if the account is dead,
        // in this case by having no balance in the account.
        let refund = Refund {
            account: account.clone(),
            identity: 1u32.into(),
            amount: 10u64.into(),
        };
    }: { Pallet::<T>::try_refund(refund) }
    verify {
        assert_has_event::<T>(Event::<T>::RefundFailed ( account ).into());
    }
    do_refund {
        let account: T::AccountId = account("Alice", 1, 1);
        let _ = CurrencyOf::<T>:: make_free_balance_be(
                &T::RefundAccount::get(),u32::MAX.into(),
            );
        // The worst-case scenario is when the refund fails
        // and can only be triggered if the account is dead,
        // in this case by having no balance in the account.
        let refund = Refund {
            account: account.clone(),
            identity: 1u32.into(),
            amount: 10u64.into(),
        };
    }: { Pallet::<T>::try_refund(refund) }
    verify {
        assert_has_event::<T>(Event::<T>::RefundFailed ( account ).into());
    }
    // The base weight consumed on processing refund queue when empty.
    on_process_refund_queue {
        assert_eq!(RefundQueue::<T>::get().len() as u32, 0);
    }: { Pallet::<T>::process_refund_queue(Weight::MAX) }
    // The weight consumed on processing refund queue with one element.
    // Can deduce the process_refund_queue overhead by subtracting try_refund weight.
    #[pov_mode = Measured]
    on_process_refund_queue_elements {
        let i in 1..MAX_QUEUED_REFUNDS;
        let account: T::AccountId = account("Alice", 1, 1);
        let idty_id: IdtyId<T> = 1u32.into();
        IdtyQuota::<T>::insert(
            idty_id,
            Quota {
                last_use: T::BlockNumber::zero(),
                amount: 10u64.into(),
            },
        );
        let _ = CurrencyOf::<T>:: make_free_balance_be(
                &T::RefundAccount::get(),u32::MAX.into(),
            );
        // The worst-case scenario is when the refund fails
        // and can only be triggered if the account is dead,
        // in this case by having no balance in the account.
        let refund = Refund {
            account: account.clone(),
            identity: 1u32.into(),
            amount: 10u64.into(),
        };
        for _ in 0..i {
            Pallet::<T>::queue_refund(refund.clone());
        }
        assert_eq!(RefundQueue::<T>::get().len() as u32, i);
    }: { Pallet::<T>::process_refund_queue(Weight::MAX) }
    verify {
        assert_eq!(RefundQueue::<T>::get().len() as u32, 0);
        assert_has_event::<T>(Event::<T>::RefundFailed ( account ).into());
    }
    impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(crate::mock::QuotaConfig{identities: vec![1, 2]}), crate::mock::Test);
}
