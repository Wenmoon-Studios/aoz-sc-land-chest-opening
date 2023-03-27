multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    //SC
    #[view(getEnabled)]
    #[storage_mapper("enabled")]
    fn enabled(&self) -> SingleValueMapper<bool>;

    //COLLECTION
    #[view(getChestTokenId)]
    #[storage_mapper("chestTokenId")]
    fn chest_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    //PRIZES
    #[view(getAllPrizePoolIds)]
    #[storage_mapper("allPrizePoolIds")]
    fn all_prize_pool_ids(&self) -> UnorderedSetMapper<u64>;

    #[view(getElibiglePoolIds)]
    #[storage_mapper("eligiblePoolIds")]
    fn eligible_pool_ids(&self, chest_nonce: u64) -> UnorderedSetMapper<u64>;

    #[view(getPrizePool)]
    #[storage_mapper("prizePool")]
    fn prize_pool(&self, pool_id: u64) -> UnorderedSetMapper<EsdtTokenPayment<Self::Api>>;

    #[view(getPoolQuantity)]
    #[storage_mapper("poolQuantity")]
    fn pool_quantity(&self, pool_id: u64) -> SingleValueMapper<u64>;
}
