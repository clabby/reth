use crate::U8;
use reth_codecs::{derive_arbitrary, Compact};
use serde::{Deserialize, Serialize};

/// Identifier for legacy transaction, however [TxLegacy](crate::TxLegacy) this is technically not
/// typed.
pub const LEGACY_TX_TYPE_ID: u8 = 0;

/// Identifier for [TxEip2930](crate::TxEip2930) transaction.
pub const EIP2930_TX_TYPE_ID: u8 = 1;

/// Identifier for [TxEip1559](crate::TxEip1559) transaction.
pub const EIP1559_TX_TYPE_ID: u8 = 2;

/// Identifier for [TxDeposit](crate::TxDeposit) transaction.
#[cfg(feature = "optimism")]
use crate::DEPOSIT_TX_TYPE;

/// Transaction Type
#[derive_arbitrary(compact)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum TxType {
    /// Legacy transaction pre EIP-2929
    #[default]
    Legacy = 0_isize,
    /// AccessList transaction
    EIP2930 = 1_isize,
    /// Transaction with Priority fee
    EIP1559 = 2_isize,
    /// OP Deposit transaction.
    #[cfg(feature = "optimism")]
    DEPOSIT = DEPOSIT_TX_TYPE as isize,
}

impl From<TxType> for u8 {
    fn from(value: TxType) -> Self {
        match value {
            TxType::Legacy => LEGACY_TX_TYPE_ID,
            TxType::EIP2930 => EIP2930_TX_TYPE_ID,
            TxType::EIP1559 => EIP1559_TX_TYPE_ID,
            #[cfg(feature = "optimism")]
            TxType::DEPOSIT => DEPOSIT_TX_TYPE,
        }
    }
}

impl From<TxType> for U8 {
    fn from(value: TxType) -> Self {
        U8::from(u8::from(value))
    }
}

impl Compact for TxType {
    fn to_compact<B>(self, _: &mut B) -> usize
    where
        B: bytes::BufMut + AsMut<[u8]>,
    {
        match self {
            TxType::Legacy => LEGACY_TX_TYPE_ID as usize,
            TxType::EIP2930 => EIP2930_TX_TYPE_ID as usize,
            TxType::EIP1559 => EIP1559_TX_TYPE_ID as usize,
            #[cfg(feature = "optimism")]
            TxType::DEPOSIT => DEPOSIT_TX_TYPE as usize,
        }
    }

    fn from_compact(buf: &[u8], identifier: usize) -> (Self, &[u8]) {
        (
            match identifier {
                0 => TxType::Legacy,
                1 => TxType::EIP2930,
                2 => TxType::EIP1559,
                #[cfg(feature = "optimism")]
                _ => TxType::DEPOSIT,
            },
            buf,
        )
    }
}
