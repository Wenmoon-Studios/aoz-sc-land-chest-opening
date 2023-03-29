use aoz_sc_land_chest_opening::*;
use aoz_sc_land_chest_opening::owner::OwnerModule;
use multiversx_sc_scenario::{whitebox::{BlockchainStateWrapper,ContractObjWrapper,TxTokenTransfer}, rust_biguint, managed_token_id};
multiversx_sc::imports!();
multiversx_sc::derive_imports!();
use multiversx_sc::types::{Address, EsdtLocalRole, ManagedAddress, MultiValueEncoded};
use multiversx_sc_scenario::{
    DebugApi,
};

pub const WASM_PATH: &str = "../output/aoz-sc-land-chest-opening.wasm";
pub const CHEST_TOKEN_ID: &[u8] = b"LANDCHEST-123456";

pub const GUARANTEED_DROP_TOKEN_ID: &[u8] = b"LANDCHEST-123456";
pub const GUARANTEED_SET_TOKEN_ID_1: &[u8] = b"GUARANTEED1-123456";
pub const GUARANTEED_SET_TOKEN_ID_2: &[u8] = b"GUARANTEED2-123456";
pub const GUARANTEED_SET_TOKEN_ID_3: &[u8] = b"GUARANTEED3-123456";
pub const LEGENDARY_DROP_1: &[u8] = b"LEGENDARY1-123456";
pub const LEGENDARY_DROP_2: &[u8] = b"LEGENDARY2-123456";
pub const LEGENDARY_DROP_3: &[u8] = b"LEGENDARY3-123456";
pub const LEGENDARY_DROP_4: &[u8] = b"LEGENDARY4-123456";
pub const LEGENDARY_DROP_5: &[u8] = b"LEGENDARY5-123456";
pub struct ChestOpeningSetup<ChestOpeningObjBuilder>
where
    ChestOpeningObjBuilder: 'static + Copy + Fn() -> aoz_sc_land_chest_opening::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub user_address: Address,
    pub sc_wrapper: ContractObjWrapper<aoz_sc_land_chest_opening::ContractObj<DebugApi>, ChestOpeningObjBuilder>
}

impl<ChestOpeningObjBuilder> ChestOpeningSetup<ChestOpeningObjBuilder>
where
    ChestOpeningObjBuilder: 'static + Copy + Fn() -> aoz_sc_land_chest_opening::ContractObj<DebugApi>,
{
    pub fn new(builder: ChestOpeningObjBuilder) -> Self {
        let _ = DebugApi::dummy();

        let rust_zero = rust_biguint!(0u64);
        let mut b_mock = BlockchainStateWrapper::new();
        let owner_addr = b_mock.create_user_account(&rust_zero);
        let user_addr = b_mock.create_user_account(&rust_biguint!(1));
        let sc_wrapper = 
            b_mock.create_sc_account(
                &rust_zero,
                Some(&owner_addr),
                builder,
                WASM_PATH
            );
        b_mock
            .execute_tx(&owner_addr, &sc_wrapper, &rust_zero, |sc| {
                sc.init(
                    OptionalValue::Some(false),
                    OptionalValue::Some(managed_token_id!(CHEST_TOKEN_ID))
                );

                sc.set_guaranteed_item(1, managed_token_id!(GUARANTEED_DROP_TOKEN_ID), 100);
                sc.add_guaranteed_set_item(1, managed_token_id!(GUARANTEED_SET_TOKEN_ID_1), 1, 40);
                sc.add_guaranteed_set_item(1, managed_token_id!(GUARANTEED_SET_TOKEN_ID_2), 2, 30);
                sc.add_guaranteed_set_item(1, managed_token_id!(GUARANTEED_SET_TOKEN_ID_3), 3, 30);

                sc.add_chance_set_item(1, managed_token_id!(LEGENDARY_DROP_1), 1, 1);
                sc.add_chance_set_item(1, managed_token_id!(LEGENDARY_DROP_2), 2, 2);
                sc.add_chance_set_item(1, managed_token_id!(LEGENDARY_DROP_3), 3, 3);
                sc.add_chance_set_item(1, managed_token_id!(LEGENDARY_DROP_4), 4, 4);
                sc.add_chance_set_item(1, managed_token_id!(LEGENDARY_DROP_5), 5, 5);
            })
            .assert_ok();
        ChestOpeningSetup { b_mock: b_mock, owner_address: owner_addr, user_address: user_addr, sc_wrapper: sc_wrapper }
    }
}