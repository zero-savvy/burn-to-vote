use std::hash::pedersen_hash;


fn hash_left_right(left: Field, right: Field) -> Field {
    let hasher: Field = pedersen_hash([left, right]);
    hasher
}

fn dual_mux(input: [Field; 2], s: Field) -> [Field; 2] {
    assert(s * (1 - s) == 0);
    let out0 = (input[1] - input[0]) * s + input[0];
    let out1 = (input[0] - input[1]) * s + input[1];
    [out0, out1]
}


pub fn merkle_tree_checker(leaf: Field, path_elements: [Field], path_indices: [Field]) -> Field {
    let levels = path_elements.len();
    let mut current_hash = leaf;

    for i in 0..levels {
        let muxed = dual_mux([current_hash, path_elements[i]], path_indices[i]);
        current_hash = hash_left_right(muxed[0], muxed[1]);
    }

    current_hash
}

global H: u32 = 7;

fn main(lambda: Field, v: pub Field, a: Field, rlp: Field, 
    path_elem: [Field; H], path_index: [Field; H], reg_path_elem: [Field; H], reg_path_index: [Field; H], 
    u_id: Field, alpha_info: Field, beta: Field, eta: pub Field, v_id: pub Field, delta_reg: pub Field, sr: pub Field) {
    let tmp = pedersen_hash([u_id,v_id,v]);
    let expected_lambda = pedersen_hash([tmp,beta]);
    std::println(expected_lambda);
    assert_eq(expected_lambda, lambda);

    let expected_eta = pedersen_hash([alpha_info,v_id]);
    std::println(expected_eta);
    assert_eq(expected_eta, eta);

    let root = merkle_tree_checker(lambda, path_elem, path_index);
    assert_eq(root,sr);
    let root2 = merkle_tree_checker(eta, reg_path_elem, reg_path_index);
    assert_eq(root2,delta_reg);


}