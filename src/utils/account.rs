use alloy::primitives::keccak256;
use alloy_rlp::Encodable;
use ethers::{prelude::*, utils::rlp};
use ethers::types::Address;
use ethers::providers::{Http, Middleware, Provider,
};
use super::serialize_hex;
use crate::circuits::mpt_c::MptCircuit;
use log::info;
use super::get_mpt_node_type;


pub async fn get_account_proof(address: H160) -> EIP1186ProofResponse {
    let provider: Provider<Http> = Provider::<Http>::try_from("http://localhost:8545/")
        .unwrap()
        .clone();
    let block = provider.get_block_number().await.unwrap();
    provider
        .get_proof(address, vec![], Some(BlockId::from(block)))
        .await
        .unwrap()
}
pub fn get_account_rlp(proof: EIP1186ProofResponse) -> Vec<u8> {
    let mut rlp_stream = rlp::RlpStream::new();
    rlp_stream.begin_unbounded_list();
    rlp_stream.append(&proof.nonce);
    rlp_stream.append(&proof.balance);
    rlp_stream.append(&proof.storage_hash);
    rlp_stream.append(&proof.code_hash);
    rlp_stream.finalize_unbounded_list();
    rlp_stream.out().to_vec()
}

pub async fn prepare_mpt_data(burn_address: Address, provider: Provider<Http>) -> MptCircuit {
    info!("Preparing MPT circuit data ...");
    let addres_proof = get_account_proof(burn_address).await;
    let _account_rlp = get_account_rlp(addres_proof.clone());
    let rlp_hex = hex::encode(_account_rlp.clone());

    let mut serilized_rlp = serialize_hex(&rlp_hex);
    let serilized_rlp_len = serilized_rlp.len();
    if serilized_rlp_len < 164 {
        let diff = 164 - serilized_rlp_len;
        serilized_rlp.resize(serilized_rlp_len + diff, 0);
    }

    let block = provider.get_block_number().await.unwrap();
    let bp = provider.get_block(block).await.unwrap();
    let mut block_root: [u8; 32] = [0; 32];
    match bp {
        Some(x) => {
            let root = x.state_root;
            block_root = root.as_fixed_bytes().clone();
        }
        None => println!("block number does not exist"),
    }
    let state_root_hex = hex::encode(block_root);
    let serialized_state_root = serialize_hex(&state_root_hex);

    let mut proof: Vec<Vec<u8>> = vec![];
    let mut prooflen: Vec<usize> = vec![];

    let mut decoded_proof_bytes : Vec<Vec<Vec<u8>>>  = vec![];
    for item in &addres_proof.account_proof {
        let mut node = item.to_vec();
        let rlp_decode:Vec<Vec<u8>> = rlp::decode_list(&node);
        decoded_proof_bytes.push(rlp_decode.clone());

        let mut node_hex_array = serialize_hex(&hex::encode(item));
        log::info!("item{:?}", item.to_vec());
        log::info!("len:   {:?}", item.to_vec().len());
        log::info!("node_hex_array{:?}", node_hex_array);
        log::info!("len:  {:?}", node_hex_array.len());
        log::info!("decoded nodes: {:?}",rlp_decode.clone());
        log::info!("decoded len: {:?}",rlp_decode.len());
        let hex_hash = hex::encode(keccak256(item.clone().to_vec()));
        let ssss = serialize_hex(&hex_hash);
        log::info!("node hash: {:?}", ssss);
        log::info!("//////////////////////////////////////////////////////////////////////////");

        let len = node_hex_array.len();

        prooflen.push(len);
        if len < 1064 {
            let diff = 1064 - len;
            node_hex_array.resize(len + diff, 0);
        }
        proof.push(node_hex_array); 
    }

    let node_types = get_mpt_node_type(decoded_proof_bytes.clone());
    log::info!("prooflen{:?}",node_types.len());
    log::info!("prooflen{:?}",node_types);
    let address_hash =hex::encode(keccak256(burn_address));
    let sa = serialize_hex(&address_hash);
    log::info!("serialize address: {:?}",sa);
    log::info!("decoded nodes: {:?}",decoded_proof_bytes.len());
    let mut decoded_noded_length: Vec<Vec<usize>> = Vec::new();
    let mut node_lengths = Vec::new();
    for item in decoded_proof_bytes{
        let mut node_length = Vec::new();
        node_lengths.push(item.len());
        for i in item.clone() {
            node_length.push(i.len());
        }
        let nl: Vec<usize> = node_length.iter().map(|x| (x + 1)*2).collect();
        decoded_noded_length.push(nl);
        log::info!("decoded nodes: {:?}",item);
        log::info!("decoded nodes: {:?}",item.len());
    }
    log::info!("address bytes: {:?}",keccak256(burn_address).to_vec());
    log::info!("decoded_noded_length: {:?}",decoded_noded_length);
    log::info!("decoded_noded_length: {:?}",node_lengths);
    // log::info!("decoded nodes: {:?}",decoded_proof_bytes[1]);

    let aaaaaa = hex::encode([51, 93, 253, 141, 184, 39, 12, 245, 120, 220, 164, 70, 79, 239, 185, 39, 116, 83, 11, 202, 65, 208, 249, 226, 254, 161, 123, 220, 4, 202, 124, 214]);
    let h = serialize_hex(&aaaaaa);
    log::info!("serialize_hex: {:?}",h);


    let max_proof_len = 8;
    let real_proof_len = proof.len().clone();
    let empty_array = [0;1064].to_vec();
    for i in 0 .. max_proof_len - real_proof_len{
        proof.push(empty_array.clone());
        prooflen.push(0);
    }

    info!("MPT circuit data generated.");

    MptCircuit::new(
        addres_proof.nonce,
        addres_proof.balance,
        addres_proof.code_hash.to_fixed_bytes(),
        addres_proof.storage_hash.to_fixed_bytes(),
        serialized_state_root.clone(),
        serilized_rlp.clone(),
        serilized_rlp_len,
        proof.clone(),
        real_proof_len,
        prooflen,
    )
    
}


