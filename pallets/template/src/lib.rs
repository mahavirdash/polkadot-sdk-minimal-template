#![cfg_attr(not(feature = "std"), no_std)]
use frame::{prelude::*, testing_prelude::Blake2_128Concat};
use sp_runtime::traits::Zero;

pub use pallet::*;

#[frame::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn get_balance)]
    pub type Balance<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, T::Balance, ValueQuery>;

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId", T::Balance = "Balance")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        BalanceUpdated(T::AccountId, T::Balance),
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn update_balance(origin: OriginFor<T>, to: T::AccountId, amount: T::Balance) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            Balance::<T>::insert(&to, amount);

            Self::deposit_event(Event::BalanceUpdated(to, amount));
            Ok(().into())
        }
    }
}