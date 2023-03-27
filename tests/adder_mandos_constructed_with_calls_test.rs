use aoz_sc_land_chest_opening::*;
use multiversx_sc::storage::mappers::SingleValue;
use multiversx_sc_scenario::{num_bigint::BigUint, scenario_model::*, *};

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/lib");

    blockchain.register_contract("file:output/aoz-sc-land-chest-opening.wasm", aoz_sc_land_chest_opening::ContractBuilder);
    blockchain
}

#[test]
fn lib_scenario_constructed_raw() {
    let _ = DebugApi::dummy();
    let mut world = world();
    let ic = world.interpreter_context();
    let owner_address = "address:owner";
    let mut lib_contract = ContractInfo::<lib::Proxy<DebugApi>>::new("sc:lib");

    world
        .set_state_step(
            SetStateStep::new()
                .put_account(owner_address, Account::new().nonce(1))
                .new_address(owner_address, 1, "sc:lib"),
        )
        .sc_deploy_step(
            ScDeployStep::new()
                .from(owner_address)
                .contract_code("file:output/aoz-sc-land-chest-opening.wasm", &ic)
                .call(lib_contract.init(5u32))
                .gas_limit("5,000,000")
                .expect(TxExpect::ok().no_result()),
        )
        .sc_query_step(
            ScQueryStep::new()
                .to(&lib_contract)
                .call_expect(lib_contract.sum(), SingleValue::from(BigUint::from(5u32))),
        )
        .sc_call_step(
            ScCallStep::new()
                .from(owner_address)
                .to(&lib_contract)
                .call(lib_contract.add(3u32))
                .expect(TxExpect::ok().no_result()),
        )
        .check_state_step(
            CheckStateStep::new()
                .put_account(owner_address, CheckAccount::new())
                .put_account(
                    &lib_contract,
                    CheckAccount::new().check_storage("str:sum", "8"),
                ),
        )
        .write_scenario_trace("trace1.scen.json");
}
