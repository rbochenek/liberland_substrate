#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  #[pallet::without_storage_info]
  pub struct Pallet<T>(_);

  #[pallet::config]
  pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
  }
  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    LawAdded { who: T::AccountId, index1: u32, index2: u32 },
    LawRepealed { who: T::AccountId, index1: u32, index2: u32 },
  }
  #[pallet::error]
  pub enum Error<T> {
    LawAlreadyExists,
  }
  #[pallet::storage]
  pub(super) type Laws<T: Config> = StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, u32, BoundedVec<u8, ConstU32<65536>>, ValueQuery>;
  #[pallet::call]
  impl<T: Config> Pallet<T> {

    #[pallet::weight(0)]
    pub fn add_law(origin: OriginFor<T>, index1: u32, index2:u32, lawContent: BoundedVec<u8, ConstU32<65536>> ) -> DispatchResult {
    	let sender = ensure_signed(origin)?;

    	ensure!(!Laws::<T>::contains_key(&index1, &index2), Error::<T>::LawAlreadyExists);

    	Laws::<T>::insert(&index1, &index2, &lawContent);

    	Self::deposit_event(Event::LawAdded { who: sender, index1, index2 });

    	Ok(())
    }

	#[pallet::weight(0)]
    pub fn repeal_law(origin: OriginFor<T>, index1: u32, index2:u32) -> DispatchResult {
        	let sender = ensure_signed(origin)?;

        	Laws::<T>::remove(&index1, &index2);

        	Self::deposit_event(Event::LawRepealed { who: sender, index1, index2 });

        	Ok(())
        }

  }
}
