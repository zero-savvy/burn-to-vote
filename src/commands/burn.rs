use crate::{circuits::mpt_c::MptCircuit, utils::{account::{get_account_proof, get_account_rlp}, get_mpt_node_type, serialize_hex}};
use ethers::{
    core::types::TransactionRequest,
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    utils::{self, keccak256, rlp},
    

};
use crate::circuits::Circuit;
use log::info;


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
    // println!("Account RLP: {:?}", _account_rlp);
    let b = provider.get_block_number().await.unwrap();
    // println!("block number: {:?}", b);
    let bp = provider.get_block(b).await.unwrap();
    match bp {
        Some(x)=> {
            let root = x.state_root;
            println!("block number: {:?}", root.as_fixed_bytes())
        },
        None =>println!("block number does not exist"),
    
    }
    // println!("addres: {:?}", hex_encode(keccak256(addres_proof.clone().account_proof[0].clone())));
    println!("addres: {:?}", keccak256(addres_proof.clone().account_proof[0].clone()));
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

    let mut proofBytes : Vec<Vec<u8>>  = vec![];
    let mut prooflen : Vec<usize>  = vec![];
    let mut decoded_proof_bytes : Vec<Vec<Vec<u8>>>  = vec![];

    println!("addres_proof.account_proof {:?}", addres_proof.account_proof);

    // 532
    for item in &addres_proof.account_proof {
        let hex_array = serialize_hex(&hex::encode(item));
        println!("item {:?}", item);
        let mut node = item.to_vec();
        let len = node.len();
        prooflen.push(len);
        let rlp_decode:Vec<Vec<u8>> = rlp::decode_list(&node);
        decoded_proof_bytes.push(rlp_decode);
        // println!("decode: {:?}\n len{:?}", rlp_decofe, rlp_decofe.len());
        if len < 532 {
            let diff = 532 - len; 
            node.resize(len+diff, 0);
        }
        proofBytes.push(node);
    }
    print!("lensssss{:?}",prooflen );
    let node_types = get_mpt_node_type(decoded_proof_bytes);
    // println!("node_types : {:?}\n", proofBytes.clone());
    // let mut file = File::create("circuits/inputs/mpt.json").unwrap();
    // let data = format!(
    //     "{{\"account_proof\": {:?}}}",
    //     proofBytes
    // );
    // file.write_all(data.as_bytes());

    let circuit = MptCircuit::new(proofBytes.clone(), proofBytes.len().clone(), prooflen, node_types);
    info!("Burn address circuit: ");
    // println!("proofBytes.len() : {:?}\n", proofBytes.len());
    // println!("proofBytes[0].len() : {:?}\n", proofBytes[0].len());
    let inputs = circuit.format_inputs().unwrap();
    circuit.generate_input_file(inputs).unwrap();
    circuit.generate_witness().unwrap();
    circuit.setup_zkey().unwrap();
    circuit.generate_proof().unwrap();
    // circuit.setup_vkey().unwrap();
    // circuit.verify_proof().unwrap();
    let node0 = addres_proof.account_proof[0].clone();

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
    // println!("addresss: {:?}", keccak256(addres_proof.account_proof[3].clone().to_vec()));


    // let node = addres_proof.account_proof[3].clone();
    // println!("node: {:?}\n", node.to_vec());




    "burn token and generate burn proof".to_string()
    // to do : add generate proof
}
