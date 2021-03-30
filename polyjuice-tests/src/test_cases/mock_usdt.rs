//! Test ERC20 contract
//!   See ./evm-contracts/ERC20.bin

use crate::helper::{
    account_id_to_eth_address, deploy, new_account_script, new_block_info, setup,
    PolyjuiceArgsBuilder, CKB_SUDT_ACCOUNT_ID,
};
use gw_common::state::State;
use gw_generator::traits::StateExt;
use gw_jsonrpc_types::parameter::RunResult;
use gw_types::{bytes::Bytes, packed::RawL2Transaction, prelude::*};

const JSON_DATA: &str = include_str!("./evm-contracts/error-permission.calldata.json");

#[test]
fn test_mock_sudt() {
    let (store, mut tree, generator, creator_account_id) = setup();
    let from_script1 = gw_generator::sudt::build_l2_sudt_script([1u8; 32].into());
    let from_id1 = tree.create_account_from_script(from_script1).unwrap();
    let from_script2 = gw_generator::sudt::build_l2_sudt_script([2u8; 32].into());
    let from_id2 = tree.create_account_from_script(from_script2).unwrap();
    let from_script3 = gw_generator::sudt::build_l2_sudt_script([3u8; 32].into());
    let from_id3 = tree.create_account_from_script(from_script3).unwrap();
    tree.mint_sudt(CKB_SUDT_ACCOUNT_ID, from_id1, 20000000)
        .unwrap();
    tree.mint_sudt(CKB_SUDT_ACCOUNT_ID, from_id3, 80000)
        .unwrap();
    println!("creator_account_id: {}", creator_account_id);
    println!("from_id1: {}", from_id1);
    println!("from_id2: {}", from_id2);
    println!("from_id3: {}", from_id3);

    let data: serde_json::Value = serde_json::from_str(JSON_DATA).unwrap();
    // Deploy ERC20
    let _run_result = deploy(
        &generator,
        &store,
        &mut tree,
        creator_account_id,
        from_id1,
        &data[0]["calldata"].as_str().unwrap()[2..],
        182000,
        0,
        1,
    );
    let erc20_account_script = new_account_script(&mut tree, from_id1, false);
    let erc20_account_id = tree
        .get_account_id_by_script_hash(&erc20_account_script.hash().into())
        .unwrap()
        .unwrap();
    println!("=============================================");

    // Deploy ERROR permission
    let _run_result = deploy(
        &generator,
        &store,
        &mut tree,
        creator_account_id,
        from_id1,
        &data[1]["calldata"].as_str().unwrap()[2..],
        182000,
        0,
        1,
    );
    let demo_account_script = new_account_script(&mut tree, from_id1, false);
    let demo_account_id = tree
        .get_account_id_by_script_hash(&demo_account_script.hash().into())
        .unwrap()
        .unwrap();
    println!("=============================================");

    {
        // user approve demo, MockUSDT, MaxUint256;
        let block_info = new_block_info(0, 2, 0);
        // _approve(from_id1, demo_account_id)
        let input =
            hex::decode(format!(
                "095ea7b30000000000000000000000000{}00000000000000000000000000000000000000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", demo_account_id,
            ).as_str())
            .unwrap();
        let args = PolyjuiceArgsBuilder::default()
            .gas_limit(21000)
            .gas_price(1)
            .value(0)
            .input(&input)
            .build();
        let raw_tx = RawL2Transaction::new_builder()
            .from_id(from_id1.pack())
            .to_id(erc20_account_id.pack())
            .args(Bytes::from(args).pack())
            .build();
        let run_result = generator
            .execute(&store.begin_transaction(), &tree, &block_info, &raw_tx)
            .expect("construct");
        tree.apply_run_result(&run_result).expect("update state");
        // println!("result {:?}", run_result);
    }
    println!("=============================================");

    {
        // user approve demo, MockUSDT, MaxUint256;
        let block_info = new_block_info(0, 2, 0);
        // transferFrom(from_id1, demo_account_id)
        let input =
            hex::decode(format!(
                "05a37e9a0000000000000000000000000{}000000000000000000000000000000000000000000000000000000000000000{}0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003e7", erc20_account_id, demo_account_id,
            ).as_str())
            .unwrap();
        let args = PolyjuiceArgsBuilder::default()
            .gas_limit(81000)
            .gas_price(1)
            .value(0)
            .input(&input)
            .build();
        let raw_tx = RawL2Transaction::new_builder()
            .from_id(from_id1.pack())
            .to_id(demo_account_id.pack())
            .args(Bytes::from(args).pack())
            .build();
        let run_result = generator
            .execute(&store.begin_transaction(), &tree, &block_info, &raw_tx)
            .expect("construct");
        tree.apply_run_result(&run_result).expect("update state");
        // println!("result {:?}", run_result);
    }
}
