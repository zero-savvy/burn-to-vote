use crate::{circuits::mpt_c::MptCircuit, utils::{account::{get_account_proof, get_account_rlp}, get_mpt_node_type, serialize_hex}};
use ethers::{
    core::types::TransactionRequest,
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    utils::{self, keccak256, rlp},
    

};
use serde::ser::SerializeTuple;
use crate::circuits::Circuit;
use log::info;

use hex;
use structopt::StructOpt;
use std::convert::TryFrom;


#[derive(Debug, StructOpt)]
pub struct Burn {
    private_key: String,
    burn_address: Address,
    amount: U256,
}

pub async fn burn(burn_data: Burn) -> String {
    let provider = Provider::<Http>::try_from("http://localhost:8545/")
        .unwrap()
        .clone();

    let chain_id = provider.get_chainid().await.unwrap();
    let wallet: LocalWallet = burn_data
        .private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id.as_u64());

    let to_address = burn_data.burn_address;
    // create circuit data
    let addres_proof = get_account_proof(burn_data.burn_address).await;
    // println!("////////////////");
    // println!("account proof {:?}", serde_json::to_string(&addres_proof.account_proof));
    // println!("account proof len{:?}", &addres_proof.account_proof.len());
    let _account_rlp = get_account_rlp(addres_proof.clone());
    let rlp_hex = hex::encode(_account_rlp.clone());
    println!("Account RLP bytesssss: {:?}", _account_rlp.clone());
    println!("Account RLP bytessssslen : {:?}", _account_rlp.clone().len());

    // println!("Account RLP hex : {:?}", rlp_hex);
    let mut serilized_rlp = serialize_hex(&rlp_hex);
    let serilized_rlp_len = serilized_rlp.len();
    // println!("serilized_rlp RLP: {:?}", serilized_rlp_len);
    // println!("serilized_rlp RLP: {:?}", serilized_rlp);
    if serilized_rlp_len < 164 {
        let diff = 164 - serilized_rlp_len; 
        serilized_rlp.resize(serilized_rlp_len+diff, 0);
    }
    // println!("serilized_rlp RLP: {:?}", serilized_rlp);
    // println!("last var: {:?}", serilized_rlp[serilized_rlp_len -1 ]);
    // println!("last var: {:?}", serilized_rlp[serilized_rlp_len -2 ]);




    let b = provider.get_block_number().await.unwrap();
    // println!("block number: {:?}", b);
    let bp = provider.get_block(b).await.unwrap();
    let mut block_root: [u8; 32] = [0; 32];
    match bp {
        Some(x)=> {
            let root = x.state_root;
            block_root = root.as_fixed_bytes().clone();
            // println!("block number: {:?}", root.as_fixed_bytes())
        },
        None =>println!("block number does not exist"),
    
    }
    // println!("addres: {:?}", hex::encode(addres_proof.clone().account_proof[0].clone()));
    // println!("proof: {:?}", addres_proof.clone());
    let r = hex::encode(block_root);
    let serialized = serialize_hex(&r);
    // println!("serialized root: {:?}", serialized);

    // serialized root: [6, 1, 15, 6, 14, 12, 13, 7, 13, 7, 8, 15, 4, 5, 7, 14, 9, 9, 0, 1, 11, 14, 10, 8, 10, 14, 5, 1, 7, 11, 12, 9, 15, 4, 2, 10, 0, 5, 5, 3, 13, 15, 10, 6, 15, 12, 7, 7, 8, 12, 3, 0, 5, 7, 14, 1, 3, 14, 2, 0, 9, 8, 14, 9]

    // println!("addres: {:?}", keccak256(addres_proof.clone().account_proof[0].clone()));
    // println!("addres: {:?}", keccak256(addres_proof.clone().account_proof[0].clone()));

    // let trie = TrieDBBuilder::<KeccakHasher>::new(&addres_proof.account_proof, &Some(bp).account_proof).unwrap();


    let client = SignerMiddleware::new(provider.clone(), wallet);

    let pre_tx_balance = provider
        .clone()
        .get_balance(burn_data.burn_address, None)
        .await
        .unwrap();

    let tx = TransactionRequest::new()
        .to(to_address)
        .value(U256::from(utils::parse_ether(burn_data.amount).unwrap()));

    let pending_tx = client.send_transaction(tx, None).await.unwrap();
    let receipt = pending_tx
        .await
        .unwrap()
        .ok_or_else(|| eyre::format_err!("tx dropped from mempool"))
        .unwrap();
    let tx = client
        .get_transaction(receipt.transaction_hash)
        .await
        .unwrap();

    let post_tx_balance = provider
        .clone()
        .get_balance(burn_data.burn_address, None)
        .await
        .unwrap();

    // println!("Sent tx: {}\n", serde_json::to_string(&tx).unwrap());
    // println!("Tx receipt: {}", serde_json::to_string(&receipt).unwrap());
    assert_eq!(
        pre_tx_balance + U256::from(utils::parse_ether(burn_data.amount).unwrap()),
        post_tx_balance
    );

    let mut proof : Vec<Vec<u8>>  = vec![];
    let mut prooflen : Vec<usize>  = vec![];
    let mut decoded_proof_bytes : Vec<Vec<Vec<u8>>>  = vec![];

    // println!("addres_proof.account_proof {:?}", addres_proof.account_proof);

    // 532
    for item in &addres_proof.account_proof {
        let mut node_hex_array = serialize_hex(&hex::encode(item));
        // println!("item {:?}", item);
        // let mut node = item.to_vec();
        let len = node_hex_array.len();
        // print!("hexlensssss:{:?} ",len );

        prooflen.push(len);
        // let rlp_decode:Vec<Vec<u8>> = rlp::decode_list(&node);
        // decoded_proof_bytes.push(rlp_decode);
        // println!("decode: {:?}\n len{:?}", rlp_decofe, rlp_decofe.len());
        if len < 1064 {
            let diff = 1064 - len; 
            node_hex_array.resize(len+diff, 0);
        }
        proof.push(node_hex_array);
    }
    // print!("lensssss{:?}",prooflen );
    // print!("//////////////////////");

    // print!("proof{:?}",proof[0]);
    // print!("//////////////////////");

    // // proof[15, 9, 0, 1, 13, 1, 10, 0, 10, 6, 1, 1, 11, 2, 5, 5, 5, 0, 4, 3, 11, 15, 12, 2, 2, 4, 0, 9, 14, 6, 12, 13, 5, 4, 1, 13, 4, 0, 3, 11, 5, 2, 5, 8, 13, 10, 0, 10, 12, 11,
    // // proof[15, 9, 0, 1, 13, 1, 10, 0, 10, 6, 1, 1, 11, 2, 5, 5, 5, 
    // print!("lensssss0{:?}",proof[0].len());
    // print!("lensssss1{:?}",proof[0][0]);
    // print!("lensssss2{:?}",proof[0][proof[0].len()-1]);
    // print!("lensssss3{:?}",proof[3].len());
    // print!("prooflen{:?}",prooflen.len());
