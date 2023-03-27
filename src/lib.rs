#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod owner;
pub mod private;
pub mod storage;
pub mod types;

use crate::types::*;

#[multiversx_sc::contract]
pub trait AozScLandChestOpening:
    storage::StorageModule + private::PrivateModule + owner::OwnerModule
{
    #[init]
    fn init(
        &self,
        chest_token_id_opt: OptionalValue<TokenIdentifier>,
        accepted_chest_nonces_opt: OptionalValue<MultiValueEncoded<u64>>,
        nonces_and_pools_opt: OptionalValue<MultiValueEncoded<NonceAndPool<Self::Api>>>,
        pool_and_prizes_opt: OptionalValue<MultiValueEncoded<PrizePoolInfo<Self::Api>>>,
    ) {
        match accepted_chest_nonces_opt {
            OptionalValue::Some(val) => {
                for nonce in val.into_iter() {
                    self.chest_nonces().insert(nonce);
                }
            }
            OptionalValue::None => {}
        }

        match chest_token_id_opt {
            OptionalValue::Some(val) => self.chest_token_id().set(val),
            OptionalValue::None => {}
        }

        match nonces_and_pools_opt {
            OptionalValue::Some(val) => {
                for nonce_and_pool in val.into_iter() {
                    self.chest_nonces().insert(nonce_and_pool.chest_nonce);
                    for eligible_pool_id in nonce_and_pool.eligible_pool_ids.iter() {
                        self.eligible_pool_ids(nonce_and_pool.chest_nonce)
                            .insert(eligible_pool_id);
                    }
                }
            }
            OptionalValue::None => {}
        }

        match pool_and_prizes_opt {
            OptionalValue::Some(val) => {
                for pool_and_prize in val.into_iter() {
                    self.all_prize_pool_ids().insert(pool_and_prize.pool_id);
                    self.pool_quantity(pool_and_prize.pool_id)
                        .set(pool_and_prize.quantity);
                    for prize in pool_and_prize.prize.iter() {
                        self.prize_pool(pool_and_prize.pool_id).insert(prize);
                    }
                }
            }
            OptionalValue::None => {}
        }
    }

    #[payable("*")]
    #[endpoint(openChests)]
    fn open_chests(&self) {
        require!(!self.enabled().is_empty(), "maintenance");

        let payment = self.call_value().all_esdt_transfers();

        self.validate_chest_opening(&payment);

        let mut rewards_vec = ManagedVec::<Self::Api, EsdtTokenPayment<Self::Api>>::new();

        //random between 1 and sum(quantity(eligible_pool_ids))
        for chest in payment.iter() {
            let eligible_pool_ids = self.eligible_pool_ids(chest.token_nonce);
            let total_quantity = self.calc_total_quantity(&eligible_pool_ids);

            let mut random_source = RandomnessSource::new();

            //randomly take one [x, y)
            let random_number = random_source.next_u64_in_range(1, total_quantity + 1u64);

            //get corresponding prize pool
            let prize_pool_id = self.get_prize_pool(random_number, eligible_pool_ids);
            require!(prize_pool_id != 0, "not enough quantity left");

            //add reward to vec
            for prize in self.prize_pool(prize_pool_id).iter() {
                rewards_vec.push(prize);
            }

            //decrease quantity
            self.pool_quantity(prize_pool_id).update(|val| *val -= 1u64);
        }

        require!(!rewards_vec.is_empty(), "no rewards to receive");

        //send back the rewards
        self.send()
            .direct_multi(&self.blockchain().get_caller(), &rewards_vec);
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
        eligible_pool_ids: UnorderedSetMapper<u64>,
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
