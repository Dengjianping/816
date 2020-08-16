#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};

#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, Debug)]
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
#[non_exhaustive]
pub enum Symbol {
    RMB,
    USD,
}

impl Default for Symbol {
	fn default() -> Self {
		Self::RMB
	}
}


#[derive(Encode, Decode, Clone, Copy, Eq, PartialEq, Debug, Default)]
#[cfg_attr(feature = "std", derive(serde::Deserialize, serde::Serialize))]
pub struct Token {
    pub symbol: Symbol,
    pub balance: u128,
}

pub trait TokenTrait<AccountId> {
    fn decrease(who: &AccountId, symbol: Symbol, amount: u128);
    fn increase(who: &AccountId, symbol: Symbol, amount: u128);
    fn get_token(who: &AccountId, symbol: Symbol) -> Token;
    fn has_token(who: &AccountId, symbol: Symbol) -> bool;
}