// lensssss01064lensssss115lensssss20lensssss31064prooflen4% 

    // let node_types = get_mpt_node_type(decoded_proof_bytes);
    // print!("prooflen{:?}",node_types.len());

    // println!("node_types : {:?}\n", proofBytes.clone());
    // let mut file = File::create("circuits/inputs/mpt.json").unwrap();
    // let data = format!(
    //     "{{\"account_proof\": {:?}}}",
    //     proofBytes
    // );
    // file.write_all(data.as_bytes());
    // let n0 = serialize_hex(&hex::encode(addres_proof.account_proof[0].clone()));
    // println!("n0 {:?}", n0);

    let lastnode = addres_proof.account_proof[3].clone();
    let serialized_node = serialize_hex(&hex::encode(lastnode));
    // println!("serialized_node: {:?}\n", serialized_node);

    let circuit = MptCircuit::new(addres_proof.nonce, addres_proof.balance ,addres_proof.code_hash.to_fixed_bytes(), addres_proof.storage_hash.to_fixed_bytes(), serialized.clone(), serilized_rlp.clone(), serilized_rlp_len, proof.clone(), proof.len().clone(), prooflen);
    info!("Burn address circuit: ");
    // println!("proofBytes.len() : {:?}\n", proofBytes.len());
    // println!("proofBytes[0].len() : {:?}\n", proofBytes[0].len());
    let inputs = circuit.format_inputs().unwrap();
    circuit.generate_input_file(inputs).unwrap();
    // circuit.generate_witness().unwrap();
    // circuit.setup_zkey().unwrap();
    // circuit.generate_proof().unwrap();
    // circuit.setup_vkey().unwrap();
    // circuit.verify_proof().unwrap();
    // let node0 = addres_proof.account_proof[0].clone();


    // // what to check 
    let n0 = serialize_hex(&hex::encode(addres_proof.account_proof[0].clone()));
    // println!("n0 {:?}", n0);


    // let n1 = serialize_hex(&hex::encode(addres_proof.account_proof[1].clone()));
    // println!("n1 hash: {:?}", keccak256(n1));

    // let n00 = serialize_hex(&hex::encode(addres_proof.account_proof[0].clone()));
    // println!("n0o {:?}", addres_proof.account_proof[0].clone().to_vec());


    // let n11 = serialize_hex(addres_proof.account_proof[1].clone().to_vec());
    // println!("n1 hash: {:?}", keccak256(addres_proof.account_proof[1].clone().to_vec()));


    // println!("node 0 : {:?}\n", node0.to_vec().len());

    // let rlp_decofe:Vec<Vec<u8>> = rlp::decode_list(&node0.to_vec());
    // println!("decode: {:?}\n", rlp_decofe);
    // println!("addresss: {:?}", to_address);
    // let address_bytes = hex::decode("a8c41add5116d220f406c3ffeb2cad8ff3064475").expect("Invalid hex string");
    // println!("addresss: {:?}", keccak256(addres_proof.account_proof[1].clone().to_vec()));

    // let node1 = addres_proof.account_proof[1].clone();
    // println!("node 1: {:?}\n", node1.to_vec().len());

    // let rlp_decofe:Vec<Vec<u8>> = rlp::decode_list(&node.to_vec());
    // println!("decode: {:?}\n", rlp_decofe);
    // // println!("addresss: {:?}", to_address);
    // // let address_bytes = hex::decode("a8c41add5116d220f406c3ffeb2cad8ff3064475").expect("Invalid hex string");
    // println!("addresss: {:?}", keccak256(addres_proof.account_proof[2].clone().to_vec()));

    // let node = addres_proof.account_proof[2].clone();
    // println!("node: {:?}\n", node.to_vec());

    // let rlp_decofe:Vec<Vec<u8>> = rlp::decode_list(&node.to_vec());
    // println!("decode: {:?}\n", rlp_decofe);
    // // println!("addresss: {:?}", to_address);
    // // let address_bytes = hex::decode("a8c41add5116d220f406c3ffeb2cad8ff3064475").expect("Invalid hex string");
    // println!("node length: {:?}", addres_proof.account_proof[3].clone().len());
    // println!("node0 length: {:?}", addres_proof.account_proof[0].clone().len());
    // println!("node1 length: {:?}", addres_proof.account_proof[1].clone().len());
    // println!("node3 length: {:?}", addres_proof.account_proof[2].clone().len());
    // // println!("node 3: {:?}", keccak256(addres_proof.account_proof[3].clone().to_vec()));
    // println!("node 3: {:?}", proof[2]);


    // let hex_hash = hex::encode(keccak256(addres_proof.account_proof[3].clone().to_vec()));
    // let ssss = serialize_hex(&hex_hash);
    // println!("node 322: {:?}", ssss);

    // println!("node 2: {:?}", keccak256(addres_proof.account_proof[2].clone().to_vec()));
    // println!("node 1: {:?}", keccak256(addres_proof.account_proof[1].clone().to_vec()));

    println!("rlppppp: {:?}", serilized_rlp.clone());
    let bbb = hexToBytes(serilized_rlp.clone());
    println!("rlppppp bytes: {:?}", bbb);
    println!("rlppppp bytes: {:?}", bbb.len());
    println!("rlppppp bytes: {:?}", bbb.len());


    println!("node 1: {:?}", serilized_rlp_len);
    println!("nonce: {:?}", addres_proof.nonce);
    let mut bytes = [0u8; 32];
    addres_proof.balance.to_little_endian(&mut bytes);
    println!("balance: {:?}", addres_proof.balance);
    println!("balance: {:?}", serialize_hex(&hex::encode(&bytes)));
    println!("balance: {:?}", serialize_hex(&addres_proof.balance.to_string()).len());

    println!("code hash: {:?}", serialize_hex(&hex::encode(&addres_proof.code_hash.to_fixed_bytes())));
    println!("code hash: {:?}", addres_proof.code_hash.to_fixed_bytes());

    println!("storage hash: {:?}", serialize_hex(&hex::encode(&addres_proof.storage_hash.to_fixed_bytes())));
    println!("storage_hash: {:?}", addres_proof.storage_hash.to_fixed_bytes());


    println!("storagae hash: {:?}", serialize_hex(&addres_proof.storage_hash.to_string()).len());








    "burn token and generate burn proof".to_string()
    // to do : add generate proof
}


