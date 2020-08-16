#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::ensure_signed;
use frame_support::{decl_module, decl_event, decl_error, decl_storage, ensure};
use sp_std::prelude::*;
// token primitives
use token_primitives::{Symbol, TokenTrait};

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    
    /// Token trait handler
    type TokenTrait: TokenTrait<Self::AccountId>;
}

decl_event! {
	pub enum Event<T> where <T as frame_system::Trait>::AccountId,
	{
		ExchangeUSD(AccountId, u128, u128),
		ExchangeRMB(AccountId, u128, u128),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		ZeroBalance,
		UserHasNoThisToken,
        ExchangeTooMuch,
        ExchangeBetweenSameSymbol,
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as TokenExchange {
		pub ExchangeRate get(fn get_exchange_rate): u16 = 7; // default value, 1usd =  7rmb
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 10000]
		pub fn exchange(origin, exchange_symbol: Symbol, amount: u128, target_symbol: Symbol) {
            let exchanger = ensure_signed(origin)?;

            ensure!(amount != 0u128, Error::<T>::ZeroBalance);
            ensure!(exchange_symbol != target_symbol, Error::<T>::ExchangeBetweenSameSymbol);
            ensure!(
                T::TokenTrait::has_token(&exchanger, exchange_symbol) && T::TokenTrait::has_token(&exchanger, target_symbol),
                Error::<T>::UserHasNoThisToken
            );
            ensure!(
                T::TokenTrait::get_token(&exchanger, exchange_symbol).balance >= amount,
                Error::<T>::ExchangeTooMuch
            );

			match (exchange_symbol, target_symbol) {
                (Symbol::USD, Symbol::RMB) => {
                    let rmb = amount * (ExchangeRate::get() as u128);

                    // destroy usd from exchanger
                    T::TokenTrait::decrease(&exchanger, exchange_symbol, amount);
                    // issue rmb to exchanger
                    T::TokenTrait::increase(&exchanger, target_symbol, rmb);

                    Self::deposit_event(RawEvent::ExchangeRMB(exchanger, amount, rmb));
                }
                (Symbol::RMB, Symbol::USD) => {
                    let usd = amount / (ExchangeRate::get() as u128);

                    // destroy usd from exchanger
                    T::TokenTrait::decrease(&exchanger, exchange_symbol, amount);
                    // issue rmb to exchanger
                    T::TokenTrait::increase(&exchanger, target_symbol, usd);

                    Self::deposit_event(RawEvent::ExchangeUSD(exchanger, amount, usd));
                }
                _ => unreachable!(),
            }
        }
    }
}
