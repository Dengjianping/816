/*
todo: test
*/

#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::{ensure_root};
use frame_support::{decl_module, decl_event, decl_error, decl_storage, ensure};
use sp_std::prelude::*;
// token primitives
use token_primitives::{Symbol, Token};

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

decl_event! {
	pub enum Event<T> where <T as frame_system::Trait>::AccountId,
	{
		TokenIssued(AccountId, Symbol, u128),
		TokenDestroyed(AccountId, Symbol, u128),
	}
}

decl_error! {
	pub enum Error for Module<T: Trait> {
		ZeroBalance,
		UserHasNoThisToken,
		DestroyTooMuch,
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Token {
		pub AccountToken get(fn token): map hasher(blake2_128_concat) (T::AccountId, Symbol) => Token;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 10000]
		pub fn issue(origin, who: T::AccountId, symbol: Symbol, amount: u128) {
            ensure_root(origin)?;

			ensure!(amount != 0u128, Error::<T>::ZeroBalance);

			// check user has the token or not
			if <AccountToken<T>>::contains_key((&who, symbol)) {
				<AccountToken<T>>::mutate((&who, symbol), |token|{
					token.balance = token.balance.saturating_add(amount);
				});
			} else {
				// create token and issue it to user
				let token = Token {
					symbol,
					balance: amount,
				};
				<AccountToken<T>>::insert((&who, symbol), token);
			}


			Self::deposit_event(RawEvent::TokenIssued(who, symbol, amount));
		}
		
		// [weight = 10000]
		// pub fn transfer(origin, to: T::AccountId, symbol: Symbol, amount: u128) {
        //     todo!("someone might help to impl it.");
        // }

        #[weight = 10000]
		pub fn destroy(origin, who: T::AccountId, symbol: Symbol, amount: u128) {
            ensure_root(origin)?;

			// ensure this user has the token
			ensure!(<AccountToken<T>>::contains_key((&who, symbol)), Error::<T>::UserHasNoThisToken);
			// ensure this user has enough to destroy 
			ensure!(<AccountToken<T>>::get((&who, symbol)).balance >= amount, Error::<T>::DestroyTooMuch);

			// destroy token
			<AccountToken<T>>::mutate((&who, symbol), |token| {
				token.balance = token.balance.saturating_sub(amount);
			});

			Self::deposit_event(RawEvent::TokenDestroyed(who, symbol, amount));
		}
	}
}

// impl this trait for others module can access and modyfy token'data
impl<T: Trait> token_primitives::TokenTrait<T::AccountId> for Module<T> {
	fn decrease(who: &T::AccountId, symbol: Symbol, amount: u128) {
		<AccountToken<T>>::mutate((who, symbol), |token| {
			token.balance = token.balance.saturating_sub(amount);
		});
	}

	fn increase(who: &T::AccountId, symbol: Symbol, amount: u128) {
		<AccountToken<T>>::mutate((who, symbol), |token| {
			token.balance = token.balance.saturating_add(amount);
		});
	}

	fn get_token(who: &T::AccountId, symbol: Symbol) -> Token {
		<AccountToken<T>>::get((who, symbol))
	}

	fn has_token(who: &T::AccountId, symbol: Symbol) -> bool {
		<AccountToken<T>>::contains_key((who, symbol))
	}
}