fn hexToBytes(hex: Vec<u8>)->Vec<u8>{
    let mut v:Vec<u8>=vec![];
    for i in (0 .. hex.len()).step_by(2) {
        v.push(hex[i]*16+hex[i+1]);
    }
    v

}

// balance: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 8, 4, 9, 3, 15, 11, 10, 6, 4, 14, 15, 0, 0, 0, 0, 0]
// balance: [0, 0, 0, 0, 5, 4, 15, 6, 5, 9, 11, 2, 7, 4, 9, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]


// nonce: 0
// balance: [0, 0, 0, 0, 0, 4, 12, 15, 12, 5, 4, 2, 15, 13, 3, 8, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// balance: 20
// code hash: [12, 5, 13, 2, 4, 6, 0, 1, 8, 6, 15, 7, 2, 3, 3, 12, 9, 2, 7, 14, 7, 13, 11, 2, 13, 12, 12, 7, 0, 3, 12, 0, 14, 5, 0, 0, 11, 6, 5, 3, 12, 10, 8, 2, 2, 7, 3, 11, 7, 11, 15, 10, 13, 8, 0, 4, 5, 13, 8, 5, 10, 4, 7, 0]
// code hash: [197, 210, 70, 1, 134, 247, 35, 60, 146, 126, 125, 178, 220, 199, 3, 192, 229, 0, 182, 83, 202, 130, 39, 59, 123, 250, 216, 4, 93, 133, 164, 112]
// storage hash: [5, 6, 14, 8, 1, 15, 1, 7, 1, 11, 12, 12, 5, 5, 10, 6, 15, 15, 8, 3, 4, 5, 14, 6, 9, 2, 12, 0, 15, 8, 6, 14, 5, 11, 4, 8, 14, 0, 1, 11, 9, 9, 6, 12, 10, 13, 12, 0, 0, 1, 6, 2, 2, 15, 11, 5, 14, 3, 6, 3, 11, 4, 2, 1]
// storage_hash: [86, 232, 31, 23, 27, 204, 85, 166, 255, 131, 69, 230, 146, 192, 248, 110, 91, 72, 224, 27, 153, 108, 173, 192, 1, 98, 47, 181, 227, 99, 180, 33]
// storagae hash: 9

