multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub const ONE_HOUR: u64 = 60 * 60;

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
pub struct NonceAndPool<M: ManagedTypeApi> {
    pub chest_nonce: u64,
    pub eligible_pool_ids: ManagedVec<M, u64>,
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
pub struct PrizePoolInfo<M: ManagedTypeApi> {
    pub pool_id: u64,
    pub prize: ManagedVec<M, EsdtTokenPayment<M>>,
    pub quantity: u64
}
