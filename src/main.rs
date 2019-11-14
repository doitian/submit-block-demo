use ckb_jsonrpc_types::{Block, BlockTemplate};
use ckb_types::{packed, prelude::*};

fn main() {
    let block_template_text = r#"
        {"bytes_limit":"0x91c08","cellbase":{"cycles":null,"data":{"cell_deps":[],"header_deps":[],"inputs":[{"previous_output":{"index":"0xffffffff","tx_hash":"0x0000000000000000000000000000000000000000000000000000000000000000"},"since":"0x51b5"}],"outputs":[{"capacity":"0x3dfc9dcd68","lock":{"args":"0xb2e61ff569acf041b3c2c17724e2379c581eeac3","code_hash":"0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8","hash_type":"type"},"type":null}],"outputs_data":["0x"],"version":"0x0","witnesses":["0x590000000c00000055000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8011400000048b12c67274f56be27ccf4920402e16cd401c71c00000000"]},"hash":"0x47ad3afa5e8cb070929b92c9bbec5a76076bea8ce40494bef9a28233faf05691"},"compact_target":"0x1d722992","current_time":"0x16e67721478","cycles_limit":"0x2540be400","dao":"0x6180438021b5630054f3c3099883260012d1aa0131ef05000036d94c26860b00","epoch":"0x2eb012a00001f","number":"0x51b5","parent_hash":"0x097048451c099f846f2c4d19b79360cd9fe54b415496b454e008748bed493e8b","proposals":[],"raw":"0x000000009229721d8d0072676e010000b5510000000000001f00002a01eb0200097048451c099f846f2c4d19b79360cd9fe54b415496b454e008748bed493e8b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006180438021b5630054f3c3099883260012d1aa0131ef05000036d94c26860b00","transactions":[],"uncles":[],"uncles_count_limit":"0x2","version":"0x0","work_id":"0x1"}
    "#;

    let block_template: BlockTemplate = serde_json::from_str(block_template_text).unwrap();

    let block: packed::Block = block_template.into();
    let raw_header = block.header().raw();

    println!("raw_header = {:#x}", raw_header);
    // => raw_header = 0x000000009229721d781472676e010000b5510000000000001f00002a01eb0200097048451c099f846f2c4d19b79360cd9fe54b415496b454e008748bed493e8b78fdfacab36063f7863901aa6de85631254fe3741c387e5b0330c610014f1da4000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006180438021b5630054f3c3099883260012d1aa0131ef05000036d94c26860b00
    // Yours         = 0x000000009229721d8d0072676e010000b5510000000000001f00002a01eb0200097048451c099f846f2c4d19b79360cd9fe54b415496b454e008748bed493e8b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006180438021b5630054f3c3099883260012d1aa0131ef05000036d94c26860b00

    let nonce = {
        let nonce_buf = "000000010000000000000002a9df0189";
        let mut nonce_bytes = [0u8; 16];
        faster_hex::hex_decode(nonce_buf.as_bytes(), &mut nonce_bytes).unwrap();
        u128::from_le_bytes(nonce_bytes)
    };

    println!("nonce = {:#x}", nonce);
    // => nonce = 0x8901dfa9020000000000000001000000

    let pow_header = block.header().as_builder().nonce(nonce.pack()).build();
    let pow_block = block.as_builder().header(pow_header).build();
    let pow_block_as_json: Block = pow_block.into();
    let pow_block_text = serde_json::to_string(&pow_block_as_json).unwrap();
    println!("args[1] = {}", pow_block_text);
    // args[1] = {"header":{"version":"0x0","compact_target":"0x1d722992","parent_hash":"0x097048451c099f846f2c4d19b79360cd9fe54b415496b454e008748bed493e8b","timestamp":"0x16e67721478","number":"0x51b5","epoch":"0x2eb012a00001f","transactions_root":"0x78fdfacab36063f7863901aa6de85631254fe3741c387e5b0330c610014f1da4","proposals_hash":"0x0000000000000000000000000000000000000000000000000000000000000000","uncles_hash":"0x0000000000000000000000000000000000000000000000000000000000000000","dao":"0x6180438021b5630054f3c3099883260012d1aa0131ef05000036d94c26860b00","nonce":"0x8901dfa9020000000000000001000000"},"uncles":[],"transactions":[{"version":"0x0","cell_deps":[],"header_deps":[],"inputs":[{"previous_output":{"tx_hash":"0x0000000000000000000000000000000000000000000000000000000000000000","index":"0xffffffff"},"since":"0x51b5"}],"outputs":[{"capacity":"0x3dfc9dcd68","lock":{"args":"0xb2e61ff569acf041b3c2c17724e2379c581eeac3","code_hash":"0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8","hash_type":"type"},"type":null}],"witnesses":["0x590000000c00000055000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8011400000048b12c67274f56be27ccf4920402e16cd401c71c00000000"],"outputs_data":["0x"]}],"proposals":[]}

    let real_block: Block = serde_json::from_str(r#"{"header":{"compact_target":"0x1d722992","dao":"0x6180438021b5630054f3c3099883260012d1aa0131ef05000036d94c26860b00","epoch":"0x2eb012a00001f","nonce":"0x10000000000000002a9df0189","number":"0x51b5","parent_hash":"0x097048451c099f846f2c4d19b79360cd9fe54b415496b454e008748bed493e8b","proposals_hash":"0x0000000000000000000000000000000000000000000000000000000000000000","timestamp":"0x16e6772008d","transactions_root":"0x0000000000000000000000000000000000000000000000000000000000000000","uncles_hash":"0x0000000000000000000000000000000000000000000000000000000000000000","version":"0x0"},"proposals":[],"transactions":[{"cell_deps":[],"header_deps":[],"inputs":[{"previous_output":{"index":"0xffffffff","tx_hash":"0x0000000000000000000000000000000000000000000000000000000000000000"},"since":"0x51b5"}],"outputs":[{"capacity":"0x3dfc9dcd68","lock":{"args":"0xb2e61ff569acf041b3c2c17724e2379c581eeac3","code_hash":"0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8","hash_type":"type"},"type":null}],"outputs_data":["0x"],"version":"0x0","witnesses":["0x590000000c00000055000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8011400000048b12c67274f56be27ccf4920402e16cd401c71c00000000"]}],"uncles":[]}"#).unwrap();
    let real_block_text = serde_json::to_string(&real_block).unwrap();
    println!("yours   = {}", real_block_text);
    // yours   = {"header":{"version":"0x0","compact_target":"0x1d722992","parent_hash":"0x097048451c099f846f2c4d19b79360cd9fe54b415496b454e008748bed493e8b","timestamp":"0x16e6772008d","number":"0x51b5","epoch":"0x2eb012a00001f","transactions_root":"0x0000000000000000000000000000000000000000000000000000000000000000","proposals_hash":"0x0000000000000000000000000000000000000000000000000000000000000000","uncles_hash":"0x0000000000000000000000000000000000000000000000000000000000000000","dao":"0x6180438021b5630054f3c3099883260012d1aa0131ef05000036d94c26860b00","nonce":"0x10000000000000002a9df0189"},"uncles":[],"transactions":[{"version":"0x0","cell_deps":[],"header_deps":[],"inputs":[{"previous_output":{"tx_hash":"0x0000000000000000000000000000000000000000000000000000000000000000","index":"0xffffffff"},"since":"0x51b5"}],"outputs":[{"capacity":"0x3dfc9dcd68","lock":{"args":"0xb2e61ff569acf041b3c2c17724e2379c581eeac3","code_hash":"0x9bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8","hash_type":"type"},"type":null}],"witnesses":["0x590000000c00000055000000490000001000000030000000310000009bd7e06f3ecf4be0f2fcd2188b23f1b9fcc88e5d4b65a8637b17723bbda3cce8011400000048b12c67274f56be27ccf4920402e16cd401c71c00000000"],"outputs_data":["0x"]}],"proposals":[]}
}
