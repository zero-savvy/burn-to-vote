import { deflattenFields, UltraHonkBackend, Barretenberg } from "@aztec/bb.js";
import innerCircuit from "../circuits/inner/target/inner.json" with { type: "json" };
import recursiveCircuit from "../circuits/recursive/target/recursive.json" with { type: "json" };
import { CompiledCircuit, Noir } from "@noir-lang/noir_js";
import TOML from "@iarna/toml";
import fs from "fs";

const FR_SIZE = 32;
type Fr = Uint8Array;

// -------------------- Helpers --------------------
function frToBigInt(fr: Uint8Array): bigint {
  if (fr.length !== FR_SIZE) {
    throw new Error(`Invalid Fr length: expected ${FR_SIZE}, got ${fr.length}`);
  }

  let x = 0n;
  for (let i = 0; i < FR_SIZE; i++) {
    x = (x << 8n) | BigInt(fr[i]);
  }
  return x;
}

function bigintToFr(x: bigint): Fr {
  const buf = new Uint8Array(FR_SIZE);
  let v = x;
  for (let i = FR_SIZE - 1; i >= 0; i--) {
    buf[i] = Number(v & 0xffn);
    v >>= 8n;
  }
  return buf;
}

export function frToHex32(fr: Uint8Array): string {
  return (
    "0x" +
    Array.from(fr)
      .map(b => b.toString(16).padStart(2, "0"))
      .join("")
  );
}

function randomFr(): Fr {
  // Generate random 256-bit integer
  const r = BigInt(Math.round(Math.random() * 1000000));
  return bigintToFr(r);
}

function randomBitFr(): Fr {
  return bigintToFr(BigInt(Math.round(Math.random()))); // 0n or 1n
}
// ---------------- Merkle Tree Class ----------------
class MerkleTree {
  bb: Barretenberg;
  depth: number;
  leaves: Fr[];
  nodes: Fr[][]; // nodes[level][index]

  constructor(bb: Barretenberg, leaves: Fr[]) {
    this.bb = bb;
    this.depth = Math.ceil(Math.log2(leaves.length));
    this.leaves = leaves;
    this.nodes = [];
  }

  // Build tree bottom-up
  async build() {
    let currentLevel = [...this.leaves];
    this.nodes.push(currentLevel);

    while (currentLevel.length > 1) {
      const nextLevel: Fr[] = [];
      for (let i = 0; i < currentLevel.length; i += 2) {
        const left = currentLevel[i];
        const right = currentLevel[i + 1];

        const { hash } = await this.bb.pedersenHash({ inputs: [left, right], hashIndex: 0 });
        nextLevel.push(hash);
      }
      currentLevel = nextLevel;
      this.nodes.push(currentLevel);
    }

    return this.root();
  }

  root(): Fr {
    return this.nodes[this.nodes.length - 1][0];
  }

  // Returns path elements + indices for a leaf at position idx
  getPath(idx: number): { pathElements: Fr[]; pathIndices: Fr[] } {
    const pathElements: Fr[] = [];
    const pathIndices: Fr[] = [];

    let index = idx;
    for (let level = 0; level < this.depth; level++) {
      const siblingIndex = index % 2 === 0 ? index + 1 : index - 1;
      const levelNodes = this.nodes[level];
      pathElements.push(levelNodes[siblingIndex]);
      pathIndices.push(bigintToFr(BigInt(index % 2)));
      index = Math.floor(index / 2);
    }

    return { pathElements, pathIndices };
  }
}

// ---------------- Generate Inputs ----------------
async function generateNoirMerkleInputs(depth = 3) {
  const bb = await Barretenberg.new({ threads: 2 });

  const leafCount = 1 << depth; // 2^5 = 32 leaves per tree

  // sr Merkle tree (leaves = lambda)
  const vID = randomFr();
  const uIDs: Fr[] = Array.from({ length: leafCount }, randomFr);
  const votes: Fr[] = Array.from({ length: leafCount }, randomBitFr);
  const betas: Fr[] = Array.from({ length: leafCount }, randomFr);
  const alpha_infos: Fr[] = Array.from({ length: leafCount }, randomFr);
  const lambdaLeaves: Fr[] = [];
  const uIdLeaves: Fr[] = [];
  for (let i = 0; i < leafCount; i++) {
    const eta = await bb.pedersenHash({ inputs: [alpha_infos[i], vID], hashIndex: 0 });
    const val = await bb.pedersenHash({ inputs: [uIDs[i], vID, votes[i]], hashIndex: 0 });
    const com = await bb.pedersenHash({ inputs: [val.hash, betas[i]], hashIndex: 0 });
    lambdaLeaves.push(com.hash);
    uIdLeaves.push(eta.hash);
  }
  const srTree = new MerkleTree(bb, lambdaLeaves);
  const sr = await srTree.build();

  // delta_reg Merkle tree (leaves = u_id)
  const deltaRegTree = new MerkleTree(bb, uIdLeaves);
  const delta_reg = await deltaRegTree.build();

  // Pick example leaf indices to call merkle_tree_checker
  const exampleLeafIdx = 0;
  const srLeaf = lambdaLeaves[exampleLeafIdx];
  const deltaLeaf = uIdLeaves[exampleLeafIdx];

  const srPath = srTree.getPath(exampleLeafIdx);
  const deltaPath = deltaRegTree.getPath(exampleLeafIdx);

  // console.log('sr root:', sr);
  // console.log('delta_reg root:', delta_reg);
  // console.log('example sr leaf path:', srPath);
  // console.log('example delta_reg leaf path:', deltaPath);

  await bb.destroy();

  return {
    sr,
    vID,
    alpha_infos,
    votes,
    uIDs,
    betas,
    delta_reg,
    lambdaLeaves,
    uIdLeaves,
    srTree,
    deltaRegTree,
  };
}

