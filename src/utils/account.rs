use ethers::{prelude::*, types::Bytes, utils::rlp};
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
pub fn get_account_rlp(proof: EIP1186ProofResponse) -> Bytes {
    let mut rlp_stream = rlp::RlpStream::new();
    rlp_stream.begin_unbounded_list();
    rlp_stream.append(&proof.nonce);
    rlp_stream.append(&proof.balance);
    rlp_stream.append(&proof.storage_hash);
    rlp_stream.append(&proof.code_hash);
    rlp_stream.finalize_unbounded_list();
    Bytes::from(rlp_stream.out().to_vec())
}
