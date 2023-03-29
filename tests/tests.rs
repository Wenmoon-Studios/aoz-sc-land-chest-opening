mod setup;
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::setup::*;

#[test]
fn init_test() {
    let _ = ChestOpeningSetup::new(aoz_sc_land_chest_opening::contract_obj);
}