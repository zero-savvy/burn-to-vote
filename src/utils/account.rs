use super::get_mpt_node_type;
use super::serialize_hex;
use crate::circuits::mpt_c::MptCircuit;
use alloy::primitives::keccak256;
use alloy_rlp::Encodable;
use ethers::providers::{Http, Middleware, Provider};
use ethers::types::Address;
use ethers::{prelude::*, utils::rlp};
use log::info;

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

    let mut decoded_proof_bytes: Vec<Vec<Vec<u8>>> = vec![];
    for item in &addres_proof.account_proof {
        let mut node = item.to_vec();
        let rlp_decode: Vec<Vec<u8>> = rlp::decode_list(&node);
        decoded_proof_bytes.push(rlp_decode.clone());

        let mut node_hex_array = serialize_hex(&hex::encode(item));

        let len = node_hex_array.len();

        prooflen.push(len);
        if len < 1064 {
            let diff = 1064 - len;
            node_hex_array.resize(len + diff, 0);
        }
        proof.push(node_hex_array);
    }

    let mut node_types = get_mpt_node_type(decoded_proof_bytes.clone());
    let address_hash = hex::encode(keccak256(burn_address));
    let sa = serialize_hex(&address_hash);
    let mut node_items_len: Vec<Vec<usize>> = Vec::new();
    let mut node_lengths = Vec::new();
    for item in decoded_proof_bytes.clone() {
        let mut node_length = Vec::new();
        node_lengths.push(item.len());
        for i in item.clone() {
            node_length.push(i.len());
        }
        let nl: Vec<usize> = node_length.iter().map(|x| (x + 1) * 2).collect();
        node_items_len.push(nl);

    }

    let aaaaaa = hex::encode([
        51, 93, 253, 141, 184, 39, 12, 245, 120, 220, 164, 70, 79, 239, 185, 39, 116, 83, 11, 202,
        65, 208, 249, 226, 254, 161, 123, 220, 4, 202, 124, 214,
    ]);
    let h: Vec<u8> = serialize_hex(&aaaaaa);

    let max_proof_len = 8;
    let real_proof_len = proof.len().clone();
    let empty_array = [0; 1064].to_vec();
    for i in 0..max_proof_len - real_proof_len {
        proof.push(empty_array.clone());
        prooflen.push(0);
    }

    for item in &mut node_items_len {
        if item.len() < 17 {
            let diff = 17 - item.len();
            item.resize(item.len() + diff, 0);
        }
    }


    // get extension nibbles
    let mut extension_nodes_shared_nibbles = [0; 8];
    for (i, item) in node_types.iter().enumerate() {
        if *item == 0 {
            let nibs = &decoded_proof_bytes[i][0];
            // bytes
            let nibs_len = nibs.len();
            // -1 is to remove the prefix of leaf and extension nodes
            let nibs_len_without_prefix = (nibs_len - 1) * 2;
            extension_nodes_shared_nibbles[i] = nibs_len_without_prefix;
        }
    }

    let leaf_nibbles = extension_nodes_shared_nibbles[real_proof_len - 1];

    let ba = hex::encode(burn_address.clone());
    let s_ba: Vec<u8> = serialize_hex(&ba);

    info!("MPT circuit data generated.");

    MptCircuit::new(
        s_ba,
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
        leaf_nibbles,
    )
}
