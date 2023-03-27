multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait PrivateModule: crate::storage::StorageModule {
    fn validate_chest_opening(&self, payment: &ManagedVec<EsdtTokenPayment<Self::Api>>) {
        let collection_token_id = self.chest_token_id().get();

        for chest in payment.iter() {
            require!(
                chest.token_identifier == collection_token_id,
                "wrong SFTs sent"
            );
        }
    }
}
