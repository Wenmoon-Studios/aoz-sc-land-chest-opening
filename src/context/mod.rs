multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const DROP_CHANCE_DENOMINATION: usize = 1_000_000_000;

pub struct StorageCache<'a, C>
where
    C: crate::storage::StorageModule,
{
    sc_ref: &'a C,
    pub chest_nonce: u64,
    pub remaining_guaranteed_drops: usize,
    pub remaining_chance_drops: usize,
    pub remaining_guaranteed_set: ManagedVec::<C::Api, DropItem<C::Api>>,
    pub remaining_chance_set: ManagedVec::<C::Api, DropItem<C::Api>>,
    pub randomness_source: RandomnessSource<C::Api>,
}

impl<'a, C> StorageCache<'a, C>
where
    C: crate::storage::StorageModule,
{
    pub fn new(sc_ref: &'a C,  chest_nonce: u64) -> Self {
        let mut remaining_guaranteed_set = ManagedVec::<C::Api, DropItem<C::Api>>::new();
        let mut remaining_chance_set = ManagedVec::<C::Api, DropItem<C::Api>>::new();
        let mut remaining_chance_drops = 0usize;
        let mut remaining_guaranteed_drops = 0usize;

        for guaranteed_drop_item in sc_ref.guaranteed_item_set(chest_nonce).iter() {
            let (drop, amount) = guaranteed_drop_item;
            remaining_guaranteed_drops += amount;
            remaining_guaranteed_set.push(DropItem {
                drop_content: drop,
                amount_left: amount,
            });
        }

        for chance_drop_item in sc_ref.chance_based_item_set(chest_nonce).iter() {
            let (drop, amount) = chance_drop_item;
            remaining_chance_drops += amount;
            remaining_chance_set.push(DropItem {
                drop_content: drop,
                amount_left: amount,
            });
        }

        StorageCache {
            sc_ref,
            chest_nonce,
            remaining_guaranteed_drops,
            remaining_chance_drops,
            remaining_guaranteed_set,
            remaining_chance_set,
            randomness_source: RandomnessSource::new()
        }
    }

    pub fn has_won_chance_drop(&mut self) -> bool {
        let random = self.randomness_source.next_usize_in_range(0, self.remaining_guaranteed_drops + 1);
        random < self.remaining_chance_drops
    }

    pub fn get_chance_drop(&mut self) -> EsdtTokenPayment<C::Api> {
        self.remaining_chance_drops -= 1;
        Self::get_drop_item_from_set(
            &mut self.randomness_source, 
            &mut self.remaining_chance_set
        )
    }

    pub fn get_guaranteed_drop_from_set(&mut self) -> EsdtTokenPayment<C::Api> {
        self.remaining_guaranteed_drops -= 1;
        Self::get_drop_item_from_set(
            &mut self.randomness_source, 
            &mut self.remaining_guaranteed_set
        )
    }

    fn get_drop_item_from_set(
        randomness_source: &mut RandomnessSource<C::Api>, 
        drop_set: &mut ManagedVec::<C::Api, DropItem<C::Api>>
    ) -> EsdtTokenPayment<C::Api> {
        let random_drop_idx = randomness_source.next_usize_in_range(0, drop_set.len() + 1);
        let mut set_drop_item = drop_set.get(random_drop_idx);
        let drop_content = set_drop_item.drop_content.clone();

        drop_set.remove(random_drop_idx);

        set_drop_item.amount_left -= 1;
        drop_set.push(set_drop_item);
        
        drop_content
    }

    pub fn get_guaranteed_drop(&mut self) -> EsdtTokenPayment<C::Api> {
        self.sc_ref.guaranteed_item(self.chest_nonce).get()
    }
}

impl<'a, C> Drop for StorageCache<'a, C>
where
    C: crate::storage::StorageModule,
{
    fn drop(&mut self) {
        // commit changes to storage for the mutable fields
        for new_chance_drop in self.remaining_chance_set.iter() {
            let key = new_chance_drop.drop_content;
            let value = new_chance_drop.amount_left;
            self.sc_ref
                .chance_based_item_set(self.chest_nonce)
                .entry(key)
                .and_modify(|amount| *amount = value);
        }

        for new_guaranteed_drop in self.remaining_guaranteed_set.iter() {
            let key = new_guaranteed_drop.drop_content;
            let value = new_guaranteed_drop.amount_left;
            self.sc_ref
                .guaranteed_item_set(self.chest_nonce)
                .entry(key)
                .and_modify(|amount| *amount = value);
        }
    }
}

#[derive(
    ManagedVecItem,
    TopEncode,
    TopDecode,
    NestedEncode,
    NestedDecode,
    TypeAbi,
    Clone,
    PartialEq,
    Debug,
)]
pub struct DropItem<M: ManagedTypeApi> {
    pub drop_content: EsdtTokenPayment<M>,
    pub amount_left: usize
}