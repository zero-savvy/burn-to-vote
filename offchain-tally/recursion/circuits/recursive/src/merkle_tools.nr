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