// nonce: 0
// balance: 48000000000000000000
// code hash: [12, 5, 13, 2, 4, 6, 0, 1, 8, 6, 15, 7, 2, 3, 3, 12, 9, 2, 7, 14, 7, 13, 11, 2, 13, 12, 12, 7, 0, 3, 12, 0, 14, 5, 0, 0, 11, 6, 5, 3, 12, 10, 8, 2, 2, 7, 3, 11, 7, 11, 15, 10, 13, 8, 0, 4, 5, 13, 8, 5, 10, 4, 7, 0]
// code hash: [197, 210, 70, 1, 134, 247, 35, 60, 146, 126, 125, 178, 220, 199, 3, 192, 229, 0, 182, 83, 202, 130, 39, 59, 123, 250, 216, 4, 93, 133, 164, 112]
// storage hash: [5, 6, 14, 8, 1, 15, 1, 7, 1, 11, 12, 12, 5, 5, 10, 6, 15, 15, 8, 3, 4, 5, 14, 6, 9, 2, 12, 0, 15, 8, 6, 14, 5, 11, 4, 8, 14, 0, 1, 11, 9, 9, 6, 12, 10, 13, 12, 0, 0, 1, 6, 2, 2, 15, 11, 5, 14, 3, 6, 3, 11, 4, 2, 1]
// storage_hash: [86, 232, 31, 23, 27, 204, 85, 166, 255, 131, 69, 230, 146, 192, 248, 110, 91, 72, 224, 27, 153, 108, 173, 192, 1, 98, 47, 181, 227, 99, 180, 33]


// new
// rlppppp bytes: 82
// rlppppp bytes: 82
// node 1: 158
// nonce: 0
// balance: 49000000000000000000
// balance: [0, 0, 0, 0, 2, 4, 0, 10, 6, 3, 15, 8, 0, 2, 10, 8, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
// balance: 20
// code hash: [12, 5, 13, 2, 4, 6, 0, 1, 8, 6, 15, 7, 2, 3, 3, 12, 9, 2, 7, 14, 7, 13, 11, 2, 13, 12, 12, 7, 0, 3, 12, 0, 14, 5, 0, 0, 11, 6, 5, 3, 12, 10, 8, 2, 2, 7, 3, 11, 7, 11, 15, 10, 13, 8, 0, 4, 5, 13, 8, 5, 10, 4, 7, 0]
// code hash: [197, 210, 70, 1, 134, 247, 35, 60, 146, 126, 125, 178, 220, 199, 3, 192, 229, 0, 182, 83, 202, 130, 39, 59, 123, 250, 216, 4, 93, 133, 164, 112]
// storage hash: [5, 6, 14, 8, 1, 15, 1, 7, 1, 11, 12, 12, 5, 5, 10, 6, 15, 15, 8, 3, 4, 5, 14, 6, 9, 2, 12, 0, 15, 8, 6, 14, 5, 11, 4, 8, 14, 0, 1, 11, 9, 9, 6, 12, 10, 13, 12, 0, 0, 1, 6, 2, 2, 15, 11, 5, 14, 3, 6, 3, 11, 4, 2, 1]
// storage_hash: [86, 232, 31, 23, 27, 204, 85, 166, 255, 131, 69, 230, 146, 192, 248, 110, 91, 72, 224, 27, 153, 108, 173, 192, 1, 98, 47, 181, 227, 99, 180, 33]
// storagae hash: 9