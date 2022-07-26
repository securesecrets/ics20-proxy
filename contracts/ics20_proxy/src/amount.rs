use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use std::convert::TryInto;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, Uint128};
use crate::error::ContractError;

#[cw_serde]
pub struct Snip20Coin {
    pub address: String,
    pub amount: Uint128,
}

#[cw_serde]
pub enum Amount {
    Native(Coin),
    // FIXME? USe Snip20CoinVerified, and validate cw20 addresses
    Snip20(Snip20Coin),
}

impl Amount {
    // TODO: write test for this
    pub fn from_parts(denom: String, amount: Uint128) -> Self {
        if denom.starts_with("cw20:") {
            let address = denom.get(5..).unwrap().into();
            Amount::Snip20(Snip20Coin { address, amount })
        } else {
            Amount::Native(Coin { denom, amount })
        }
    }

    pub fn snip20(amount: u128, addr: &str) -> Self {
        Amount::Snip20(Snip20Coin {
            address: addr.into(),
            amount: Uint128::new(amount),
        })
    }

    pub fn native(amount: u128, denom: &str) -> Self {
        Amount::Native(Coin {
            denom: denom.to_string(),
            amount: Uint128::new(amount),
        })
    }
}

impl Amount {
    pub fn denom(&self) -> String {
        match self {
            Amount::Native(c) => c.denom.clone(),
            Amount::Snip20(c) => format!("cw20:{}", c.address.as_str()),
        }
    }

    pub fn amount(&self) -> Uint128 {
        match self {
            Amount::Native(c) => c.amount,
            Amount::Snip20(c) => c.amount,
        }
    }

    /// convert the amount into u64
    pub fn u64_amount(&self) -> Result<u64, ContractError> {
        Ok(self.amount().u128().try_into()?)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Amount::Native(c) => c.amount.is_zero(),
            Amount::Snip20(c) => c.amount.is_zero(),
        }
    }
}
