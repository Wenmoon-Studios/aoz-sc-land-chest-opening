#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod owner;
pub mod storage;
pub mod types;
pub mod context;
use context::StorageCache;
use crate::types::*;

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
        let collection_id = self.chest_token_id().get();
        let mut rewards_vec = ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new();

        require!(payment.len() > 0usize, "no chests sent");

        let mut random_source = RandomnessSource::new();

        //get remaining qty for each prize pool
        //use this array to calc prize_pool_id instead of .get() from storage

        //random between 1 and sum(quantity(eligible_pool_ids))
        for chest in payment.iter() {
            
            require!(&chest.token_identifier == &collection_id, "wrong SFTs sent");
            let mut _storage_cache  = StorageCache::new(self, chest.token_nonce);

            require!(
                !self.eligible_pool_ids(chest.token_nonce).is_empty(),
                "eligible pool ids for chest nonce is empty"
            );

            let eligible_pool_ids = self.eligible_pool_ids(chest.token_nonce);
            let mut total_quantity = self.calc_total_quantity(&eligible_pool_ids);

            for _qty in 1..=chest.amount.to_u64().unwrap() {
                //randomly take one [x, y)
                let random_number = random_source.next_u64_in_range(1, total_quantity + 1u64);

                //get corresponding prize pool
                let prize_pool_id = self.get_prize_pool(random_number, &eligible_pool_ids);
                require!(prize_pool_id != 0, "not enough quantity left");

                //add reward to vec
                for prize in self.prize_pool(prize_pool_id).iter() {
                    rewards_vec.push(prize);
                }

                //decrease quantity
                self.pool_quantity(prize_pool_id).update(|val| *val -= 1u64);
                total_quantity -= 1u64;
            }
        }

        require!(!rewards_vec.is_empty(), "no rewards to receive");

        //send back the rewards
        self.send()
            .direct_multi(&self.blockchain().get_caller(), &rewards_vec);
    }

    #[only_owner]
    #[endpoint(setNoncesAndPools)]
    fn set_nonces_and_pools(&self, nonces_and_pools: MultiValueEncoded<NonceAndPool<Self::Api>>) {
        for nonce_and_pool in nonces_and_pools.into_iter() {
            for eligible_pool_id in nonce_and_pool.eligible_pool_ids.iter() {
                self.eligible_pool_ids(nonce_and_pool.chest_nonce)
                    .insert(eligible_pool_id);
            }
        }
    }

    #[only_owner]
    #[endpoint(setPoolsAndPrizes)]
    fn set_pools_and_prizes(&self, pool_and_prizes: MultiValueEncoded<PrizePoolInfo<Self::Api>>) {
        for pool_and_prize in pool_and_prizes.into_iter() {
            self.all_prize_pool_ids().insert(pool_and_prize.pool_id);
            self.pool_quantity(pool_and_prize.pool_id)
                .set(pool_and_prize.quantity);
            for prize in pool_and_prize.prize.iter() {
                self.prize_pool(pool_and_prize.pool_id).insert(prize);
            }
        }
    }

    fn calc_total_quantity(&self, eligible_pool_ids: &UnorderedSetMapper<u64>) -> u64 {
        let mut sum = 0u64;

        for prize_pool in eligible_pool_ids.iter() {
            sum += self.pool_quantity(prize_pool).get();
        }
        sum
    }

    fn get_prize_pool(
        &self,
        random_number: u64,
        eligible_pool_ids: &UnorderedSetMapper<u64>,
    ) -> u64 {
        let mut sum = 0u64;

        for prize_pool in eligible_pool_ids.iter() {
            sum += self.pool_quantity(prize_pool).get();
            if random_number <= sum {
                return prize_pool;
            }
        }
        return 0u64;
    }
}
