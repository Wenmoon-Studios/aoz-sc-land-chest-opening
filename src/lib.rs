#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod owner;
pub mod storage;
pub mod types;
pub mod context;
use context::StorageCache;

// TODO Phase 2:
// find a way to store remaining quantity of each prize pool outside of the main for
// use this array to identify what prize_pool_id to use based on the random number
// and pass those arrays to get_prize_pool

#[multiversx_sc::contract]
pub trait AozScLandChestOpening: storage::StorageModule + owner::OwnerModule {
    #[init]
    fn init(&self, chest_token_id_opt: OptionalValue<TokenIdentifier>) {
        match chest_token_id_opt {
            OptionalValue::Some(val) => self.chest_token_id().set(val),
            OptionalValue::None => {
                require!(
                    !self.chest_token_id().is_empty(),
                    "chest token id was not supplied"
                );
            }
        }
    }

    #[payable("*")]
    #[endpoint(openChests)]
    fn open_chests(&self) {
        require!(!self.enabled().is_empty(), "maintenance");

        let payment = self.call_value().all_esdt_transfers();
        require!(payment.len() > 0usize, "no chests sent");

        let chest_token_id = self.chest_token_id().get();
        let mut rewards_vec = ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new();

        for chest in payment.iter() {
            require!(&chest.token_identifier == &chest_token_id, "wrong SFTs sent");
            let mut storage_cache  = StorageCache::new(self, chest.token_nonce);
            let guaranteed_drop = storage_cache.get_guaranteed_drop_with_quantity(chest.amount.clone());
            rewards_vec.push(guaranteed_drop);

            for _ in 0..chest.amount.to_u64().unwrap() as usize {
                if storage_cache.has_won_chance_drop() {
                    let chance_drop = storage_cache.get_chance_drop();
                    rewards_vec.push(chance_drop);
                }
                let guaranteed_drop_from_set = storage_cache.get_guaranteed_drop_from_set();
                rewards_vec.push(guaranteed_drop_from_set);
            }
        }

        require!(!rewards_vec.is_empty(), "no rewards to receive");

        //send back the rewards
        self.send()
            .direct_multi(&self.blockchain().get_caller(), &rewards_vec);
    }
}