//  (len -1 )node hex len is 358
// [32, 32, 0, 0, 0, 0, 0, 0, 0, 0, 32, 32, 0, 32, 0, 0, 0]
// 12 item 0 
// [32, 0, 0, 0, 0, 32, 0, 32, 32, 32,
//  0, 0, 32, 0, 0, 0, 0]


// node_hex_array[15, 8, 13, 1, 10, 0, 6, 13, 10, 2, 0, 6, 14, 7, 15, 10, 2, 1, 4, 4, 11, 5, 5, 8, 2, 7, 2, 2, 0, 13, 8, 7, 11, 1, 15, 13, 3, 5, 12, 7, 10, 15, 5, 10, 13, 2, 5, 2, 5, 2, 1, 14, 11, 13, 7, 6, 8, 6, 8, 8, 3, 10, 11, 3, 0, 6, 6, 10, 3, 2, 8, 0, 8, 0, 8, 0, 8, 0, 10, 0, 6, 14, 1, 13, 15, 7, 14, 7, 0, 12, 2, 9, 8, 12, 2, 7, 4, 14, 10, 3, 1, 5, 12, 0, 12, 4, 12, 14, 14, 6, 6, 2, 5, 2, 13, 1, 3, 15, 10, 12, 4, 13, 6, 10, 4, 15, 3, 1, 11, 7, 12, 5, 14, 0, 6, 15, 1, 4, 6, 0, 2, 8, 0, 13, 8, 0, 10, 0, 14, 1, 1, 13, 11, 10, 14, 4, 1, 2, 0, 2, 0, 14, 5, 2, 1, 5, 7, 3, 11, 6, 11, 9, 3, 5, 4, 7, 4, 1, 14, 11, 15, 1, 14, 1, 7, 1, 14, 3, 2, 14, 8, 5, 8, 15, 7, 3, 4, 15, 3, 15, 13, 13, 1, 2, 14, 3, 13, 15, 3, 1, 11, 13, 10, 0, 2, 14, 11, 0, 9, 15, 15, 0, 3, 15, 5, 15, 2, 5, 2, 0, 6, 10, 6, 1, 0, 11, 6, 7, 0, 8, 10, 12, 8, 11, 5, 14, 15, 15, 8, 12, 8, 5, 7, 2, 10, 12, 7, 15, 14, 0, 3, 15, 1, 15, 1, 7, 15, 5, 14, 15, 6, 7, 14, 5, 6, 10, 2, 1, 10, 0, 3, 3, 5, 13, 15, 13, 8, 13, 11, 8, 2, 7, 0, 12, 15, 5, 7, 8, 13, 12, 10, 4, 4, 6, 4, 15, 14, 15, 11, 9, 2, 7, 7, 4, 5, 3, 0, 11, 12, 10, 4, 1, 13, 0, 15, 9, 14, 2, 15, 14, 10, 1, 7, 11, 13, 12, 0, 4, 12, 10, 7, 12, 13, 6,
//  8, 0, 8, 0, 10, 0, 13, 14, 14, 10, 15, 5, 7, 6, 3, 9, 6, 1, 3, 11, 0, 9, 1, 6, 6, 8, 12, 3, 4, 4, 1, 4, 12, 13, 5, 10, 6, 4, 10, 14, 4, 8, 14, 5, 15, 12, 3, 0, 3, 6, 14, 12, 15, 12, 5, 8, 1, 8, 6, 5, 9, 1, 15, 14, 6, 3, 0, 5, 6, 12, 8, 0, 8, 0, 8, 0, 8, 0]

// [32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 0],