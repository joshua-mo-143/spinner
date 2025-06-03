pragma circom 2.0.0;

include "node_modules/circomlib/circuits/poseidon.circom";
include "node_modules/circomlib/circuits/comparators.circom";

template MerkleProof(depth) {
    signal input leaf;
    signal input root;
    signal input pathElements[depth];
    signal input pathIndices[depth];  // 0 or 1 only
    signal output is_valid;

    signal cur[depth + 1];
    cur[0] <== leaf;

    signal leftLeaves[depth];
    signal rightLeaves[depth];

    component hashers[depth];

    for (var i = 0; i < depth; i++) {
        // ensures that pathIndices[i] is either 0 or 1 (anything multiplied by 0 is 0)
        // this works because pathIndices[i] can only ever be 0 or 1
        // due to merkle trees only being able to look at 2 leaves at once
        pathIndices[i] * (pathIndices[i] - 1) === 0;

        // select left and right nodes based on pathIndices[i]
        leftLeaves[i]  <==  cur[i] + pathIndices[i] * (pathElements[i] - cur[i]);
        rightLeaves[i] <== pathElements[i] + pathIndices[i] * (cur[i] - pathElements[i]);

        // instantiate Poseidon hasher component for this layer
        hashers[i] = Poseidon(2);
        hashers[i].inputs[0] <== leftLeaves[i];
        hashers[i].inputs[1] <== rightLeaves[i];

        cur[i + 1] <== hashers[i].out;
    }

    component eq = IsEqual();
    eq.in[0] <== cur[depth];
    eq.in[1] <== root;
    is_valid <== eq.out;


}
