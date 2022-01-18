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

use common_runtime::constants::*;
use common_runtime::{Balance, BlockNumber};
use frame_support::parameter_types;
use frame_support::weights::constants::WEIGHT_PER_SECOND;
use sp_arithmetic::Permill;
use sp_runtime::transaction_validity::TransactionPriority;

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    /// We allow for 2 seconds of compute with a 6 second average block time.
    pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights
        ::with_sensible_defaults(2 * WEIGHT_PER_SECOND, NORMAL_DISPATCH_RATIO);
    pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
        ::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
    pub const SS58Prefix: u16 = 42;
    pub const UncleGenerations: u32 = 0;
}

/*************/
/* CONSENSUS */
/*************/

// Authority discovery
parameter_types! {
    pub const MaxAuthorities: u32 = 100;
}

// Timestamp
parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

// Babe
pub const EPOCH_DURATION_IN_SLOTS: BlockNumber = 4 * HOURS;
parameter_types! {
    pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS as u64;
    pub const ExpectedBlockTime: u64 = MILLISECS_PER_BLOCK;
    pub const ReportLongevity: u64 = 168 * EpochDuration::get();
}

// ImOnline
parameter_types! {
    pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
    pub const MaxKeys: u32 = 10_000;
    pub const MaxPeerInHeartbeats: u32 = 10_000;
    pub const MaxPeerDataEncodingSize: u32 = 1_000;
}

/*********/
/* MONEY */
/*********/

// Balances
frame_support::parameter_types! {
    pub const ExistentialDeposit: Balance = 500;
    pub const MaxLocks: u32 = 50;
}

// Transaction payment
frame_support::parameter_types! {
    pub const TransactionByteFee: Balance = 0;
}

// Universal dividend
parameter_types! {
    // 0.002_381_440 = 0.0488^2
    pub const SquareMoneyGrowthRate: Permill = Permill::from_parts(2_381_440);
    pub const UdCreationPeriod: BlockNumber = DAYS;
    // TODO: this value will depend on the date of the migration
    pub const UdFirstReeval: BlockNumber = 45 * DAYS;
    pub const UdReevalPeriod: Balance = 182;
    pub const UdReevalPeriodInBlocks: BlockNumber = 2_620_800; // 86400 * 182 / 6
}

/*******/
/* WOT */
/*******/

parameter_types! {
    pub MinCertForUdRight: u8 = 5;
    pub MinCertForCertRight: u8 = 5;
    pub MinCertForCreateIdtyRight: u8 = 5;
}

// Identity
pub const IDTY_CREATE_PERIOD: BlockNumber = 100;
parameter_types! {
    pub const ConfirmPeriod: BlockNumber = 14 * DAYS;
    pub const FirstIssuableOn: BlockNumber = 30* DAYS;
    pub const IdtyCreationPeriod: BlockNumber = MONTHS;
    pub const MaxNoRightPeriod: BlockNumber = YEARS;
    pub const ValidationPeriod: BlockNumber = YEARS;
}

// Membership
parameter_types! {
    pub const MembershipPeriod: BlockNumber = YEARS;
    pub const PendingMembershipPeriod: BlockNumber = 2 * MONTHS;
    pub const RenewablePeriod: BlockNumber = 2 * MONTHS;
}

// Certification
pub const MIN_STRONG_CERT_FOR_UD: u32 = 5;
pub const MIN_STRONG_CERT_FOR_STRONG_CERT: u32 = 5;
parameter_types! {
    pub const CertPeriod: BlockNumber = 5 * DAYS;
    pub const MaxByIssuer: u8 = 100;
    pub const StrongCertRenewablePeriod: BlockNumber = 6 * MONTHS;
    pub const ValidityPeriod: BlockNumber = 2 * YEARS;
}

// Multisig
parameter_types! {
    pub const DepositBase: Balance = 1000;
    pub const DepositFactor: Balance = 10;
    pub const MaxSignatories: u16 = 5;
}
