pragma circom 2.2.2;

include "merkle.circom";

template Main() {
    component proof = MerkleProof(3); // For depth-3 tree
    signal input leaf;
    signal input root;
    signal input pathElements[3];
    signal input pathIndices[3];
    signal output is_valid;

    proof.leaf <== leaf;
    proof.root <== root;

    for (var i = 0; i < 3; i++) {
        proof.pathElements[i] <== pathElements[i];
        proof.pathIndices[i] <== pathIndices[i];
    }

    is_valid <== proof.is_valid;
}

component main = Main();
