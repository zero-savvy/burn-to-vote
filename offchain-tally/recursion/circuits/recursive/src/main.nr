use bb_proof_verification::{UltraHonkZKProof, UltraHonkVerificationKey, verify_ultrahonkzk_proof};

mod merkle_tools;

global N: u32 = 4;

fn main(
    verification_key: UltraHonkVerificationKey,
    proof: [UltraHonkZKProof; N],
    public_inputs: pub [[Field; 5]; N],
    key_hash: [Field; N],
    eta: [Field; N],
    path_elem: [[Field; 2]; N],
    path_index: [[Field; 2]; N],
    delta_usr: pub Field,
    v_id: pub Field,
    sr: pub Field,
    delta_reg: pub Field,
) {
    for i in 0..N {
        verify_ultrahonkzk_proof(verification_key, proof[i], public_inputs[i], key_hash[i]);
        assert_eq(public_inputs[i][2],v_id);
        assert_eq(public_inputs[i][3],delta_reg);
        assert_eq(public_inputs[i][4],sr);
        let root = merkle_tools::merkle_tree_checker(eta[i], path_elem[i], path_index[i]);
        assert_eq(root,delta_usr);
    }

}