(async () => {
  try {
    const innerCircuitNoir = new Noir(innerCircuit as CompiledCircuit);
    const innerBackend = new UltraHonkBackend(innerCircuit.bytecode, { threads: 1 }, { recursive: true });

    const TREE_DEPTH = 7;
    const LEAF_COUNT = 2 ** TREE_DEPTH;
    const input_data = await generateNoirMerkleInputs(TREE_DEPTH);
    
    const proofs = [];
    const public_inputs = [];
    const key_hashes = [];
    const path_elems = [];
    const path_indices = [];

    // Get verification key for inner circuit as fields
    const vk = await innerBackend.getVerificationKey();
    const vkAsFields = deflattenFields(vk);

    for (let ii = 0; ii < LEAF_COUNT; ii++) { 
      const inputs = {
        lambda: frToHex32(input_data.lambdaLeaves[ii]), 
        v: frToHex32(input_data.votes[ii]),
        a: 1000000,
        rlp: 0,
        path_elem: input_data.srTree.getPath(ii).pathElements.map(frToHex32),
        path_index: input_data.srTree.getPath(ii).pathIndices.map(frToHex32),
        reg_path_elem: input_data.deltaRegTree.getPath(ii).pathElements.map(frToHex32), 
        reg_path_index: input_data.deltaRegTree.getPath(ii).pathIndices.map(frToHex32), 
        u_id: frToHex32(input_data.uIDs[ii]),
        alpha_info: frToHex32(input_data.alpha_infos[ii]), 
        beta: frToHex32(input_data.betas[ii]), 
        eta: frToHex32(input_data.uIdLeaves[ii]), 
        v_id: frToHex32(input_data.vID), 
        delta_reg: frToHex32(input_data.delta_reg), 
        sr: frToHex32(input_data.sr)
      }

      const { witness } = await innerCircuitNoir.execute(inputs);
      const { proof, publicInputs: innerPublicInputs } = await innerBackend.generateProof(witness);
      const proofAsFields = deflattenFields(proof);

      // Generate the key hash using the backend method
      const artifacts = await innerBackend.generateRecursiveProofArtifacts(proof, innerPublicInputs.length);
      const vkHash = artifacts.vkHash;

      key_hashes.push(vkHash);
      proofs.push(proofAsFields);
      public_inputs.push(innerPublicInputs);
      path_elems.push(inputs.reg_path_elem);
      path_indices.push(inputs.reg_path_index);

    }

    // Generate proof of the recursive circuit
    const recursiveCircuitNoir = new Noir(recursiveCircuit as CompiledCircuit);
    const recursiveBackend = new UltraHonkBackend(recursiveCircuit.bytecode, { threads: 8 });

    const recursiveInputs = {
          verification_key: vkAsFields,
          proof: proofs,
          public_inputs: public_inputs,
          key_hash: key_hashes,
          eta: input_data.uIdLeaves.map(frToHex32),
          path_elem: path_elems,
          path_index: path_indices,
          v_id: frToHex32(input_data.vID), 
          delta_reg: frToHex32(input_data.delta_reg), 
          delta_usr: frToHex32(input_data.delta_reg), 
          sr: frToHex32(input_data.sr)
        };
    fs.writeFileSync("input.toml", TOML.stringify(recursiveInputs));
    // const { witness: recursiveWitness } = await recursiveCircuitNoir.execute(recursiveInputs);
    // const { proof: recursiveProof, publicInputs: recursivePublicInputs } = await recursiveBackend.generateProof(recursiveWitness);

    // // Verify recursive proof
    // const verified = await recursiveBackend.verifyProof({ proof: recursiveProof, publicInputs: recursivePublicInputs });
    // console.log("Recursive proof verified: ", verified);

    // process.exit(verified ? 0 : 1);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
})();
