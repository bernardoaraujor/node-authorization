use super::*;

pub mod v1 {
    use super::*;
    use crate::log;
    use codec::Decode;
    use frame_support::{
        dispatch::GetStorageVersion,
        pallet_prelude::Weight,
        traits::{Get, OnRuntimeUpgrade},
    };
    use sp_core::{
        crypto::{AccountId32, ByteArray},
        OpaquePeerId,
    };

    pub struct MigrateToV1<T>(sp_std::marker::PhantomData<T>);
    impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
        fn on_runtime_upgrade() -> Weight {
            let current = Pallet::<T>::current_storage_version();
            let onchain = Pallet::<T>::on_chain_storage_version();

            log!(
                info,
                "Running migration with current storage version {:?} / onchain {:?}",
                current,
                onchain
            );

            if current == 1 && onchain == 0 {
                let alice_peer_id = OpaquePeerId(vec![
                    0, 36, 8, 1, 18, 32, 28, 229, 240, 14, 246, 232, 147, 116, 175, 182, 37, 241,
                    174, 76, 21, 70, 211, 18, 52, 232, 126, 60, 63, 81, 166, 43, 145, 221, 107,
                    250, 87, 223,
                ]);

                let bob_peer_id = OpaquePeerId(vec![
                    0, 36, 8, 1, 18, 32, 218, 205, 231, 113, 77, 133, 81, 246, 116, 184, 187, 75,
                    84, 35, 147, 131, 199, 106, 43, 40, 111, 164, 54, 233, 59, 43, 126, 178, 38,
                    191, 77, 231,
                ]);

                let alice_account_id32: AccountId32 = [
                    212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130,
                    44, 133, 88, 133, 76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
                ]
                .into();

                let bob_account_id32: AccountId32 = [
                    142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135,
                    97, 54, 147, 201, 18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
                ]
                .into();

                let alice_account_id =
                    T::AccountId::decode(&mut alice_account_id32.as_slice()).unwrap();

                let bob_account_id =
                    T::AccountId::decode(&mut bob_account_id32.as_slice()).unwrap();

                let nodes = vec![
                    (alice_peer_id, alice_account_id),
                    (bob_peer_id, bob_account_id),
                ];

                let peer_ids = nodes
                    .iter()
                    .map(|item| item.0.clone())
                    .collect::<BTreeSet<PeerId>>();
                WellKnownNodes::<T>::put(&peer_ids);

                for (node, who) in nodes.iter() {
                    Owners::<T>::insert(node, who);
                }

                log!(
                    info,
                    "Upgraded well-known nodes, storage to version {:?}",
                    current
                );
                T::DbWeight::get().reads_writes(2, 2)
            } else {
                log!(
                    info,
                    "Migration did not execute. This probably should be removed"
                );
                T::DbWeight::get().reads(1)
            }
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade() -> Result<(), &'static str> {
            // new version must be set.
            assert_eq!(Pallet::<T>::on_chain_storage_version(), 1);
            Ok(())
        }
    }
}
