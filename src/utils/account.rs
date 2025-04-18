use ethers::{prelude::*, utils::rlp};
use ethers::types::Address;
use ethers::providers::{Http, Middleware, Provider,
};
use super::serialize_hex;
use crate::circuits::mpt_c::MptCircuit;
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

    for item in &addres_proof.account_proof {
        let mut node_hex_array = serialize_hex(&hex::encode(item));
        let len = node_hex_array.len();

        prooflen.push(len);
        if len < 1064 {
            let diff = 1064 - len;
            node_hex_array.resize(len + diff, 0);
        }
        proof.push(node_hex_array);
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
        proof.len().clone(),
        prooflen,
    )
    
}
