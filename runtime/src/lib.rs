pub mod cli;
pub mod pallets;

mod conn;
mod error;
mod retry;
mod rpc;
mod types;

#[cfg(test)]
mod tests;

#[cfg(feature = "testing-utils")]
pub mod integration;

pub use error::{Error, SubxtError};
pub use pallets::*;
pub use retry::{notify_retry, RetryPolicy};
pub use rpc::{
    BtcRelayPallet, BtcTxFeesPerByte, CollateralBalancesPallet, ExchangeRateOraclePallet, FeePallet, IssuePallet,
    PolkaBtcProvider, RedeemPallet, RefundPallet, ReplacePallet, SecurityPallet, StakedRelayerPallet, TimestampPallet,
    UtilFuncs, VaultRegistryPallet,
};
pub use sp_arithmetic::{traits as FixedPointTraits, FixedI128, FixedPointNumber, FixedU128};
pub use sp_runtime;
pub use substrate_subxt;
pub use types::*;

use sp_runtime::{
    generic::Header,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, OpaqueExtrinsic,
};
use std::collections::BTreeSet;
use substrate_subxt::{
    balances, extrinsic::DefaultExtra, register_default_type_sizes, sudo, system, system::SystemEventTypeRegistry,
    EventTypeRegistry, Runtime,
};

// cumulus / polkadot types
use parachain::primitives::{Id as ParaId, RelayChainBlockNumber};
use xcm::v0::{Error as XcmError, NetworkId};

pub const TX_FEES: u128 = 2000000000;
pub const PLANCK_PER_DOT: u128 = 10000000000;

pub const MILLISECS_PER_BLOCK: u64 = 6000;

// These time units are defined in number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;

pub type Balance = u128;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PolkaBtcRuntime;

impl Runtime for PolkaBtcRuntime {
    type Signature = MultiSignature;
    type Extra = DefaultExtra<Self>;

    fn register_type_sizes(registry: &mut EventTypeRegistry<Self>) {
        registry.with_core();
        registry.with_system();
        register_default_type_sizes(registry);
    }
}

pub type Index = u32;

/// An index to a block.
pub type BlockNumber = u32;

/// Some way of identifying an account on the chain.
pub type AccountId = <<MultiSignature as Verify>::Signer as IdentifyAccount>::AccountId;

// TODO: use types from actual runtime
impl system::System for PolkaBtcRuntime {
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Address = Self::AccountId;
    type Header = Header<Self::BlockNumber, BlakeTwo256>;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = balances::AccountData<Balance>;
}

impl pallets::Core for PolkaBtcRuntime {
    type Balance = Balance;
    type Collateral = Balance;
    type Wrapped = Balance;
    type BTCBalance = Balance;
    type RichBlockHeader = PolkaBtcRichBlockHeader;
    type H256Le = H256Le;
    type H160 = H160;
    type H256 = H256;
    type BtcAddress = BtcAddress;
    type BtcPublicKey = BtcPublicKey;
    type ErrorCode = ErrorCode;
    type ErrorCodes = BTreeSet<ErrorCode>;
    type StatusCode = StatusCode;
    type SignedFixedPoint = FixedI128;
    type UnsignedFixedPoint = FixedU128;
    type VaultStatus = VaultStatus;
    type RedeemRequestStatus = RedeemRequestStatus;

    // cumulus / polkadot types
    type XcmError = XcmError;
    type NetworkId = NetworkId;
    type RelayChainBlockNumber = RelayChainBlockNumber;
    type ParaId = ParaId;
}

impl balances::Balances for PolkaBtcRuntime {
    type Balance = Balance;
}

impl btc_relay::BTCRelay for PolkaBtcRuntime {}

impl security::Security for PolkaBtcRuntime {}

impl staked_relayers::StakedRelayers for PolkaBtcRuntime {}

impl vault_registry::VaultRegistry for PolkaBtcRuntime {}

impl timestamp::Timestamp for PolkaBtcRuntime {
    type Moment = u64;
}

impl exchange_rate_oracle::ExchangeRateOracle for PolkaBtcRuntime {}

impl collateral_balances::Collateral for PolkaBtcRuntime {
    type Balance = Balance;
}

impl collateral_currency::CollateralCurrency for PolkaBtcRuntime {}

impl wrapped_balances::Wrapped for PolkaBtcRuntime {
    type Balance = Balance;
}

impl wrapped_currency::WrappedCurrency for PolkaBtcRuntime {}

impl issue::Issue for PolkaBtcRuntime {}

impl frame_system::System for PolkaBtcRuntime {}

impl redeem::Redeem for PolkaBtcRuntime {}

impl replace::Replace for PolkaBtcRuntime {}

impl refund::Refund for PolkaBtcRuntime {}

impl sudo::Sudo for PolkaBtcRuntime {}

impl fee::Fee for PolkaBtcRuntime {}

impl sla::Sla for PolkaBtcRuntime {}

impl utility::Utility for PolkaBtcRuntime {}

pub const BTC_RELAY_MODULE: &str = "BTCRelay";
pub const ISSUE_MODULE: &str = "Issue";
pub const REDEEM_MODULE: &str = "Redeem";

pub const STABLE_BITCOIN_CONFIRMATIONS: &str = "StableBitcoinConfirmations";
pub const STABLE_PARACHAIN_CONFIRMATIONS: &str = "StableParachainConfirmations";

pub const DUPLICATE_BLOCK_ERROR: &str = "DuplicateBlock";
pub const INVALID_CHAIN_ID_ERROR: &str = "InvalidChainID";
pub const ISSUE_COMPLETED_ERROR: &str = "IssueCompleted";
pub const COMMIT_PERIOD_EXPIRED_ERROR: &str = "CommitPeriodExpired";
