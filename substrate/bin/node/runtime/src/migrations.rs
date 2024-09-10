use super::*;
use frame_support::{pallet_prelude::*, traits::OnRuntimeUpgrade};

#[cfg(feature = "try-runtime")]
use sp_std::vec::Vec;

#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;

type DbWeight = <Runtime as frame_system::Config>::DbWeight;

pub mod add_contracts_registry_pallet {
	use super::*;

	pub struct Migration<T>(sp_std::marker::PhantomData<T>);

	impl OnRuntimeUpgrade for Migration<Runtime> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
			Ok(().encode())
		}

		fn on_runtime_upgrade() -> Weight {
			let mut weight = DbWeight::get().reads(1);

			if StorageVersion::get::<ContractsRegistry>() == 0 {
                StorageVersion::new(1).put::<ContractsRegistry>();
                weight = weight.saturating_add(DbWeight::get().reads_writes(1, 1));
            }

            weight
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
			Ok(())
		}
	}
}

pub mod add_sora_bridge {
	use super::*;

	pub struct Migration<T>(sp_std::marker::PhantomData<T>);

	impl OnRuntimeUpgrade for Migration<Runtime> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
			Ok(().encode())
		}

		fn on_runtime_upgrade() -> Weight {
			let mut weight = DbWeight::get().reads(1);

			if StorageVersion::get::<SubstrateBridgeInboundChannel>() == 0 {
                StorageVersion::new(1).put::<SubstrateBridgeInboundChannel>();
                weight = weight.saturating_add(DbWeight::get().reads_writes(1, 1));
            }

			if StorageVersion::get::<SubstrateBridgeOutboundChannel>() == 0 {
                StorageVersion::new(1).put::<SubstrateBridgeOutboundChannel>();
                weight = weight.saturating_add(DbWeight::get().reads_writes(1, 1));
            }

			if StorageVersion::get::<SubstrateDispatch>() == 0 {
                StorageVersion::new(1).put::<SubstrateDispatch>();
                weight = weight.saturating_add(DbWeight::get().reads_writes(1, 1));
            }

            weight
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
			Ok(())
		}
	}
}

pub mod add_senate_account_pallet {
	use super::*;

	pub struct Migration<T>(sp_std::marker::PhantomData<T>);

	impl OnRuntimeUpgrade for Migration<Runtime> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
			Ok(().encode())
		}

		fn on_runtime_upgrade() -> Weight {
			let mut weight = DbWeight::get().reads(1);

			if StorageVersion::get::<SenateAccount>() == 0 {
                StorageVersion::new(1).put::<SenateAccount>();
                weight = weight.saturating_add(DbWeight::get().reads_writes(1, 1));
            }

            weight
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_state: Vec<u8>) -> Result<(), TryRuntimeError> {
			Ok(())
		}
	}
}

pub mod add_senate_membership_pallet {
	use super::*;

	pub struct Migration<T>(sp_std::marker::PhantomData<T>);

	impl OnRuntimeUpgrade for Migration<Runtime> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
			Ok(().encode())
		}

		fn on_runtime_upgrade() -> Weight {
			let mut weight = DbWeight::get().reads(1);

			if StorageVersion::get::<SenateMembership>() == 0 {
				StorageVersion::new(4).put::<SenateMembership>();
				weight = weight.saturating_add(DbWeight::get().reads_writes(1, 1));
			}

			weight
		}
		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_: Vec<u8>) -> Result<(), TryRuntimeError> {
			Ok(())
		}
	}
}

pub mod copy_senate_members_from_collective_to_membership {
	use super::*;
	use frame_support::storage_alias;

	pub struct Migration<T>(sp_std::marker::PhantomData<T>);

	#[storage_alias(verbatim)]
	type Prime = StorageValue<SenateMembership, AccountId, OptionQuery>;
	#[storage_alias(verbatim)]
	type Members = StorageValue<SenateMembership, BoundedVec<AccountId, ConstU32<100>>, ValueQuery>;

	impl OnRuntimeUpgrade for Migration<Runtime> {
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, TryRuntimeError> {
			Ok(().encode())
		}

		fn on_runtime_upgrade() -> Weight {
			let mut weight = DbWeight::get().reads(2);

			// Get current Senate members and prime from collective pallet
			let senate_members = crate::Senate::members();
			let senate_prime = crate::Senate::prime();

			// Copy Senate members to membership pallet
			if !senate_members.is_empty() {
				let membership_members: BoundedVec<AccountId, ConstU32<100>> = senate_members
					.try_into()
					.expect("Error fitting Senate members into BoundedVec");
				log::warn!("Copying {} Senate Members", membership_members.len());
				Members::put(membership_members);
				weight = weight.saturating_add(DbWeight::get().writes(1));
			}

			// Copy Senate prime member to membership pallet
			if let Some(prime) = senate_prime {
				log::warn!("Copying Senate Prime member");
				Prime::put(prime);
				weight = weight.saturating_add(DbWeight::get().writes(1));
			}

			weight
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_: Vec<u8>) -> Result<(), TryRuntimeError> {
			Ok(())
		}
	}
}

