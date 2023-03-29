mod setup;
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::setup::*;

#[test]
fn init_test() {
    let _ = ChestOpeningSetup::new(aoz_sc_land_chest_opening::contract_obj);
}

#[test]
fn open_1_chest() {
    let mut setup = ChestOpeningSetup::new(aoz_sc_land_chest_opening::contract_obj);
    setup.open_chests(1);
    // case pull is winner: 3 items
    // case pull is loser: 2 items
    setup.check_prize_drop_count(3);
}

#[test]
fn open_10_chests() {
    let mut setup = ChestOpeningSetup::new(aoz_sc_land_chest_opening::contract_obj);
    // case pull is winner: 3 items
    // case pull is loser: 2 items
    // 15 total items to be won
    // => all lucky = 30 items
    setup.open_chests(10);
    setup.check_prize_drop_count(30);
}

#[test]
fn open_50_chests() {
    let mut setup = ChestOpeningSetup::new(aoz_sc_land_chest_opening::contract_obj);
    // case pull is winner: 3 items
    // case pull is loser: 2 items
    // 15 total items to be won
    // => 15 lucky chests at most + 35 regular chests at worst
    // totals 15 * 3 + 35 * 2 = 45 + 70 = 115
    setup.open_chests(50);
    setup.check_prize_drop_count(115);
}

#[test]
fn open_100_chests() {
    let mut setup = ChestOpeningSetup::new(aoz_sc_land_chest_opening::contract_obj);
    setup.open_chests(100);
    setup.check_prize_drop_count(215);
}