multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait OwnerModule: crate::storage::StorageModule {
    #[only_owner]
    #[endpoint(enableSc)]
    fn enable_sc(&self) {
        self.enabled().set(true);
    }

    #[only_owner]
    #[endpoint(disableSc)]
    fn disable_sc(&self) {
        self.enabled().clear();
    }
}
