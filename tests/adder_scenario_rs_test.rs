use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("contracts/examples/lib");

    blockchain.register_contract("file:output/aoz-sc-land-chest-opening.wasm", aoz_sc_land_chest_opening::ContractBuilder);
    blockchain
}

#[test]
fn lib_rs() {
    multiversx_sc_scenario::run_rs("scenarios/lib.scen.json", world());
}
