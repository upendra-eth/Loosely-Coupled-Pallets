#![cfg_attr(not(feature = "std"), no_std)]
pub use frame_system::pallet::*;
// use codec::{Encode, Decode};

#[frame_support::pallet]
pub mod pallet {
	use core::u32;

use frame_support::{pallet_prelude::{*, OptionQuery}, StorageMap, Blake2_128Concat};
	use frame_system::pallet_prelude::*;
	use frame_support::inherent::Vec;
	

// use sp_io::transaction_index::index;
	
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	// #[pallet::without_storage_info]
	pub struct Pallet<T>(_);
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Provider: DataProvider<Key = u32, Data = Vec<u8>>;
		// fn store_esg(&self) -> Vec<u8>;
	}

	#[pallet::storage]
	#[pallet::getter(fn Data)]
	// pub type CompanyNameStorage<T:Config> = StorageMap <_, u32, Vec<u8>, OptionQuery>;
	pub(super) type CompanyNameStorage<T: Config> = StorageMap <_, 
		Blake2_128Concat,
	 	key: u32, 
	 	data: Vec<u8>, 
	 	OptionQuery>;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ESGStored(u32, Vec<u8>, T::AccountId),

	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		#[pallet::weight(0)]		
		pub fn esgscore(origin: OriginFor<T>, Key: u32, Data: Vec<u8>) -> DispatchResult {
		
			let signer = ensure_signed(origin)?;
	
			<CompanyNameStorage<T>>::insert( Key, Data);
			// <CompanyNameStorage::get();
		
			Self::deposit_event(Event::ESGStored(Key, Data, signer.clone()));
			
			
			Ok(())

		}

		// #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		// pub fn name_error(origin: OriginFor<T>) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	match <CompanyNameStorage<T>>::get() {
		// 		// Return an error if the value has not been set.
		// 		None => return Err(Error::<T>::NoneValue.into()),
		// 		Some(_old) => return Err(Error::<T>::StorageOverflow.into()),
					
					
		// 	}
		// }

	}



	pub trait DataProvider {
		// type Key = u32;
		// type Data = Vec<u8>;
		type Key;
		type Data;
	 
		fn query(key: u32) -> Option<Vec<u8>>;
	}

	impl<T: Config> DataProvider for Pallet<T> {
		// type Key = T::Index;
		// type Data = T::Data;
	 
		fn query(key: u32) -> Option<Vec<u8>> {
			CompanyNameStorage::<T>::get(&key)
		}
	}


	
}


	
                                                                                                        
	

